pub(self) use crate::{
    MayFail,
    Progress::*,
    generators::{
        Generator, GeneratorExt,
        gen_factories::wiki_champions::{Ability, Champion},
        gen_utils::RegExtractor,
    },
};
use std::ops::{Index, IndexMut};
pub(self) use tutorlolv2_gen::CtxVar::*;
pub(self) use tutorlolv2_types::{
    AbilityId, AbilityId::*, AbilityName::*, Attrs::*, ComboElement::*, DamageType::*, Key,
};
pub(self) use tutorlolv2_wiki::champions::WikiChampion;

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

            impl Index<AbilityId> for $Name {
                type Output = Ability;

                fn index(&self, index: AbilityId) -> &Self::Output {
                    self.get(index).unwrap()
                }
            }

            impl IndexMut<AbilityId> for $Name {
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

decl_mod!(
    Aatrox,
    Ahri,
    Akali,
    Akshan,
    Alistar,
    Ambessa,
    Amumu,
    Anivia,
    Annie,
    Aphelios,
    Ashe,
    AurelionSol,
    Aurora,
    Azir,
    Bard,
    Belveth,
    Blitzcrank,
    Brand,
    Braum,
    Briar,
    Caitlyn,
    Camille,
    Cassiopeia,
    Chogath,
    Corki,
    Darius,
    Diana,
    Draven,
    DrMundo,
    Ekko,
    Elise,
    Evelynn,
    Ezreal,
    Fiddlesticks,
    Fiora,
    Fizz,
    Galio,
    Gangplank,
    Garen,
    Gnar,
    Gragas,
    Graves,
    Gwen,
    Hecarim,
    Heimerdinger,
    Hwei,
    Illaoi,
    Irelia,
    Ivern,
    Janna,
    JarvanIV,
    Jax,
    Jayce,
    Jhin,
    Jinx,
    Kaisa,
    Kalista,
    Karma,
    Karthus,
    Kassadin,
    Katarina,
    Kayle,
    Kayn,
    Kennen,
    Khazix,
    Kindred,
    Kled,
    KogMaw,
    KSante,
    Leblanc,
    LeeSin,
    Leona,
    Lillia,
    Lissandra,
    Lucian,
    Lulu,
    Lux,
    Malphite,
    Malzahar,
    Maokai,
    MasterYi,
    Mel,
    Milio,
    MissFortune,
    MonkeyKing,
    Mordekaiser,
    Morgana,
    Naafiri,
    Nami,
    Nasus,
    Nautilus,
    Neeko,
    Nidalee,
    Nilah,
    Nocturne,
    Nunu,
    Olaf,
    Orianna,
    Ornn,
    Pantheon,
    Poppy,
    Pyke,
    Qiyana,
    Quinn,
    Rakan,
    Rammus,
    RekSai,
    Rell,
    Renata,
    Renekton,
    Rengar,
    Riven,
    Rumble,
    Ryze,
    Samira,
    Sejuani,
    Senna,
    Seraphine,
    Sett,
    Shaco,
    Shen,
    Shyvana,
    Singed,
    Sion,
    Sivir,
    Skarner,
    Smolder,
    Sona,
    Soraka,
    Swain,
    Sylas,
    Syndra,
    TahmKench,
    Taliyah,
    Talon,
    Taric,
    Teemo,
    Thresh,
    Tristana,
    Trundle,
    Tryndamere,
    TwistedFate,
    Twitch,
    Udyr,
    Urgot,
    Varus,
    Vayne,
    Veigar,
    Velkoz,
    Vex,
    Vi,
    Viego,
    Viktor,
    Vladimir,
    Volibear,
    Warwick,
    Xayah,
    Xerath,
    XinZhao,
    Yasuo,
    Yone,
    Yorick,
    Yunara,
    Yuumi,
    Zaahen,
    Zac,
    Zed,
    Zeri,
    Ziggs,
    Zilean,
    Zoe,
    Zyra,
);
