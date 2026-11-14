pub use crate::generators::parser::WithMapChanges;
pub use crate::{
    MayFail, formula as f,
    generators::{
        Generator, GeneratorExt,
        constants::WithConstants,
        parser::items::{Item, ItemEvent::*, Source::*},
        utils::RegExtractor,
    },
};
pub use lazy_regex::regex;
pub use tutorlolv2_types::MapMode;
pub use tutorlolv2_types::{AttackType::*, Attrs::*, CtxVar::*, DamageIndex::*, DamageType::*};
pub use tutorlolv2_wiki::items::item_parser::WikiItem;
pub use tutorlolv2_wiki::wikitext::{Formula, IntoFormula};

#[macro_export]
macro_rules! decl_items {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<$Name:snake:lower:replace("c44", "c_44")>];
        }
    };
    ($($Name:ident),*$(,)*) => {
        $(
            $crate::decl_items!(inner $Name);

            #[derive(derive_more::Deref, derive_more::DerefMut)]
            pub struct $Name {
                #[deref]
                #[deref_mut]
                pub inner: Item
            }

            impl $Name {
                pub fn try_from(data: WikiItem) -> MayFail<Box<dyn GeneratorExt<Item>>> {
                    Ok(Box::new(Self {
                        inner: Item::try_from(data)?
                    }))
                }
            }

            impl GeneratorExt<Item> for $Name {
                fn end(self: Box<Self>) -> MayFail<Item> {
                    Ok(self.inner)
                }
            }
        )*

        pub fn item_gen_fn(item_id: &str) -> Option<
            fn(WikiItem) -> MayFail<Box<dyn GeneratorExt<Item>>>
        > {
            match item_id {
                $(stringify!($Name) => Some($Name::try_from),)*
                _ => None,
            }
        }
    };
}
