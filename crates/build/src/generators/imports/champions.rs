//! What champion generator files import, and [`decl_champions`], which declares the
//! the supported champions

pub use crate::{
    MayFail, formula as f,
    generators::champions::{ChampionGenerator, Damage},
    generators::{
        Generator, GeneratorExt, constants::WithConstants, parser::champions::Champion,
        utils::RegExtractor,
    },
};
pub use lazy_regex::regex;
pub use types::{
    AbilityId, AbilityId::*, AbilityName::*, Attrs::*, ComboElement::*, CtxVar::*, DamageType::*,
    Key, MapMode,
};
pub use wiki::{
    champions::WikiChampion,
    wikitext::{Formula, IntoFormula},
};

/// Declares the supported champions: for each one, its module, a
/// struct that wraps a [`ChampionGenerator`] and dereferences to it, and the shared generator traits the
/// build calls. The registry covers every supported champion.
#[macro_export]
macro_rules! decl_champions {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<$Name:snake:lower:replace("jarvan_i_v", "jarvan_iv")>];
        }
    };
    ($($Name:ident),*$(,)?) => {
        $(
            $crate::decl_champions!(inner $Name);

            #[derive(derive_more::Deref, derive_more::DerefMut)]
            pub struct $Name {
                #[deref]
                #[deref_mut]
                pub inner: ChampionGenerator,
            }

            impl $Name {
                pub fn try_from(data: WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>> {
                    let model = ChampionGenerator::models()?
                        .get(&data.champion_id)
                        .cloned()
                        .ok_or_else(|| format!("[{}] has no model in model.json", data.champion_id))?;

                    Ok(Box::new(Self {
                        inner: ChampionGenerator::new(Champion::try_from(data)?, model),
                    }))
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
