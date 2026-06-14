use crate::{
    Build, MayFail, generators::parser::runes::Rune, model::runes::WikiRune,
    scripts::utils::ItemOrRuneExt,
};
use std::{fmt::Write, path::PathBuf};
use tutorlolv2_fmt::to_ssnake;

impl Build for Rune {
    fn build(&mut self, out: PathBuf) -> MayFail<String> {
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
        let mut docs = String::new();

        let upper_id = to_ssnake(&rune_id);

        write!(
            rust,
            "pub static {upper_id}: X = X {{
                name: {name:?},
                metadata: {metadata},
                {fn_names}
                deals_damage: {deals_damage:?},
                riot_id: {riot_id},
                identifiers: {identifiers:?},
                custom: {custom}
            }};",
            identifiers = self.identifiers(),
            deals_damage = self.deals_damage(),
            fn_names = self.function_names(),
            metadata = self.repr_metadata()
        )?;

        write!(
            docs,
            "#[fmt({fmt})]
            static {upper_id}: X = X {{
                name: {name:?}, {damage}
            }};",
            damage = self.repr_damages(),
            fmt = self.formula_fmt()
        )?;

        let (code, doc) = &self.closures()?;
        rust.push_str(code);
        docs.push_str(doc);

        crate::write(out.with_extension("rs"), rust)?;
        crate::write(out.with_extension("w48"), docs)?;
        Ok(self.eval())
    }
}
