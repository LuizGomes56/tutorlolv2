use crate::{
    Build, BuildContext, BuildOutput, MayFail,
    generators::{parser::runes::Rune, utils::Tag},
    scripts::utils::{ItemOrRuneExt, bake},
};
use heck::ToShoutySnakeCase;
use std::fmt::Write;
use tutorlolv2_wiki::runes::WikiRune;

impl Build for Rune {
    fn build(&mut self, context: &BuildContext) -> MayFail<BuildOutput> {
        let Self {
            riot_id,
            data:
                WikiRune {
                    name,
                    rune_id,
                    custom,
                    ..
                },
            ..
        } = &self;

        let mut rust = String::new();
        let plan = self.prepare_formulas();

        write!(
            rust,
            r#"pub static {upper_id}: X = X {{
                name: {name:?},
                metadata: {metadata},
                {fn_names}
                deals_damage: {deals_damage:?},
                riot_id: {riot_id},
                custom: {custom},

                #[cfg(feature = "yew")]
                identifiers: {identifiers},
            }};"#,
            upper_id = rune_id.to_shouty_snake_case(),
            identifiers = bake(&self.identifiers().as_slice()),
            deals_damage = self.deals_damage(),
            fn_names = self.function_names(),
            metadata = self.repr_metadata()
        )?;

        rust.push_str(&self.closures(&plan)?);

        let out = context.out.join(Tag::Runes.as_ref()).join(rune_id);

        tutorlolv2_wiki::write(out.with_extension("rs"), rust)?;

        Ok(BuildOutput {
            eval: self.eval(),
            formulas: if context.docs {
                self.formula_docs(&plan)
            } else {
                Vec::new()
            },
        })
    }
}
