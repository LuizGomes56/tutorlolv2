//! What v2 generator files import, and [`decl_champions_v2`], which declares the
//! champions that have a v2 generator

pub use crate::{
    MayFail, formula_v2 as f,
    generators::{
        Generator, GeneratorExt, constants::WithConstants, parser::champions::Champion,
        utils::RegExtractor,
    },
    generators_v2::{ChampionV2, Damage, GeneratorV2},
};
pub use lazy_regex::regex;
pub use tutorlolv2_types::{
    AbilityId, AbilityId::*, AbilityName::*, Attrs::*, ComboElement::*, CtxVar::*, DamageType::*,
    Key, MapMode,
};
pub use tutorlolv2_wiki::{
    champions::WikiChampion,
    wikitext::{Formula, IntoFormula},
};

/// Declares the champions that have a v2 generator: for each one, its module, a
/// struct that wraps a [`ChampionV2`] and dereferences to it, and the shared generator traits the
/// build calls. The registry covers every supported champion.
#[macro_export]
macro_rules! decl_champions_v2 {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<$Name:snake:lower:replace("jarvan_i_v", "jarvan_iv")>];
        }
    };
    ($($Name:ident),*$(,)?) => {
        $(
            $crate::decl_champions_v2!(inner $Name);

            #[derive(derive_more::Deref, derive_more::DerefMut)]
            pub struct $Name {
                #[deref]
                #[deref_mut]
                pub inner: ChampionV2,
            }

            impl $Name {
                pub fn try_from(data: WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>> {
                    let model = ChampionV2::models()?
                        .get(&data.champion_id)
                        .cloned()
                        .ok_or_else(|| format!("[{}] has no model in model.json", data.champion_id))?;

                    Ok(Box::new(Self {
                        inner: ChampionV2::new(Champion::try_from(data)?, model),
                    }))
                }
            }

            impl Generator for $Name {
                fn generate(&mut self) -> MayFail {
                    GeneratorV2::generate(self)
                }
            }

            impl GeneratorExt<Champion> for $Name {
                fn end(self: Box<Self>) -> MayFail<Champion> {
                    self.inner.into_champion()
                }
            }
        )*

        pub fn champion_gen_fn(champion_id: &str) -> Option<
            fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>
        > {
            match champion_id {
                $(stringify!($Name) => Some($Name::try_from),)*
                _ => None,
            }
        }

        /// Supported champions
        pub const CHAMPIONS: &[&str] = &[$(stringify!($Name)),*];
    };
}
