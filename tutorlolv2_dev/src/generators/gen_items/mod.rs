pub(self) use crate::{
    MayFail,
    generators::{
        Generator, GeneratorExt,
        gen_factories::fac_items::{Capture::*, ItemData},
        gen_utils::RegExtractor,
    },
};
pub(self) use tutorlolv2_gen::{
    ItemId,
    enums::{Attrs::*, DamageType::*},
    eval::CtxVar::*,
};

macro_rules! decl_items {
    (inner $Name:ident) => {
        pub struct $Name {
            pub inner: ItemData
        }

        impl $Name {
            pub fn new(data: ItemData) -> Box<dyn GeneratorExt<ItemData>> {
                Box::new(Self { inner: data })
            }
        }

        impl GeneratorExt<ItemData> for $Name {
            fn end(self: Box<Self>) -> MayFail<ItemData> {
                println!(concat!("[ok] ending generator for ", stringify!($Name)));
                Ok(self.inner)
            }
        }

        impl ::core::ops::Deref for $Name {
            type Target = ItemData;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl ::core::ops::DerefMut for $Name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
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
            fn(ItemData) -> Box<dyn GeneratorExt<ItemData>>
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
    ArdentCenser,
    BladeOfTheRuinedKing,
    LichBane,
    Malignance,
    NashorsTooth,
    LudensEcho,
    HextechRocketbelt,
    GuinsoosRageblade,
    Sheen,
    Redemption,
    KrakenSlayer,
    Rageknife,
    Stridebreaker,
    StatikkShiv,
    DeadMansPlate,
    EssenceReaver,
    WitsEnd,
    HextechAlternator,
    RavenousHydra,
    RunaansHurricane,
    RecurveBow,
    Tiamat,
    IronspikeWhip,
    TitanicHydra,
    Muramana,
    KircheisShard,
    RapidFirecannon,
    SunfireAegis,
    Eclipse,
    Heartsteel,
    Stormrazor3097,
    Bastionbreaker,
    ImperialMandate,
    LiandrysTorment,
    BrambleVest,
    Everfrost,
    Thornmail,
    BamisCinder,
    IcebornGauntlet,
    TrinityForce
);
