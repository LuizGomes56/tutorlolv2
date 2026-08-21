use crate::{
    Build, MayFail, OUT_DIR,
    generators::{parser::runes::Rune, utils::Tag},
    scripts::utils::ItemOrRuneExt,
};
use heck::ToShoutySnakeCase;
use std::fmt::Write;
use tutorlolv2_wiki::runes::WikiRune;

impl Build for Rune {
    fn build(&mut self) -> MayFail<String> {
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
                identifiers: &{identifiers:?},
            }};"#,
            upper_id = rune_id.to_shouty_snake_case(),
            identifiers = self.identifiers(),
            deals_damage = self.deals_damage(),
            fn_names = self.function_names(),
            metadata = self.repr_metadata()
        )?;

        docs.push_str(&self.repr_damages());
        rust.push_str(&self.closures()?);

        let out = OUT_DIR.join(Tag::Runes.plural()).join(rune_id);

        tutorlolv2_wiki::write(out.with_extension("rs"), rust)?;
        tutorlolv2_wiki::write(out.with_extension("w48"), docs)?;

        Ok(self.eval())
    }
}
