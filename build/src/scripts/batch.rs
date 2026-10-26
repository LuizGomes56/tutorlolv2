//! Typed documentation transport. Metadata never travels inside Rust attributes.
use crate::{BuildContext, MayFail, context::run_formatter, generators::utils::Tag};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::ErrorKind, path::Path, process::Command};
use syn::spanned::Spanned;
use tutorlolv2_codec::{FormulaDbBuilder, FormulaSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaDoc {
    pub local: u8,
    pub function: String,
    pub source: String,
    pub name: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityDocs {
    pub tag: Tag,
    pub index: u16,
    pub variant: String,
    pub formulas: Vec<FormulaDoc>,
    pub generator: Option<String>,
}

/// Preserve the original source, including comments, instead of rebuilding tokens.
pub fn generator_impl(source: &str) -> MayFail<String> {
    let file = syn::parse_file(source)?;

    let item = file
        .items
        .iter()
        .find(|item| {
            matches!(item,
                syn::Item::Impl(item) if item.trait_.as_ref().is_some_and(|(path, _)|
                    path.segments.last().is_some_and(|s| s.ident == "Generator"))
            )
        })
        .ok_or("Generator source has no impl Generator block")?;

    Ok(source[item.span().byte_range()].to_owned())
}

pub fn read_generator(root: &Path, tag: Tag, variant: &str) -> MayFail<Option<String>> {
    use heck::ToSnakeCase;

    let path = root
        .join("build/src/generators/impls")
        .join(tag.as_ref())
        .join(variant.to_snake_case())
        .with_extension("rs");

    match std::fs::read_to_string(&path) {
        Ok(source) => generator_impl(&source)
            .map(Some)
            .map_err(|e| format!("Invalid generator documentation {path:?}: {e}").into()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("Unable to read generator documentation {path:?}: {e}").into()),
    }
}

impl BuildContext {
    /// Format all formulas in one process. Only text is recovered from Rust syntax;
    /// IDs, references, names, and comments remain in their typed records throughout.
    pub fn format_formulas(&self, docs: &mut [EntityDocs]) -> MayFail {
        let mut source = String::new();

        let count = docs
            .iter()
            .flat_map(|doc| &doc.formulas)
            .enumerate()
            .map(|(i, formula)| {
                source.push_str(&format!("fn __formula_{i}() {{\n{}\n}}\n", formula.source));
            })
            .count();

        if count == 0 {
            return Ok(());
        }

        let path = self.out.join("formula_sources.rs");

        tutorlolv2_wiki::write(&path, source)?;

        run_formatter(
            Command::new("rustfmt")
                .args(["--edition", "2024", "--config", "max_width=48"])
                .arg(&path),
        )?;

        let formatted = tutorlolv2_wiki::read_to_string(&path)?;
        let file = syn::parse_file(&formatted)?;

        if file.items.len() != count {
            return Err("Formatted formula count changed".into());
        }

        for (i, (item, formula)) in file
            .items
            .iter()
            .zip(docs.iter_mut().flat_map(|doc| &mut doc.formulas))
            .enumerate()
        {
            let syn::Item::Fn(item) = item else {
                return Err("Expected a formatted formula function".into());
            };

            if item.sig.ident != format!("__formula_{i}") {
                return Err(
                    format!("Unexpected formatted formula name: {}", item.sig.ident).into(),
                );
            }

            let start = item.block.brace_token.span.open().byte_range().end;
            let end = item.block.brace_token.span.close().byte_range().start;

            formula.source = formatted[start..end].trim().to_owned();
        }

        Ok(())
    }
}

impl EntityDocs {
    pub fn pack_formulas(&self, packer: &mut FormulaDbBuilder) -> MayFail {
        if self.formulas.is_empty() {
            return Ok(());
        }

        let formulas = self
            .formulas
            .iter()
            .map(|formula| FormulaSource {
                local: formula.local,
                source: &formula.source,
            })
            .collect::<Vec<_>>();

        let refs = self
            .formulas
            .iter()
            .map(|formula| (formula.function.clone(), formula.local))
            .collect::<HashMap<_, _>>();

        match self.tag {
            Tag::Champions => packer.push_champion(self.index, &formulas, &refs),
            _ => packer.push_sparse(self.tag.entity_kind(), self.index, &formulas, &refs),
        }
        .map_err(|e| {
            format!(
                "Packing {} {} formulas at index {}: {e}",
                self.tag.as_ref(),
                self.variant,
                self.index
            )
            .into()
        })
    }
}
