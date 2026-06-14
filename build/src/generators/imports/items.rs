pub use crate::{
    MayFail,
    generators::{
        Generator, GeneratorExt,
        parser::items::{Item, Source::*},
        utils::RegExtractor,
    },
    model::items::WikiItem,
};
pub use core::fmt::Display;
pub use tutorlolv2_types::{AttackType::*, Attrs::*, CtxVar::*, DamageIndex::*, DamageType::*};

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

            pub struct $Name {
                pub inner: Item
            }

            impl $Name {
                pub fn new(data: WikiItem) -> MayFail<Box<dyn GeneratorExt<Item>>> {
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

            impl ::core::ops::Deref for $Name {
                type Target = Item;
                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl ::core::ops::DerefMut for $Name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inner
                }
            }
        )*

        pub fn item_gen_fn(item_id: &str) -> Option<
            fn(WikiItem) -> MayFail<Box<dyn GeneratorExt<Item>>>
        > {
            match item_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}
