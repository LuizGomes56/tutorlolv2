pub use crate::{
    MayFail, formula as f,
    generators::{
        Generator, GeneratorExt,
        parser::champions::{Ability, Champion},
        utils::RegExtractor,
    },
    model::{LevelArm, Scaling, champions::WikiChampion},
};
pub use tutorlolv2_types::{
    AbilityId, AbilityId::*, AbilityName::*, Attrs::*, ComboElement::*, CtxVar::*, DamageType::*,
    Key,
};

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
                pub fn new(data: WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>> {
                    Ok(Box::new(Self {
                        inner: Champion::try_from(data)?
                    }))
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
            fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>
        > {
            match champion_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}
