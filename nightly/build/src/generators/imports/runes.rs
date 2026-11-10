pub use crate::{
    MayFail, formula as f,
    generators::{Generator, GeneratorExt, parser::runes::Rune, utils::RegExtractor},
};
pub use tutorlolv2_types::{AttackType::*, CtxVar::*, DamageIndex::*, DamageType::*};
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

            #[derive(derive_more::Deref, derive_more::DerefMut)]
            pub struct $Name {
                #[deref]
                #[deref_mut]
                pub inner: Rune
            }

            impl $Name {
                pub fn try_from(data: WikiRune) -> MayFail<Box<dyn GeneratorExt<Rune>>> {
                    Ok(Box::new(Self {
                        inner: Rune::try_from(data)?
                    }))
                }
            }

            impl GeneratorExt<Rune> for $Name {
                fn end(self: Box<Self>) -> MayFail<Rune> {
                    Ok(self.inner)
                }
            }
        )*

        pub fn rune_gen_fn(rune_id: &str) -> Option<
            fn(WikiRune) -> MayFail<Box<dyn GeneratorExt<Rune>>>
        > {
            match rune_id {
                $(stringify!($Name) => Some($Name::try_from),)*
                _ => None,
            }
        }
    };
}
