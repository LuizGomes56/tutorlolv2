pub use crate::{
    MayFail,
    Progress::*,
    generators::{
        Generator, GeneratorExt,
        gen_factories::wiki_champions::{Ability, Champion},
        gen_utils::RegExtractor,
    },
};
pub use tutorlolv2_types::{
    AbilityId, AbilityId::*, AbilityName::*, Attrs::*, ComboElement::*, CtxVar::*, DamageType::*,
    Key,
};
pub use tutorlolv2_wiki::champions::WikiChampion;

#[macro_export]
macro_rules! decl_champions {
    (inner $Name:ident) => {
        pastey::paste! {
            pub mod [<$Name:lower>];
        }
    };
    ($($Name:ident),*$(,)*) => {
        $(
            $crate::decl_champions!(inner $Name);

            pub struct $Name {
                pub inner: Champion
            }

            impl $Name {
                pub fn new(data: WikiChampion) -> Box<dyn GeneratorExt<Champion>> {
                    Box::new(Self {
                        inner: Champion::new(data)
                    })
                }
            }

            impl GeneratorExt<Champion> for $Name {
                fn end(self: Box<Self>) -> MayFail<Champion> {
                    println!(concat!("[ok] ending generator for ", stringify!($Name)));
                    Ok(self.inner)
                }
            }

            impl ::core::ops::Index<AbilityId> for $Name {
                type Output = Ability;

                fn index(&self, index: AbilityId) -> &Self::Output {
                    self.get(index).unwrap()
                }
            }

            impl ::core::ops::IndexMut<AbilityId> for $Name {
                fn index_mut(&mut self, index: AbilityId) -> &mut Self::Output {
                    self.get_mut(index).unwrap()
                }
            }

            impl ::core::ops::Deref for $Name {
                type Target = Champion;
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

        pub fn champion_gen_fn(champion_id: &str) -> Option<
            fn(WikiChampion) -> Box<dyn GeneratorExt<Champion>>
        > {
            match champion_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}
