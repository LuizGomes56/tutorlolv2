pub(self) use crate::{
    MayFail,
    generators::{Generator, gen_factories::fac_items::ItemData, gen_utils::RegExtractor},
};
pub(self) use tutorlolv2_gen::{
    ItemId,
    enums::{Attrs::*, DamageType::*},
    eval::CtxVar::*,
};

macro_rules! decl_items {
    (inner $Name:ident) => {
        pub struct $Name(pub ItemData);

        impl $Name {
            pub fn new(data: ItemData) -> Box<dyn Generator<ItemData>> {
                Box::new(Self(data))
            }

            pub fn end(self) -> MayFail<ItemData> {
                Ok(self.0)
            }

            pub fn into_inner(self) -> ItemData {
                self.0
            }
        }

        impl ::core::ops::Deref for $Name {
            type Target = ItemData;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $Name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($($Name:ident),*$(,)*) => {
        $(
            pastey::paste! {
                pub mod [<$Name:snake>];
            }

            decl_items!(inner $Name);
        )*

        pub fn item_gen_fn(item_id: &str) -> Option<
            fn(ItemData) -> Box<dyn Generator<ItemData>>
        > {
            $(const _: ItemId = ItemId::$Name;)*
            match item_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }
    };
}

decl_items!(
    BladeOfTheRuinedKing,
    LichBane,
    Malignance,
    NashorsTooth,
    LudensEcho,
    HextechRocketbelt,
    GuinsoosRageblade,
    Sheen,
    RavenousHydra,
    RunaansHurricane,
    RecurveBow,
    Tiamat,
    IronspikeWhip,
);
