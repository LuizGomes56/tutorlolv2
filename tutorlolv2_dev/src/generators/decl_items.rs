pub use crate::{
    MayFail,
    Progress::*,
    generators::{
        Generator, GeneratorExt,
        gen_factories::wiki_items::{Item, Source::*},
        gen_utils::RegExtractor,
    },
};
pub use tutorlolv2_types::{Attrs::*, CtxVar::*, DamageType::*};
pub use tutorlolv2_wiki::items::item_parser::WikiItem;

#[macro_export]
macro_rules! decl_items {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<
                $Name:snake
                    :lower
                    :replace("b_f", "bf")
                    :replace("c44", "c_44")
            >];
        }
    };
    ($($Name:ident),*$(,)*) => {
        $(
            $crate::decl_items!(inner $Name);

            pub struct $Name {
                pub inner: Item
            }

            impl $Name {
                pub fn new(data: WikiItem) -> Box<dyn GeneratorExt<Item>> {
                    Box::new(Self {
                        inner: Item::new(data)
                    })
                }
            }

            impl GeneratorExt<Item> for $Name {
                fn end(self: Box<Self>) -> MayFail<Item> {
                    println!(concat!("[ok] ending generator for ", stringify!($Name)));
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
            fn(WikiItem) -> Box<dyn GeneratorExt<Item>>
        > {
            match item_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}
