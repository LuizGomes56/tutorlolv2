pub(self) use crate::{
    MayFail,
    Progress::*,
    champions::{Ability, Champion, MerakiChampion},
    generators::{Generator, gen_factories::fac_champions::ChampionData, gen_utils::RegExtractor},
};
pub(self) use tutorlolv2_gen::{
    AbilityId,
    AbilityId::*,
    AbilityName::*,
    ComboElement::*,
    Key,
    enums::{Attrs::*, DamageType::*},
    eval::CtxVar::*,
};

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
                pub inner: ChampionData
            }

            impl $Name {
                pub const fn name() -> &'static str {
                    stringify!($Name)
                }

                pub fn new(data: MerakiChampion) -> Box<dyn Generator<Champion>> {
                    Box::new(Self {
                        inner: ChampionData::new(data)
                    })
                }

                pub fn end(self) -> MayFail<Champion> {
                    println!("Ending generator for {}", Self::name());
                    self.inner.end()
                }
            }

            impl ::core::ops::Deref for $Name {
                type Target = ChampionData;
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
            fn(MerakiChampion) -> Box<dyn Generator<Champion>>
        > {
            match champion_id {
                $(stringify!($Name) => Some($Name::new),)*
                _ => None,
            }
        }

        pub const fn champion_gen_names() -> &'static [&'static str] {
            &[$(stringify!($Name),)*]
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
