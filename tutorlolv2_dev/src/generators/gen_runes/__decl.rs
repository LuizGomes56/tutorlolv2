pub use crate::{
    MayFail,
    Progress::*,
    generators::{
        Generator, GeneratorExt, gen_factories::wiki_runes::Rune, gen_utils::RegExtractor,
    },
};
pub use tutorlolv2_gen::CtxVar::*;
pub use tutorlolv2_types::{Attrs::*, DamageType::*};
pub use tutorlolv2_wiki::runes::WikiRune;

#[macro_export]
macro_rules! decl_runes {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<$Name:snake>];
        }
    };
    ($($Name:ident),*$(,)*) => {
        $(
            $crate::decl_runes!(inner $Name);

            pub struct $Name {
                pub inner: Rune
            }

            impl $Name {
                pub fn new(data: WikiRune) -> Box<dyn GeneratorExt<Rune>> {
                    Box::new(Self {
                        inner: Rune::new(data)
                    })
                }
            }

            impl GeneratorExt<Rune> for $Name {
                fn end(self: Box<Self>) -> MayFail<Rune> {
                    println!(concat!("[ok] ending generator for ", stringify!($Name)));
                    Ok(self.inner)
                }
            }

            impl ::core::ops::Deref for $Name {
                type Target = Rune;
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

        pub fn rune_gen_fn(rune_id: &str) -> Option<
            fn(WikiRune) -> Box<dyn GeneratorExt<Rune>>
        > {
            match rune_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}
