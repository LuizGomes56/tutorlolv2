use serde::{Deserialize, Serialize};

pub struct EvalContext {
    pub chogath_stacks: f64,
    pub veigar_stacks: f64,
    pub nasus_stacks: f64,
    pub smolder_stacks: f64,
    pub aurelion_sol_stacks: f64,
    pub thresh_stacks: f64,
    pub kindred_stacks: f64,
    pub belveth_stacks: f64,
    pub adaptative_damage: f64,
    pub level: f64,
    pub physical_multiplier: f64,
    pub magic_multiplier: f64,
    pub steelcaps_effect: f64,
    pub randuin_effect: f64,
    pub rocksolid_effect: f64,
    pub enemy_bonus_health: f64,
    pub enemy_armor: f64,
    pub enemy_max_health: f64,
    pub enemy_health: f64,
    pub enemy_current_health: f64,
    pub enemy_missing_health: f64,
    pub enemy_magic_resist: f64,
    pub base_health: f64,
    pub base_ad: f64,
    pub base_armor: f64,
    pub base_magic_resist: f64,
    pub base_mana: f64,
    pub bonus_ad: f64,
    pub bonus_armor: f64,
    pub bonus_magic_resist: f64,
    pub bonus_health: f64,
    pub bonus_mana: f64,
    pub bonus_move_speed: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub max_mana: f64,
    pub current_mana: f64,
    pub max_health: f64,
    pub current_health: f64,
    pub armor: f64,
    pub magic_resist: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub attack_speed: f64,
    pub missing_health: f64,
    pub ap: f64,
    pub ad: f64,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    #[default]
    Unknown,
}

impl ToString for DamageType {
    fn to_string(&self) -> String {
        let res = match self {
            DamageType::Physical => "PHYSICAL_DAMAGE",
            DamageType::Magic => "MAGIC_DAMAGE",
            DamageType::Mixed => "MIXED_DAMAGE",
            DamageType::True => "TRUE_DAMAGE",
            DamageType::Adaptative => "ADAPTATIVE_DAMAGE",
            DamageType::Unknown => "UNKNOWN_DAMAGE",
        };
        res.to_string()
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Attrs {
    None,
    Onhit,
    OnhitMin,
    OnhitMax,
}

#[derive(Copy, Clone)]
pub enum AttackType {
    Melee,
    Ranged,
}

#[derive(Copy, Clone)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

#[derive(Copy, Clone)]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

#[derive(Copy, Clone, Serialize)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
    A,
    C,
    O,
}

macro_rules! impl_key {
    ($field:ident) => {
        paste::paste! {
            impl AbilityLike {
                pub fn [<from_str_ $field>](s: &str) -> &'static str {
                    match s {
                        concat!(stringify!([<$field:upper>]), "1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1)"),
                        concat!(stringify!([<$field:upper>]), "2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2)"),
                        concat!(stringify!([<$field:upper>]), "3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3)"),
                        concat!(stringify!([<$field:upper>]), "4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4)"),
                        concat!(stringify!([<$field:upper>]), "5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5)"),
                        concat!(stringify!([<$field:upper>]), "6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6)"),
                        concat!(stringify!([<$field:upper>]), "7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7)"),
                        concat!(stringify!([<$field:upper>]), "8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8)"),
                        concat!(stringify!([<$field:upper>]), "MEGA") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Mega)"),
                        concat!(stringify!([<$field:upper>]), "MAX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Max)"),
                        concat!(stringify!([<$field:upper>]), "MIN") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Min)"),
                        concat!(stringify!([<$field:upper>]), "MNX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion)"),
                        concat!(stringify!([<$field:upper>]), "MN1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion1)"),
                        concat!(stringify!([<$field:upper>]), "MN2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion2)"),
                        concat!(stringify!([<$field:upper>]), "MN3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion3)"),
                        concat!(stringify!([<$field:upper>]), "MMNX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::MinionMax)"),
                        concat!(stringify!([<$field:upper>]), "MSTR") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster)"),
                        concat!(stringify!([<$field:upper>]), "MST1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster1)"),
                        concat!(stringify!([<$field:upper>]), "MST2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster2)"),
                        concat!(stringify!([<$field:upper>]), "MST3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster3)"),
                        concat!(stringify!([<$field:upper>]), "MST4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster4)"),
                        concat!(stringify!([<$field:upper>]), "MMST") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::MonsterMax)"),
                        stringify!([<$field:upper>]) => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Void)"),
                        concat!(stringify!([<$field:upper>]), "MAX1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8Max)"),
                        concat!(stringify!([<$field:upper>]), "MIN1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8Min)"),
                        _ => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Void)")
                    }
                }
                pub fn [<to_str_ $field>](&self) -> &'static str {
                    match self {
                        AbilityLike::[<$field:upper>](v) => {
                            match v {
                                AbilityName::_1 => concat!(stringify!([<$field:upper>]), "1"),
                                AbilityName::_2 => concat!(stringify!([<$field:upper>]), "2"),
                                AbilityName::_3 => concat!(stringify!([<$field:upper>]), "3"),
                                AbilityName::_4 => concat!(stringify!([<$field:upper>]), "4"),
                                AbilityName::_5 => concat!(stringify!([<$field:upper>]), "5"),
                                AbilityName::_6 => concat!(stringify!([<$field:upper>]), "6"),
                                AbilityName::_7 => concat!(stringify!([<$field:upper>]), "7"),
                                AbilityName::_8 => concat!(stringify!([<$field:upper>]), "8"),
                                AbilityName::Mega => concat!(stringify!([<$field:upper>]), "MEGA"),
                                AbilityName::Max => concat!(stringify!([<$field:upper>]), "MAX"),
                                AbilityName::Min => concat!(stringify!([<$field:upper>]), "MIN"),
                                AbilityName::Minion => concat!(stringify!([<$field:upper>]), "MNX"),
                                AbilityName::Minion1 => concat!(stringify!([<$field:upper>]), "MN1"),
                                AbilityName::Minion2 => concat!(stringify!([<$field:upper>]), "MN2"),
                                AbilityName::Minion3 => concat!(stringify!([<$field:upper>]), "MN3"),
                                AbilityName::MinionMax => concat!(stringify!([<$field:upper>]), "MMNX"),
                                AbilityName::Monster => concat!(stringify!([<$field:upper>]), "MSTR"),
                                AbilityName::Monster1 => concat!(stringify!([<$field:upper>]), "MST1"),
                                AbilityName::Monster2 => concat!(stringify!([<$field:upper>]), "MST2"),
                                AbilityName::Monster3 => concat!(stringify!([<$field:upper>]), "MST3"),
                                AbilityName::Monster4 => concat!(stringify!([<$field:upper>]), "MST4"),
                                AbilityName::MonsterMax => concat!(stringify!([<$field:upper>]), "MMST"),
                                AbilityName::Void => stringify!([<$field:upper>]),
                                AbilityName::_1Max => concat!(stringify!([<$field:upper>]), "MAX1"),
                                AbilityName::_2Max => concat!(stringify!([<$field:upper>]), "MAX2"),
                                AbilityName::_3Max => concat!(stringify!([<$field:upper>]), "MAX3"),
                                AbilityName::_4Max => concat!(stringify!([<$field:upper>]), "MAX4"),
                                AbilityName::_5Max => concat!(stringify!([<$field:upper>]), "MAX5"),
                                AbilityName::_6Max => concat!(stringify!([<$field:upper>]), "MAX6"),
                                AbilityName::_7Max => concat!(stringify!([<$field:upper>]), "MAX7"),
                                AbilityName::_8Max => concat!(stringify!([<$field:upper>]), "MAX8"),
                                AbilityName::_1Min => concat!(stringify!([<$field:upper>]), "MIN1"),
                                AbilityName::_2Min => concat!(stringify!([<$field:upper>]), "MIN2"),
                                AbilityName::_3Min => concat!(stringify!([<$field:upper>]), "MIN3"),
                                AbilityName::_4Min => concat!(stringify!([<$field:upper>]), "MIN4"),
                                AbilityName::_5Min => concat!(stringify!([<$field:upper>]), "MIN5"),
                                AbilityName::_6Min => concat!(stringify!([<$field:upper>]), "MIN6"),
                                AbilityName::_7Min => concat!(stringify!([<$field:upper>]), "MIN7"),
                                AbilityName::_8Min => concat!(stringify!([<$field:upper>]), "MIN8"),
                            }
                        },
                        _ => {
                            "UNDF"
                        }
                    }
                }
            }
        }
    };
}

impl_key!(p);
impl_key!(q);
impl_key!(w);
impl_key!(e);
impl_key!(r);

#[derive(Clone, Copy, Serialize)]
pub enum AbilityName {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    Mega,
    Max,
    Min,
    Minion,
    Minion1,
    Minion2,
    Minion3,
    MinionMax,
    Monster,
    Monster1,
    Monster2,
    Monster3,
    Monster4,
    MonsterMax,
    Void,
    _1Max,
    _2Max,
    _3Max,
    _4Max,
    _5Max,
    _6Max,
    _7Max,
    _8Max,
    _1Min,
    _2Min,
    _3Min,
    _4Min,
    _5Min,
    _6Min,
    _7Min,
    _8Min,
}

pub struct CachedChampion {
    pub adaptative_type: AdaptativeType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: CachedChampionStats,
    pub abilities: &'static [(AbilityLike, CachedChampionAbility)],
}

struct _V((AbilityLike, CachedChampionAbility));

pub struct CachedChampionAbility {
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

pub struct CachedChampionStatsMap {
    pub flat: f64,
    pub per_level: f64,
}

pub struct CachedChampionStats {
    pub health: CachedChampionStatsMap,
    pub mana: CachedChampionStatsMap,
    pub armor: CachedChampionStatsMap,
    pub magic_resistance: CachedChampionStatsMap,
    pub attack_damage: CachedChampionStatsMap,
    pub attack_speed: CachedChampionStatsMap,
    pub movespeed: f64,
    pub critical_strike_damage: f64,
    pub critical_strike_damage_modifier: f64,
    pub attack_speed_ratio: f64,
    pub attack_range: f64,
    pub aram_damage_taken: f64,
    pub aram_damage_dealt: f64,
    pub urf_damage_taken: f64,
    pub urf_damage_dealt: f64,
}

pub struct CachedItemDamages {
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

pub enum StatName {
    AbilityHaste(f64),
    AbilityPower(f64),
    Armor(f64),
    ArmorPenetration(f64),
    MagicPenetration(f64),
    AttackDamage(f64),
    AttackSpeed(f64),
    GoldPer10Seconds(f64),
    AdaptiveForce(f64),
    CriticalStrikeChance(f64),
    CriticalStrikeDamage(f64),
    Health(f64),
    LifeSteal(f64),
    MagicResist(f64),
    Mana(f64),
    MoveSpeed(f64),
    Omnivamp(f64),
    BaseHealthRegen(f64),
    BaseManaRegen(f64),
    Tenacity(f64),
    HealAndShieldPower(f64),
}

pub struct CachedItem {
    pub gold: u16,
    pub prettified_stats: &'static [StatName],
    pub damage_type: Option<DamageType>,
    pub stats: CachedItemStats,
    pub ranged: CachedItemDamages,
    pub melee: CachedItemDamages,
    pub attributes: Attrs,
}

pub struct CachedMetaItem {
    pub jungle: &'static [u32],
    pub top: &'static [u32],
    pub mid: &'static [u32],
    pub adc: &'static [u32],
    pub support: &'static [u32],
}

pub struct CachedRune {
    pub damage_type: DamageType,
    pub ranged: fn(u8, &EvalContext) -> f64,
    pub melee: fn(u8, &EvalContext) -> f64,
}

pub struct CachedItemStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_percent: f64,
    pub armor_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    pub critical_strike_chance: f64,
    pub critical_strike_damage: f64,
    pub health: f64,
    pub lifesteal: f64,
    pub magic_resistance: f64,
    pub mana: f64,
    pub movespeed: f64,
    pub omnivamp: f64,
}

#[inline(always)]
pub const fn zero(_: u8, _: &EvalContext) -> f64 {
    0.0
}

#[derive(Copy, Clone)]
pub struct DamageExpression {
    pub level: u8,
    pub attributes: Attrs,
    pub damage_type: DamageType,
    pub minimum_damage: fn(u8, &EvalContext) -> f64,
    pub maximum_damage: fn(u8, &EvalContext) -> f64,
}

pub static CHAMPION_INDEX_TO_ID: [&'static str; 171] = [
    "Aatrox",
    "Ahri",
    "Akali",
    "Akshan",
    "Alistar",
    "Ambessa",
    "Amumu",
    "Anivia",
    "Annie",
    "Aphelios",
    "Ashe",
    "AurelionSol",
    "Aurora",
    "Azir",
    "Bard",
    "Belveth",
    "Blitzcrank",
    "Brand",
    "Braum",
    "Briar",
    "Caitlyn",
    "Camille",
    "Cassiopeia",
    "Chogath",
    "Corki",
    "Darius",
    "Diana",
    "Draven",
    "DrMundo",
    "Ekko",
    "Elise",
    "Evelynn",
    "Ezreal",
    "Fiddlesticks",
    "Fiora",
    "Fizz",
    "Galio",
    "Gangplank",
    "Garen",
    "Gnar",
    "Gragas",
    "Graves",
    "Gwen",
    "Hecarim",
    "Heimerdinger",
    "Hwei",
    "Illaoi",
    "Irelia",
    "Ivern",
    "Janna",
    "JarvanIV",
    "Jax",
    "Jayce",
    "Jhin",
    "Jinx",
    "Kaisa",
    "Kalista",
    "Karma",
    "Karthus",
    "Kassadin",
    "Katarina",
    "Kayle",
    "Kayn",
    "Kennen",
    "Khazix",
    "Kindred",
    "Kled",
    "KogMaw",
    "KSante",
    "Leblanc",
    "LeeSin",
    "Leona",
    "Lillia",
    "Lissandra",
    "Lucian",
    "Lulu",
    "Lux",
    "Malphite",
    "Malzahar",
    "Maokai",
    "MasterYi",
    "Mel",
    "Milio",
    "MissFortune",
    "MonkeyKing",
    "Mordekaiser",
    "Morgana",
    "Naafiri",
    "Nami",
    "Nasus",
    "Nautilus",
    "Neeko",
    "Nidalee",
    "Nilah",
    "Nocturne",
    "Nunu",
    "Olaf",
    "Orianna",
    "Ornn",
    "Pantheon",
    "Poppy",
    "Pyke",
    "Qiyana",
    "Quinn",
    "Rakan",
    "Rammus",
    "RekSai",
    "Rell",
    "Renata",
    "Renekton",
    "Rengar",
    "Riven",
    "Rumble",
    "Ryze",
    "Samira",
    "Sejuani",
    "Senna",
    "Seraphine",
    "Sett",
    "Shaco",
    "Shen",
    "Shyvana",
    "Singed",
    "Sion",
    "Sivir",
    "Skarner",
    "Smolder",
    "Sona",
    "Soraka",
    "Swain",
    "Sylas",
    "Syndra",
    "TahmKench",
    "Taliyah",
    "Talon",
    "Taric",
    "Teemo",
    "Thresh",
    "Tristana",
    "Trundle",
    "Tryndamere",
    "TwistedFate",
    "Twitch",
    "Udyr",
    "Urgot",
    "Varus",
    "Vayne",
    "Veigar",
    "Velkoz",
    "Vex",
    "Vi",
    "Viego",
    "Viktor",
    "Vladimir",
    "Volibear",
    "Warwick",
    "Xayah",
    "Xerath",
    "XinZhao",
    "Yasuo",
    "Yone",
    "Yorick",
    "Yunara",
    "Yuumi",
    "Zac",
    "Zed",
    "Zeri",
    "Ziggs",
    "Zilean",
    "Zoe",
    "Zyra",
];
