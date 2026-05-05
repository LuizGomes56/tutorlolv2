pub(self) use crate::{
    MayFail,
    Progress::*,
    generators::{
        Generator, GeneratorExt, gen_factories::wiki_items::Item, gen_utils::RegExtractor,
    },
};
use std::ops::{Index, IndexMut};
pub(self) use tutorlolv2_gen::CtxVar::*;
pub(self) use tutorlolv2_types::{Attrs::*, DamageType::*};
pub(self) use tutorlolv2_wiki::items::item_parser::WikiItem;

macro_rules! decl_mod {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<$Name:lower>];
        }
    };
    ($($Name:ident),*$(,)*) => {
        $(
            decl_mod!(inner $Name);

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

        pub fn Item_gen_fn(item_id: &str) -> Option<
            fn(WikiItem) -> Box<dyn GeneratorExt<Item>>
        > {
            match item_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}
