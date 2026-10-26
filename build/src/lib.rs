use crate::{
    context::run_formatter,
    generators::parser::{
        MapValueExt, Parser, champions::ChampionParser, items::ItemParser, runes::RuneParser,
    },
    scripts::{
        batch::{EntityDocs, FormulaDoc, read_generator},
        finish::{eval_abilities, eval_items_or_runes},
    },
};
use heck::ToSnakeCase;
use rayon::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, fmt::Write, process::Command, str::FromStr};
use strum::VariantNames;
use tutorlolv2_codec::{Class, ClassOverride, FormulaDbBuilder, GeneratorDbBuilder};
use tutorlolv2_types::{AbilityName, AttackType, CtxVar};

pub use context::{BuildContext, project_root};
pub use tutorlolv2_wiki::{DynError, MayFail};

pub mod context;
pub mod generators;
pub mod scripts;

#[cfg(test)]
mod tests;

pub const ENGINE_CFG: &str = r#"#[cfg(feature = "engine")]"#;
pub const YEW_CFG: &str = r#"#[cfg(feature = "yew")]"#;
pub const FORMULA_DOCS_ENV: &str = "CARGO_FEATURE_FORMULA_DOCS";

pub struct BuildOutput {
    pub eval: String,
    pub formulas: Vec<FormulaDoc>,
}

pub trait Build {
    fn build(&mut self, context: &BuildContext) -> MayFail<BuildOutput>;
}

pub fn run() -> MayFail {
    let ctx = BuildContext::from_env()?;

    ctx.run()
}

impl BuildContext {
    fn build_code<T, U>(
        &self,
        parser: &impl Parser<T, U>,
        aliases: Option<BTreeMap<String, Vec<String>>>,
        eval_fn: impl FnOnce(&str) -> String,
    ) -> MayFail<Vec<EntityDocs>>
    where
        T: Clone + DeserializeOwned + MapValueExt + Send + Sync + 'static,
        U: Build + Serialize + TryFrom<T, Error = DynError>,
    {
        let tag = parser.tag();
        let keys = parser.keys().collect::<Vec<_>>();

        // Indexed parallel collection preserves the enum/codec order.
        let builds = keys
            .par_iter()
            .map(|id| -> MayFail<_> {
                let mut entity = parser
                    .run_fn(id)
                    .map_err(|e| format!("Generating {} {id}: {e}", tag.as_ref()))?;

                let export = if self.export_json {
                    Some(parser.export(id, &entity, &self.root)?)
                } else {
                    None
                };

                let output = entity
                    .build(self)
                    .map_err(|e| format!("Building {} {id}: {e}", tag.as_ref()))?;

                Ok((output, export))
            })
            .collect::<MayFail<Vec<_>>>()?;

        let mut eval = String::new();
        let mut docs = Vec::new();
        let mut exports = BTreeMap::new();

        for (index, (id, (output, export))) in keys.iter().zip(builds).enumerate() {
            eval.push_str(&output.eval);

            if self.docs {
                docs.push(EntityDocs {
                    tag,
                    index: index.try_into()?,
                    variant: (*id).into(),
                    formulas: output.formulas,
                    generator: read_generator(&self.root, tag, id)?,
                });
            }

            if let Some(export) = export {
                exports.insert(*id, export);
            }
        }

        if self.export_json {
            use tutorlolv2_wiki::JsonWrite;

            exports.write_file(
                self.root
                    .join("internal")
                    .join(tag.as_ref())
                    .with_extension("json"),
            )?;
        }

        tutorlolv2_wiki::write(
            self.out.join(format!("{}_code.rs", tag.as_ref())),
            [
                parser.id_enum(),
                parser.phf(aliases),
                parser.data_variable(),
                eval_fn(&eval),
            ]
            .concat(),
        )?;

        Ok(docs)
    }

    fn write_module<T, U>(&self, command: &mut Command, parser: &impl Parser<T, U>) -> MayFail
    where
        T: Clone + DeserializeOwned + MapValueExt + Send + Sync + 'static,
        U: TryFrom<T, Error = DynError> + Serialize,
    {
        let mut result = String::new();
        let dir = parser.tag().to_string();

        command.arg(self.out.join(format!("{dir}_code.rs")));

        for id in parser.keys() {
            let module = id.to_snake_case();
            let model_ty = parser.tag().enum_name().trim_end_matches("Id");

            command.arg(self.out.join(&dir).join(id).with_extension("rs"));

            writeln!(
                result,
                r#"pub mod {module} {{
                    {model_imports}
                    type X = {model_ty};
                    include!(concat!(env!("OUT_DIR"), "/{dir}/{id}.rs"));
                }}"#,
                model_imports = format_args!(
                    "use crate::libgen::globals::{{{model_ty}, {}}};",
                    if matches!(parser.tag(), generators::utils::Tag::Champions) {
                        "WikiStats, WikiModifiers, Stat, Modifier"
                    } else {
                        ""
                    }
                )
            )?;
        }

        let path = self.out.join(&dir).with_extension("rs");

        tutorlolv2_wiki::write(&path, result)?;

        command.arg(path);

        Ok(())
    }

    pub fn run(&self) -> MayFail {
        for input in self.inputs() {
            println!("cargo:rerun-if-changed={}", input.display());
        }

        println!("cargo:rerun-if-env-changed={FORMULA_DOCS_ENV}");

        let (champions, (items, runes)) = rayon::join(
            || ChampionParser::from_root(&self.root),
            || {
                rayon::join(
                    || ItemParser::from_root(&self.root),
                    || RuneParser::from_root(&self.root),
                )
            },
        );

        let (champions, items, runes) = (champions?, items?, runes?);

        if self.export_json {
            champions.progress();
            items.progress();
            runes.progress();
        }

        let aliases = champions.aliases(&self.root)?;
        let recommendations = champions.recommendations(&self.root)?;

        let (c, (i, r)) = rayon::join(
            || {
                self.build_code(&champions, aliases, |arms| {
                    eval_abilities(arms, &recommendations)
                })
            },
            || {
                rayon::join(
                    || self.build_code(&items, None, |arms| eval_items_or_runes(arms, items.tag())),
                    || self.build_code(&runes, None, |arms| eval_items_or_runes(arms, runes.tag())),
                )
            },
        );

        let mut docs = [c?, i?, r?].into_iter().flatten().collect::<Vec<_>>();

        if self.docs {
            let [champion_count, item_count, rune_count] =
                [champions.len(), items.len(), runes.len()].map(u16::try_from);

            self.build_docs(&mut docs, [champion_count?, item_count?, rune_count?])?;
        }

        let mut command = Command::new("rustfmt");
        command.args(["--edition", "2024", "--config", "max_width=80"]);

        self.write_module(&mut command, &champions)?;
        self.write_module(&mut command, &items)?;
        self.write_module(&mut command, &runes)?;

        run_formatter(&mut command)
    }

    fn build_docs(&self, docs: &mut [EntityDocs], counts: [u16; 3]) -> MayFail {
        self.format_formulas(docs)?;

        let overrides = [AttackType::VARIANTS, AbilityName::VARIANTS]
            .into_iter()
            .flatten()
            .map(|&text| ClassOverride::new(text, Class::Constant))
            .collect::<Vec<_>>();

        let mut generators = GeneratorDbBuilder::new(counts[0], counts[1], counts[2], &overrides);

        let mut formulas = FormulaDbBuilder::new(counts[0], counts[1], counts[2], |s| {
            CtxVar::from_str(s).ok().map(|v| v as u8)
        });

        for doc in docs.iter() {
            doc.pack_formulas(&mut formulas)?;

            if let Some(source) = &doc.generator {
                generators
                    .push(doc.tag.entity_kind(), doc.index, source)
                    .map_err(|e| {
                        format!(
                            "Packing {} generator {}: {e}",
                            doc.tag.as_ref(),
                            doc.variant
                        )
                    })?;
            }
        }

        tutorlolv2_wiki::write(self.out.join("formulas.bin"), formulas.finish()?)?;
        tutorlolv2_wiki::write(
            self.out.join("generator.bin"),
            generators.finish_with_stats()?.bytes,
        )?;

        // A lossless, inspectable sidecar for tooling; never read back as a build input.
        tutorlolv2_wiki::write(
            self.out.join("documentation.json"),
            serde_json::to_vec_pretty(docs)?,
        )
    }
}
