use super::*;
impl ChampionId {
    pub const unsafe fn from_u8_unchecked(id: u8) -> Self {
        unsafe { core::mem::transmute(id) }
    }
    pub const fn from_u8(id: u8) -> Option<Self> {
        if id < 172 as u8 {
            Some(unsafe { Self::from_u8_unchecked(id) })
        } else {
            None
        }
    }
}
#[derive(
    Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Decode, Encode,
)]
#[repr(u8)]
pub enum ChampionId {
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
    DrMundo,
    Draven,
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
    KSante,
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
}
pub static AATROX: CachedChampion = CachedChampion {
    name: "Aatrox",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::P(AbilityName::Void),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 4.8f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.651f32,
            per_level: 2.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.651000022888183f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.7f32,
        urf_damage_dealt: 1.15f32,
    },
    merge_data: &[(3, 7), (4, 8), (9, 10), (2, 6), (1, 5)],
    closures: &[
        aatrox_p_void,
        aatrox_q_min,
        aatrox_q_1min,
        aatrox_q_2min,
        aatrox_q_3min,
        aatrox_q_max,
        aatrox_q_1max,
        aatrox_q_2max,
        aatrox_q_3max,
        aatrox_w_min,
        aatrox_w_max,
    ],
};
pub const fn aatrox_p_void(ctx: &EvalContext) -> f32 {
    match ctx.level as u8 {
        1 => 0.04f32 * ctx.enemy_bonus_health,
        2 => 0.042352941176470586f32 * ctx.enemy_bonus_health,
        3 => 0.04470588235294118f32 * ctx.enemy_bonus_health,
        4 => 0.047058823529411764f32 * ctx.enemy_bonus_health,
        5 => 0.049411764705882356f32 * ctx.enemy_bonus_health,
        6 => 0.05176470588235294f32 * ctx.enemy_bonus_health,
        7 => 0.05411764705882353f32 * ctx.enemy_bonus_health,
        8 => 0.05647058823529412f32 * ctx.enemy_bonus_health,
        9 => 0.058823529411764705f32 * ctx.enemy_bonus_health,
        10 => 0.06117647058823529f32 * ctx.enemy_bonus_health,
        11 => 0.06352941176470589f32 * ctx.enemy_bonus_health,
        12 => 0.06588235294117648f32 * ctx.enemy_bonus_health,
        13 => 0.06823529411764706f32 * ctx.enemy_bonus_health,
        14 => 0.07058823529411765f32 * ctx.enemy_bonus_health,
        15 => 0.07294117647058823f32 * ctx.enemy_bonus_health,
        16 => 0.07529411764705882f32 * ctx.enemy_bonus_health,
        17 => 0.07764705882352942f32 * ctx.enemy_bonus_health,
        18 => 0.08f32 * ctx.enemy_bonus_health,
        _ => 0.0,
    }
}
pub const fn aatrox_q_min(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            10f32
                + 0.6f32 * ctx.ad
                + (12.5f32 + 0.75f32 * ctx.ad)
                + (15f32 + 0.9f32 * ctx.ad)
        }
        2 => {
            25f32
                + 0.675f32 * ctx.ad
                + (31.25f32 + 0.84375f32 * ctx.ad)
                + (37.5f32 + 1.0125f32 * ctx.ad)
        }
        3 => {
            40f32
                + 0.75f32 * ctx.ad
                + (50f32 + 0.9375f32 * ctx.ad)
                + (60f32 + 1.125f32 * ctx.ad)
        }
        4 => {
            55f32
                + 0.825f32 * ctx.ad
                + (68.75f32 + 1.03125f32 * ctx.ad)
                + (82.5f32 + 1.2375f32 * ctx.ad)
        }
        5 => {
            70f32
                + 0.9f32 * ctx.ad
                + (87.5f32 + 1.125f32 * ctx.ad)
                + (105f32 + 1.35f32 * ctx.ad)
        }
        _ => 0.0,
    }
}
pub const fn aatrox_q_1min(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.6f32 * ctx.ad,
        2 => 25f32 + 0.675f32 * ctx.ad,
        3 => 40f32 + 0.75f32 * ctx.ad,
        4 => 55f32 + 0.825f32 * ctx.ad,
        5 => 70f32 + 0.9f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_q_2min(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 12.5f32 + 0.75f32 * ctx.ad,
        2 => 31.25f32 + 0.84375f32 * ctx.ad,
        3 => 50f32 + 0.9375f32 * ctx.ad,
        4 => 68.75f32 + 1.03125f32 * ctx.ad,
        5 => 87.5f32 + 1.125f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_q_3min(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15f32 + 0.9f32 * ctx.ad,
        2 => 37.5f32 + 1.0125f32 * ctx.ad,
        3 => 60f32 + 1.125f32 * ctx.ad,
        4 => 82.5f32 + 1.2375f32 * ctx.ad,
        5 => 105f32 + 1.35f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_q_max(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            17f32
                + 1.02f32 * ctx.ad
                + (21.25f32 + 1.275f32 * ctx.ad)
                + (25.5f32 + 1.53f32 * ctx.ad)
        }
        2 => {
            42.5f32
                + 1.1475f32 * ctx.ad
                + (53.125f32 + 1.434375f32 * ctx.ad)
                + (63.75f32 + 1.72125f32 * ctx.ad)
        }
        3 => {
            68f32
                + 1.275f32 * ctx.ad
                + (85f32 + 1.59375f32 * ctx.ad)
                + (102f32 + 1.9125f32 * ctx.ad)
        }
        4 => {
            93.5f32
                + 1.4025f32 * ctx.ad
                + (116.875f32 + 1.753125f32 * ctx.ad)
                + (140.25f32 + 2.10375f32 * ctx.ad)
        }
        5 => {
            119f32
                + 1.53f32 * ctx.ad
                + (148.75f32 + 1.9125f32 * ctx.ad)
                + (178.5f32 + 2.295f32 * ctx.ad)
        }
        _ => 0.0,
    }
}
pub const fn aatrox_q_1max(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 17f32 + 1.02f32 * ctx.ad,
        2 => 42.5f32 + 1.1475f32 * ctx.ad,
        3 => 68f32 + 1.275f32 * ctx.ad,
        4 => 93.5f32 + 1.4025f32 * ctx.ad,
        5 => 119f32 + 1.53f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_q_2max(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 21.25f32 + 1.275f32 * ctx.ad,
        2 => 53.125f32 + 1.434375f32 * ctx.ad,
        3 => 85f32 + 1.59375f32 * ctx.ad,
        4 => 116.875f32 + 1.753125f32 * ctx.ad,
        5 => 148.75f32 + 1.9125f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_q_3max(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 25.5f32 + 1.53f32 * ctx.ad,
        2 => 63.75f32 + 1.72125f32 * ctx.ad,
        3 => 102f32 + 1.9125f32 * ctx.ad,
        4 => 140.25f32 + 2.10375f32 * ctx.ad,
        5 => 178.5f32 + 2.295f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_w_min(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.4f32 * ctx.ad,
        2 => 40f32 + 0.4f32 * ctx.ad,
        3 => 50f32 + 0.4f32 * ctx.ad,
        4 => 60f32 + 0.4f32 * ctx.ad,
        5 => 70f32 + 0.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn aatrox_w_max(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.8f32 * ctx.ad,
        2 => 80f32 + 0.8f32 * ctx.ad,
        3 => 100f32 + 0.8f32 * ctx.ad,
        4 => 120f32 + 0.8f32 * ctx.ad,
        5 => 140f32 + 0.8f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static AHRI: CachedChampion = CachedChampion {
    name: "Ahri",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Min),
            damage_type: DamageType::Magic,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Max),
            damage_type: DamageType::Mixed,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::Min),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::Max),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::Void),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::Min),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::Max),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 418f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 53f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.668f32,
            per_level: 2.2f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[(0, 1), (3, 4), (6, 7)],
    closures: &[
        ahri_q_min,
        ahri_q_max,
        ahri_w_1,
        ahri_w_min,
        ahri_w_max,
        ahri_e_void,
        ahri_r_min,
        ahri_r_max,
    ],
};
pub const fn ahri_q_min(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.5f32 * ctx.ap,
        2 => 65f32 + 0.5f32 * ctx.ap,
        3 => 90f32 + 0.5f32 * ctx.ap,
        4 => 115f32 + 0.5f32 * ctx.ap,
        5 => 140f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ahri_q_max(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            (40f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier
                + (40f32 + 0.5f32 * ctx.ap)
        }
        2 => {
            (65f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier
                + (65f32 + 0.5f32 * ctx.ap)
        }
        3 => {
            (90f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier
                + (90f32 + 0.5f32 * ctx.ap)
        }
        4 => {
            (115f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier
                + (115f32 + 0.5f32 * ctx.ap)
        }
        5 => {
            (140f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier
                + (140f32 + 0.5f32 * ctx.ap)
        }
        _ => 0.0,
    }
}
pub const fn ahri_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 12f32 + 0.12f32 * ctx.ap,
        2 => 18f32 + 0.12f32 * ctx.ap,
        3 => 24f32 + 0.12f32 * ctx.ap,
        4 => 30f32 + 0.12f32 * ctx.ap,
        5 => 36f32 + 0.12f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ahri_w_min(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.4f32 * ctx.ap,
        2 => 60f32 + 0.4f32 * ctx.ap,
        3 => 80f32 + 0.4f32 * ctx.ap,
        4 => 100f32 + 0.4f32 * ctx.ap,
        5 => 120f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ahri_w_max(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 64f32 + 0.64f32 * ctx.ap,
        2 => 96f32 + 0.64f32 * ctx.ap,
        3 => 128f32 + 0.64f32 * ctx.ap,
        4 => 160f32 + 0.64f32 * ctx.ap,
        5 => 192f32 + 0.64f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ahri_e_void(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.85f32 * ctx.ap,
        2 => 120f32 + 0.85f32 * ctx.ap,
        3 => 160f32 + 0.85f32 * ctx.ap,
        4 => 200f32 + 0.85f32 * ctx.ap,
        5 => 240f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ahri_r_min(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 60f32 + 0.35f32 * ctx.ap,
        2 => 90f32 + 0.35f32 * ctx.ap,
        3 => 120f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ahri_r_max(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 3f32 * (60f32 + 0.35f32 * ctx.ap),
        2 => 3f32 * (90f32 + 0.35f32 * ctx.ap),
        3 => 3f32 * (120f32 + 0.35f32 * ctx.ap),
        _ => 0.0,
    }
}
pub static AKALI: CachedChampion = CachedChampion {
    name: "Akali",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::P(AbilityName::Void),
            damage_type: DamageType::Magic,
            attributes: Attrs::Onhit,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Void),
            damage_type: DamageType::Magic,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1Min),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::Max),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1Max),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2Min),
            damage_type: DamageType::Magic,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2Max),
            damage_type: DamageType::Magic,
            attributes: Attrs::Area,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 119f32,
        },
        mana: CachedChampionStatsMap {
            flat: 200f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 23f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.2f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[(2, 4), (6, 7)],
    closures: &[
        akali_p_void,
        akali_q_void,
        akali_e_1min,
        akali_e_max,
        akali_e_1max,
        akali_r_1,
        akali_r_2min,
        akali_r_2max,
    ],
};
pub const fn akali_p_void(ctx: &EvalContext) -> f32 {
    match ctx.level as u8 {
        1 => 35f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        2 => 43.64705882352941f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        3 => 52.294117647058826f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        4 => 60.94117647058823f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        5 => 69.58823529411765f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        6 => 78.23529411764706f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        7 => 86.88235294117646f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        8 => 95.52941176470588f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        9 => 104.17647058823529f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        10 => 112.82352941176471f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        11 => 121.47058823529412f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        12 => 130.11764705882354f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        13 => 138.76470588235293f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        14 => 147.41176470588235f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        15 => 156.05882352941177f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        16 => 164.7058823529412f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        17 => 173.35294117647058f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        18 => 182f32 + 0.6f32 * ctx.bonus_ad + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_q_void(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,
        2 => 70f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,
        3 => 95f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,
        4 => 120f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,
        5 => 145f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_e_1min(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 21f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,
        2 => 42f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,
        3 => 63f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,
        4 => 84f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,
        5 => 105f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_e_max(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + ctx.ad + 1.1f32 * ctx.ap,
        2 => 140f32 + ctx.ad + 1.1f32 * ctx.ap,
        3 => 210f32 + ctx.ad + 1.1f32 * ctx.ap,
        4 => 280f32 + ctx.ad + 1.1f32 * ctx.ap,
        5 => 350f32 + ctx.ad + 1.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_e_1max(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 49f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,
        2 => 98f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,
        3 => 147f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,
        4 => 196f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,
        5 => 245f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 110f32 + 0.5f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        2 => 220f32 + 0.5f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        3 => 330f32 + 0.5f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_r_2min(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 210f32 + 0.9f32 * ctx.ap,
        2 => 420f32 + 0.9f32 * ctx.ap,
        3 => 630f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akali_r_2max(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.3f32 * ctx.ap,
        2 => 140f32 + 0.3f32 * ctx.ap,
        3 => 210f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static AKSHAN: CachedChampion = CachedChampion {
    name: "Akshan",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::P(AbilityName::Void),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Area,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::Void),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2Min),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2Max),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 107f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 52f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 4f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.4f32,
        attack_range: 500f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[(1, 2), (4, 6), (5, 7)],
    closures: &[
        akshan_p_void,
        akshan_q_min,
        akshan_q_max,
        akshan_e_void,
        akshan_r_1min,
        akshan_r_2min,
        akshan_r_1max,
        akshan_r_2max,
    ],
};
pub const fn akshan_p_void(ctx: &EvalContext) -> f32 {
    match ctx.level as u8 {
        1 => 15f32 + 0.6f32 * ctx.ap,
        2 => 15f32 + 0.6f32 * ctx.ap,
        3 => 15f32 + 0.6f32 * ctx.ap,
        4 => 15f32 + 0.6f32 * ctx.ap,
        5 => 15f32 + 0.6f32 * ctx.ap,
        6 => 15f32 + 0.6f32 * ctx.ap,
        7 => 40f32 + 0.6f32 * ctx.ap,
        8 => 40f32 + 0.6f32 * ctx.ap,
        9 => 40f32 + 0.6f32 * ctx.ap,
        10 => 40f32 + 0.6f32 * ctx.ap,
        11 => 40f32 + 0.6f32 * ctx.ap,
        12 => 80f32 + 0.6f32 * ctx.ap,
        13 => 80f32 + 0.6f32 * ctx.ap,
        14 => 80f32 + 0.6f32 * ctx.ap,
        15 => 80f32 + 0.6f32 * ctx.ap,
        16 => 80f32 + 0.6f32 * ctx.ap,
        17 => 150f32 + 0.6f32 * ctx.ap,
        18 => 150f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn akshan_q_min(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.8f32 * ctx.ad,
        2 => 25f32 + 0.8f32 * ctx.ad,
        3 => 45f32 + 0.8f32 * ctx.ad,
        4 => 65f32 + 0.8f32 * ctx.ad,
        5 => 85f32 + 0.8f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn akshan_q_max(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 1.6f32 * ctx.ad,
        2 => 50f32 + 1.6f32 * ctx.ad,
        3 => 90f32 + 1.6f32 * ctx.ad,
        4 => 130f32 + 1.6f32 * ctx.ad,
        5 => 170f32 + 1.6f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn akshan_e_void(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 15f32 + 0.15f32 * ctx.ad + 0.01f32 * ctx.attack_speed,
        2 => 30f32 + 0.15f32 * ctx.ad + 0.01f32 * ctx.attack_speed,
        3 => 45f32 + 0.15f32 * ctx.ad + 0.01f32 * ctx.attack_speed,
        4 => 60f32 + 0.15f32 * ctx.ad + 0.01f32 * ctx.attack_speed,
        5 => 75f32 + 0.15f32 * ctx.ad + 0.01f32 * ctx.attack_speed,
        _ => 0.0,
    }
}
pub const fn akshan_r_1min(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => (25f32 + 0.15f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        2 => (30f32 + 0.15f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        3 => (35f32 + 0.15f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        _ => 0.0,
    }
}
pub const fn akshan_r_2min(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => (125f32 + 0.75f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        2 => (210f32 + 0.9f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        3 => (315f32 + 1.05f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        _ => 0.0,
    }
}
pub const fn akshan_r_1max(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => (75f32 + 0.45f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        2 => (90f32 + 0.45f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        3 => (105f32 + 0.45f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        _ => 0.0,
    }
}
pub const fn akshan_r_2max(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => (375f32 + 2.25f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        2 => (630f32 + 2.7f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        3 => (945f32 + 3.15f32 * ctx.ad) * (1f32 + ctx.crit_chance / 200f32),
        _ => 0.0,
    }
}
pub static ALISTAR: CachedChampion = CachedChampion {
    name: "Alistar",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 685f32,
            per_level: 120f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 40f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 3.75f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.125f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        alistar_q_1,
        alistar_w_1,
        alistar_e_1,
        alistar_e_2,
        alistar_r_1,
    ],
};
pub const fn alistar_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.8f32 * ctx.ap,
        2 => 100f32 + 0.8f32 * ctx.ap,
        3 => 140f32 + 0.8f32 * ctx.ap,
        4 => 180f32 + 0.8f32 * ctx.ap,
        5 => 220f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn alistar_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 55f32 + ctx.ap,
        2 => 110f32 + ctx.ap,
        3 => 165f32 + ctx.ap,
        4 => 220f32 + ctx.ap,
        5 => 275f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn alistar_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 8f32 + 0.07f32 * ctx.ap,
        2 => 11f32 + 0.07f32 * ctx.ap,
        3 => 14f32 + 0.07f32 * ctx.ap,
        4 => 17f32 + 0.07f32 * ctx.ap,
        5 => 20f32 + 0.07f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn alistar_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 110f32 + 0.7f32 * ctx.ap,
        3 => 140f32 + 0.7f32 * ctx.ap,
        4 => 170f32 + 0.7f32 * ctx.ap,
        5 => 200f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn alistar_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.55f32,
        2 => 0.65f32,
        3 => 0.75f32,
        _ => 0.0,
    }
}
pub static AMBESSA: CachedChampion = CachedChampion {
    name: "Ambessa",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 200f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 4.9f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        ambessa_q_3,
        ambessa_q_4,
        ambessa_w_1,
        ambessa_w_2,
        ambessa_e_1,
        ambessa_e_2,
        ambessa_r_1,
    ],
};
pub const fn ambessa_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            40f32
                + 0.6f32 * ctx.bonus_ad
                + 0.02f32 * ctx.enemy_max_health
                + 0.03f32 * 0.01f32 * ctx.bonus_ad
        }
        2 => {
            60f32
                + 0.6f32 * ctx.bonus_ad
                + 0.03f32 * ctx.enemy_max_health
                + 0.03f32 * 0.01f32 * ctx.bonus_ad
        }
        3 => {
            80f32
                + 0.6f32 * ctx.bonus_ad
                + 0.04f32 * ctx.enemy_max_health
                + 0.03f32 * 0.01f32 * ctx.bonus_ad
        }
        4 => {
            100f32
                + 0.6f32 * ctx.bonus_ad
                + 0.05f32 * ctx.enemy_max_health
                + 0.03f32 * 0.01f32 * ctx.bonus_ad
        }
        5 => {
            120f32
                + 0.6f32 * ctx.bonus_ad
                + 0.06f32 * ctx.enemy_max_health
                + 0.03f32 * 0.01f32 * ctx.bonus_ad
        }
        _ => 0.0,
    }
}
pub const fn ambessa_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            20f32
                + 0.3f32 * ctx.bonus_ad
                + 0.01f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.bonus_ad
        }
        2 => {
            30f32
                + 0.3f32 * ctx.bonus_ad
                + 0.015f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.bonus_ad
        }
        3 => {
            40f32
                + 0.3f32 * ctx.bonus_ad
                + 0.02f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.bonus_ad
        }
        4 => {
            50f32
                + 0.3f32 * ctx.bonus_ad
                + 0.025f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.bonus_ad
        }
        5 => {
            60f32
                + 0.3f32 * ctx.bonus_ad
                + 0.03f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.bonus_ad
        }
        _ => 0.0,
    }
}
pub const fn ambessa_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.5f32 * ctx.bonus_ad,
        2 => 75f32 + 0.5f32 * ctx.bonus_ad,
        3 => 100f32 + 0.5f32 * ctx.bonus_ad,
        4 => 125f32 + 0.5f32 * ctx.bonus_ad,
        5 => 150f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn ambessa_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 75f32 + 0.75f32 * ctx.bonus_ad,
        2 => 112.5f32 + 0.75f32 * ctx.bonus_ad,
        3 => 150f32 + 0.75f32 * ctx.bonus_ad,
        4 => 187.5f32 + 0.75f32 * ctx.bonus_ad,
        5 => 225f32 + 0.75f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn ambessa_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 40f32 + 0.4f32 * ctx.bonus_ad,
        2 => 60f32 + 0.45f32 * ctx.bonus_ad,
        3 => 80f32 + 0.5f32 * ctx.bonus_ad,
        4 => 100f32 + 0.55f32 * ctx.bonus_ad,
        5 => 120f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn ambessa_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.8f32 * ctx.bonus_ad,
        2 => 120f32 + 0.9f32 * ctx.bonus_ad,
        3 => 160f32 + ctx.bonus_ad,
        4 => 200f32 + 1.1f32 * ctx.bonus_ad,
        5 => 240f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn ambessa_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.8f32 * ctx.bonus_ad,
        2 => 250f32 + 0.8f32 * ctx.bonus_ad,
        3 => 350f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static AMUMU: CachedChampion = CachedChampion {
    name: "Amumu",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 685f32,
            per_level: 94f32,
        },
        mana: CachedChampionStatsMap {
            flat: 285f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 57f32,
            per_level: 3.8f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.736f32,
            per_level: 2.18f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[amumu_q_1, amumu_w_1, amumu_e_1, amumu_e_2, amumu_r_1],
};
pub const fn amumu_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.85f32 * ctx.ap,
        2 => 95f32 + 0.85f32 * ctx.ap,
        3 => 120f32 + 0.85f32 * ctx.ap,
        4 => 145f32 + 0.85f32 * ctx.ap,
        5 => 170f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn amumu_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            5f32 + 0.005f32 * ctx.enemy_max_health
                + 0.0025f32 * 0.01f32 * ctx.ap
        }
        2 => {
            5f32 + 0.00625f32 * ctx.enemy_max_health
                + 0.0025f32 * 0.01f32 * ctx.ap
        }
        3 => {
            5f32 + 0.0075f32 * ctx.enemy_max_health
                + 0.0025f32 * 0.01f32 * ctx.ap
        }
        4 => {
            5f32 + 0.00875f32 * ctx.enemy_max_health
                + 0.0025f32 * 0.01f32 * ctx.ap
        }
        5 => {
            5f32 + 0.01f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn amumu_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 65f32 + 0.5f32 * ctx.ap,
        2 => 95f32 + 0.5f32 * ctx.ap,
        3 => 125f32 + 0.5f32 * ctx.ap,
        4 => 155f32 + 0.5f32 * ctx.ap,
        5 => 185f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn amumu_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn amumu_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.8f32 * ctx.ap,
        2 => 300f32 + 0.8f32 * ctx.ap,
        3 => 400f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ANIVIA: CachedChampion = CachedChampion {
    name: "Anivia",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 550f32,
            per_level: 92f32,
        },
        mana: CachedChampionStatsMap {
            flat: 495f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 51f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 1.68f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 600f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.92f32,
        urf_damage_dealt: 1.15f32,
    },
    merge_data: &[],
    closures: &[
        anivia_q_1, anivia_q_2, anivia_q_3, anivia_e_1, anivia_e_2, anivia_r_1,
        anivia_r_2,
    ],
};
pub const fn anivia_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 50f32 + 0.25f32 * ctx.ap,
        2 => 70f32 + 0.25f32 * ctx.ap,
        3 => 90f32 + 0.25f32 * ctx.ap,
        4 => 110f32 + 0.25f32 * ctx.ap,
        5 => 130f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn anivia_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.45f32 * ctx.ap,
        2 => 95f32 + 0.45f32 * ctx.ap,
        3 => 130f32 + 0.45f32 * ctx.ap,
        4 => 165f32 + 0.45f32 * ctx.ap,
        5 => 200f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn anivia_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 110f32 + 0.7f32 * ctx.ap,
        2 => 165f32 + 0.7f32 * ctx.ap,
        3 => 220f32 + 0.7f32 * ctx.ap,
        4 => 275f32 + 0.7f32 * ctx.ap,
        5 => 330f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn anivia_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 100f32 + 1.1f32 * ctx.ap,
        2 => 150f32 + 1.1f32 * ctx.ap,
        3 => 200f32 + 1.1f32 * ctx.ap,
        4 => 250f32 + 1.1f32 * ctx.ap,
        5 => 300f32 + 1.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn anivia_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.55f32 * ctx.ap,
        2 => 75f32 + 0.55f32 * ctx.ap,
        3 => 100f32 + 0.55f32 * ctx.ap,
        4 => 125f32 + 0.55f32 * ctx.ap,
        5 => 150f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn anivia_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 15f32 + 0.0625f32 * ctx.ap,
        2 => 22.5f32 + 0.0625f32 * ctx.ap,
        3 => 30f32 + 0.0625f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn anivia_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 45f32 + 0.1875f32 * ctx.ap,
        2 => 67.5f32 + 0.1875f32 * ctx.ap,
        3 => 90f32 + 0.1875f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ANNIE: CachedChampion = CachedChampion {
    name: "Annie",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 560f32,
            per_level: 96f32,
        },
        mana: CachedChampionStatsMap {
            flat: 418f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 23f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 50f32,
            per_level: 2.65f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.61f32,
            per_level: 1.36f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 625f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[annie_q_1, annie_w_1, annie_e_1, annie_r_1],
};
pub const fn annie_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 120f32 + 0.8f32 * ctx.ap,
        3 => 160f32 + 0.8f32 * ctx.ap,
        4 => 200f32 + 0.8f32 * ctx.ap,
        5 => 240f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn annie_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.8f32 * ctx.ap,
        2 => 115f32 + 0.8f32 * ctx.ap,
        3 => 160f32 + 0.8f32 * ctx.ap,
        4 => 205f32 + 0.8f32 * ctx.ap,
        5 => 250f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn annie_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 25f32 + 0.4f32 * ctx.ap,
        2 => 35f32 + 0.4f32 * ctx.ap,
        3 => 45f32 + 0.4f32 * ctx.ap,
        4 => 55f32 + 0.4f32 * ctx.ap,
        5 => 65f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn annie_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.75f32 * ctx.ap,
        2 => 275f32 + 0.75f32 * ctx.ap,
        3 => 400f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static APHELIOS: CachedChampion = CachedChampion {
    name: "Aphelios",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[TypeMetadata {
        kind: AbilityId::P(AbilityName::_1),
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    }],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 102f32,
        },
        mana: CachedChampionStatsMap {
            flat: 348f32,
            per_level: 42f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 2.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.665f32,
            per_level: 2.1f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[aphelios_p_1],
};
pub const fn aphelios_p_1(ctx: &EvalContext) -> f32 {
    match ctx.level as u8 {
        1 => 5f32,
        2 => 10f32,
        3 => 15f32,
        4 => 20f32,
        5 => 25f32,
        6 => 30f32,
        _ => 0.0,
    }
}
pub static ASHE: CachedChampion = CachedChampion {
    name: "Ashe",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 101f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 2.95f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.33f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 100f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 600f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[ashe_q_1, ashe_q_2, ashe_w_1, ashe_r_1],
};
pub const fn ashe_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.22f32 * ctx.ad,
        2 => 0.235f32 * ctx.ad,
        3 => 0.25f32 * ctx.ad,
        4 => 0.265f32 * ctx.ad,
        5 => 0.28f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn ashe_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 1.1f32 * ctx.ad,
        2 => 1.175f32 * ctx.ad,
        3 => 1.25f32 * ctx.ad,
        4 => 1.325f32 * ctx.ad,
        5 => 1.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn ashe_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 1.1f32 * ctx.bonus_ad,
        2 => 95f32 + 1.1f32 * ctx.bonus_ad,
        3 => 130f32 + 1.1f32 * ctx.bonus_ad,
        4 => 165f32 + 1.1f32 * ctx.bonus_ad,
        5 => 200f32 + 1.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn ashe_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.2f32 * ctx.ap,
        2 => 400f32 + 1.2f32 * ctx.ap,
        3 => 600f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static AURELIONSOL: CachedChampion = CachedChampion {
    name: "Aurelion Sol",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 90f32,
        },
        mana: CachedChampionStatsMap {
            flat: 530f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        aurelionsol_q_1,
        aurelionsol_q_2,
        aurelionsol_q_3,
        aurelionsol_q_4,
        aurelionsol_w_1,
        aurelionsol_e_1,
        aurelionsol_e_2,
        aurelionsol_r_2,
    ],
};
pub const fn aurelionsol_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5.625f32 + 0.06875f32 * ctx.ap,
        2 => 7.5f32 + 0.06875f32 * ctx.ap,
        3 => 9.375f32 + 0.06875f32 * ctx.ap,
        4 => 11.25f32 + 0.06875f32 * ctx.ap,
        5 => 13.125f32 + 0.06875f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurelionsol_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 2.8125f32 + 0.034375f32 * ctx.ap,
        2 => 3.75f32 + 0.034375f32 * ctx.ap,
        3 => 4.6875f32 + 0.034375f32 * ctx.ap,
        4 => 5.625f32 + 0.034375f32 * ctx.ap,
        5 => 6.5625f32 + 0.034375f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurelionsol_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 146.25f32 + 1.7875f32 * ctx.ap,
        2 => 195f32 + 1.7875f32 * ctx.ap,
        3 => 243.75f32 + 1.7875f32 * ctx.ap,
        4 => 292.5f32 + 1.7875f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurelionsol_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn aurelionsol_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 1.08f32,
        2 => 1.09f32,
        3 => 1.1f32,
        4 => 1.11f32,
        5 => 1.12f32,
        _ => 0.0,
    }
}
pub const fn aurelionsol_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 2.5f32 + 0.03f32 * ctx.ap,
        2 => 3.75f32 + 0.03f32 * ctx.ap,
        3 => 5f32 + 0.03f32 * ctx.ap,
        4 => 6.25f32 + 0.03f32 * ctx.ap,
        5 => 7.5f32 + 0.03f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurelionsol_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.6f32 * ctx.ap,
        2 => 75f32 + 0.6f32 * ctx.ap,
        3 => 100f32 + 0.6f32 * ctx.ap,
        4 => 125f32 + 0.6f32 * ctx.ap,
        5 => 150f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurelionsol_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.75f32 * ctx.ap,
        2 => 250f32 + 0.75f32 * ctx.ap,
        3 => 350f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static AURORA: CachedChampion = CachedChampion {
    name: "Aurora",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 607f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 475f32,
            per_level: 30f32,
        },
        armor: CachedChampionStatsMap {
            flat: 23f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 53f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.668f32,
            per_level: 2f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.668f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        aurora_q_1, aurora_q_2, aurora_q_3, aurora_q_4, aurora_q_5, aurora_e_1,
        aurora_r_1,
    ],
};
pub const fn aurora_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.4f32 * ctx.ap,
        2 => 70f32 + 0.4f32 * ctx.ap,
        3 => 95f32 + 0.4f32 * ctx.ap,
        4 => 120f32 + 0.4f32 * ctx.ap,
        5 => 145f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurora_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 67.5f32 + 0.6f32 * ctx.ap,
        2 => 105f32 + 0.6f32 * ctx.ap,
        3 => 142.5f32 + 0.6f32 * ctx.ap,
        4 => 180f32 + 0.6f32 * ctx.ap,
        5 => 217.5f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurora_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.4f32 * ctx.ap,
        2 => 70f32 + 0.4f32 * ctx.ap,
        3 => 95f32 + 0.4f32 * ctx.ap,
        4 => 120f32 + 0.4f32 * ctx.ap,
        5 => 145f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurora_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 13.5f32 + 0.12f32 * ctx.ap,
        2 => 21f32 + 0.12f32 * ctx.ap,
        3 => 28.5f32 + 0.12f32 * ctx.ap,
        4 => 36f32 + 0.12f32 * ctx.ap,
        5 => 43.5f32 + 0.12f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurora_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 9f32 + 0.08f32 * ctx.ap,
        2 => 14f32 + 0.08f32 * ctx.ap,
        3 => 19f32 + 0.08f32 * ctx.ap,
        4 => 24f32 + 0.08f32 * ctx.ap,
        5 => 29f32 + 0.08f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurora_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 110f32 + 0.7f32 * ctx.ap,
        3 => 150f32 + 0.7f32 * ctx.ap,
        4 => 190f32 + 0.7f32 * ctx.ap,
        5 => 230f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn aurora_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 0.7f32 * ctx.ap,
        2 => 275f32 + 0.7f32 * ctx.ap,
        3 => 375f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static AZIR: CachedChampion = CachedChampion {
    name: "Azir",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 575f32,
            per_level: 119f32,
        },
        mana: CachedChampionStatsMap {
            flat: 320f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 5.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694f32,
        attack_range: 525f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[azir_q_1, azir_w_1, azir_e_1, azir_r_1],
};
pub const fn azir_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.35f32 * ctx.ap,
        2 => 80f32 + 0.35f32 * ctx.ap,
        3 => 100f32 + 0.35f32 * ctx.ap,
        4 => 120f32 + 0.35f32 * ctx.ap,
        5 => 140f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn azir_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0f32 + 50f32 + 0.45f32 * ctx.ap,
        2 => 2.6470588235294117f32 + 65f32 + 0.5f32 * ctx.ap,
        3 => 5.294117647058823f32 + 80f32 + 0.55f32 * ctx.ap,
        4 => 7.9411764705882355f32 + 95f32 + 0.6f32 * ctx.ap,
        5 => 10.588235294117649f32 + 110f32 + 0.65f32 * ctx.ap,
        6 => 13.235294117647058f32,
        7 => 15.882352941176473f32,
        8 => 18.52941176470588f32,
        9 => 21.176470588235293f32,
        10 => 23.823529411764707f32,
        11 => 26.470588235294116f32,
        12 => 29.11764705882353f32,
        13 => 31.764705882352946f32,
        14 => 34.411764705882355f32,
        15 => 37.05882352941176f32,
        16 => 39.705882352941174f32,
        17 => 42.35294117647059f32,
        18 => 45f32,
        _ => 0.0,
    }
}
pub const fn azir_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.4f32 * ctx.ap,
        2 => 100f32 + 0.4f32 * ctx.ap,
        3 => 140f32 + 0.4f32 * ctx.ap,
        4 => 180f32 + 0.4f32 * ctx.ap,
        5 => 220f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn azir_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.75f32 * ctx.ap,
        2 => 400f32 + 0.75f32 * ctx.ap,
        3 => 600f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static BARD: CachedChampion = CachedChampion {
    name: "Bard",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[TypeMetadata {
        kind: AbilityId::Q(AbilityName::_1),
        damage_type: DamageType::Magic,
        attributes: Attrs::Undefined,
    }],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 52f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 500f32,
        aram_damage_taken: 0.85f32,
        aram_damage_dealt: 1.15f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1.15f32,
    },
    merge_data: &[],
    closures: &[bard_q_1],
};
pub const fn bard_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 120f32 + 0.8f32 * ctx.ap,
        3 => 160f32 + 0.8f32 * ctx.ap,
        4 => 200f32 + 0.8f32 * ctx.ap,
        5 => 240f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static BELVETH: CachedChampion = CachedChampion {
    name: "Bel'Veth",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 1.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.85f32,
            per_level: 0f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.85f32,
        attack_range: 150f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        belveth_q_1,
        belveth_q_2,
        belveth_q_3,
        belveth_q_4,
        belveth_w_1,
        belveth_e_1,
        belveth_e_2,
        belveth_e_3,
        belveth_e_4,
        belveth_e_5,
        belveth_r_1,
        belveth_r_2,
        belveth_r_3,
    ],
};
pub const fn belveth_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0f32 + ctx.ad,
        2 => 5f32 + ctx.ad,
        3 => 10f32 + ctx.ad,
        4 => 15f32 + ctx.ad,
        5 => 20f32 + ctx.ad,
        _ => 0.0,
    }
}
pub const fn belveth_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.6f32,
        2 => 0.7f32,
        3 => 0.8f32,
        4 => 0.9f32,
        5 => 1f32,
        _ => 0.0,
    }
}
pub const fn belveth_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 55f32,
        2 => 65f32,
        3 => 75f32,
        4 => 85f32,
        5 => 95f32,
        _ => 0.0,
    }
}
pub const fn belveth_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 55f32 + ctx.ad,
        2 => 70f32 + ctx.ad,
        3 => 85f32 + ctx.ad,
        4 => 100f32 + ctx.ad,
        5 => 115f32 + ctx.ad,
        _ => 0.0,
    }
}
pub const fn belveth_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,
        2 => 110f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,
        3 => 150f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,
        4 => 190f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,
        5 => 230f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn belveth_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.35f32,
        2 => 0.4f32,
        3 => 0.45f32,
        4 => 0.5f32,
        5 => 0.55f32,
        _ => 0.0,
    }
}
pub const fn belveth_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 36f32 + 0.48f32 * ctx.ad,
        2 => 42f32 + 0.48f32 * ctx.ad,
        3 => 48f32 + 0.48f32 * ctx.ad,
        4 => 54f32 + 0.48f32 * ctx.ad,
        5 => 60f32 + 0.48f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn belveth_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 9f32 + 0.12f32 * ctx.ad,
        2 => 10.5f32 + 0.12f32 * ctx.ad,
        3 => 12f32 + 0.12f32 * ctx.ad,
        4 => 13.5f32 + 0.12f32 * ctx.ad,
        5 => 15f32 + 0.12f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn belveth_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 24f32 + 0.32f32 * ctx.ad,
        2 => 28f32 + 0.32f32 * ctx.ad,
        3 => 32f32 + 0.32f32 * ctx.ad,
        4 => 36f32 + 0.32f32 * ctx.ad,
        5 => 40f32 + 0.32f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn belveth_e_5(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 6f32 + 0.08f32 * ctx.ad,
        2 => 7f32 + 0.08f32 * ctx.ad,
        3 => 8f32 + 0.08f32 * ctx.ad,
        4 => 9f32 + 0.08f32 * ctx.ad,
        5 => 10f32 + 0.08f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn belveth_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + ctx.ap + 0.25f32 * ctx.missing_health,
        2 => 200f32 + ctx.ap + 0.25f32 * ctx.missing_health,
        3 => 250f32 + ctx.ap + 0.25f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn belveth_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 6f32 + 0.12f32 * ctx.bonus_ad,
        2 => 8f32 + 0.12f32 * ctx.bonus_ad,
        3 => 10f32 + 0.12f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn belveth_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32 + 0.6f32 * ctx.bonus_ad,
        2 => 40f32 + 0.6f32 * ctx.bonus_ad,
        3 => 50f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static BLITZCRANK: CachedChampion = CachedChampion {
    name: "Blitzcrank",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 267f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.13f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[blitzcrank_q_1, blitzcrank_r_1, blitzcrank_r_2],
};
pub const fn blitzcrank_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 110f32 + 1.2f32 * ctx.ap,
        2 => 160f32 + 1.2f32 * ctx.ap,
        3 => 210f32 + 1.2f32 * ctx.ap,
        4 => 260f32 + 1.2f32 * ctx.ap,
        5 => 310f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn blitzcrank_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 275f32 + ctx.ap,
        2 => 400f32 + ctx.ap,
        3 => 525f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn blitzcrank_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 50f32 + 0.3f32 * ctx.ap + 0.02f32 * ctx.max_mana,
        2 => 100f32 + 0.4f32 * ctx.ap + 0.02f32 * ctx.max_mana,
        3 => 150f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,
        _ => 0.0,
    }
}
pub static BRAND: CachedChampion = CachedChampion {
    name: "Brand",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle, Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 570f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 469f32,
            per_level: 21f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 57f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.681f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        brand_q_1, brand_w_1, brand_w_2, brand_e_1, brand_r_1, brand_r_2,
    ],
};
pub const fn brand_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.65f32 * ctx.ap,
        2 => 100f32 + 0.65f32 * ctx.ap,
        3 => 130f32 + 0.65f32 * ctx.ap,
        4 => 160f32 + 0.65f32 * ctx.ap,
        5 => 190f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn brand_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 93.75f32 + 0.75f32 * ctx.ap,
        2 => 150f32 + 0.75f32 * ctx.ap,
        3 => 206.25f32 + 0.75f32 * ctx.ap,
        4 => 262.5f32 + 0.75f32 * ctx.ap,
        5 => 318.75f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn brand_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 75f32 + 0.6f32 * ctx.ap,
        2 => 120f32 + 0.6f32 * ctx.ap,
        3 => 165f32 + 0.6f32 * ctx.ap,
        4 => 210f32 + 0.6f32 * ctx.ap,
        5 => 255f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn brand_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 0.6f32 * ctx.ap,
        2 => 80f32 + 0.6f32 * ctx.ap,
        3 => 105f32 + 0.6f32 * ctx.ap,
        4 => 130f32 + 0.6f32 * ctx.ap,
        5 => 155f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn brand_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.25f32 * ctx.ap,
        2 => 175f32 + 0.25f32 * ctx.ap,
        3 => 250f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn brand_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 0.75f32 * ctx.ap,
        2 => 525f32 + 0.75f32 * ctx.ap,
        3 => 750f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static BRAUM: CachedChampion = CachedChampion {
    name: "Braum",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 112f32,
        },
        mana: CachedChampionStatsMap {
            flat: 311f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 3.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.82f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[braum_q_1, braum_e_1, braum_r_1],
};
pub const fn braum_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.025f32 * ctx.max_health,
        2 => 125f32 + 0.025f32 * ctx.max_health,
        3 => 175f32 + 0.025f32 * ctx.max_health,
        4 => 225f32 + 0.025f32 * ctx.max_health,
        5 => 275f32 + 0.025f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn braum_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.35f32,
        2 => 0.4f32,
        3 => 0.45f32,
        4 => 0.5f32,
        5 => 0.55f32,
        _ => 0.0,
    }
}
pub const fn braum_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.6f32 * ctx.ap,
        2 => 300f32 + 0.6f32 * ctx.ap,
        3 => 450f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static BRIAR: CachedChampion = CachedChampion {
    name: "Briar",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 625f32,
            per_level: 95f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.669f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        briar_q_1, briar_w_1, briar_e_1, briar_e_2, briar_e_3, briar_e_4,
        briar_r_1,
    ],
};
pub const fn briar_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        2 => 90f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        3 => 120f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        4 => 150f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        5 => 180f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn briar_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.6f32 * ctx.ad,
        2 => 0.7f32 * ctx.ad,
        3 => 0.8f32 * ctx.ad,
        4 => 0.9f32 * ctx.ad,
        5 => ctx.ad,
        _ => 0.0,
    }
}
pub const fn briar_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 140f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,
        2 => 215f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,
        3 => 290f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,
        4 => 365f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,
        5 => 440f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn briar_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 220f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,
        2 => 330f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,
        3 => 440f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,
        4 => 550f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,
        5 => 660f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn briar_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + ctx.bonus_ad + ctx.ap,
        2 => 115f32 + ctx.bonus_ad + ctx.ap,
        3 => 150f32 + ctx.bonus_ad + ctx.ap,
        4 => 185f32 + ctx.bonus_ad + ctx.ap,
        5 => 220f32 + ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn briar_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 2f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,
        2 => 2.875f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,
        3 => 3.75f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,
        4 => 4.625f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,
        5 => 5.5f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn briar_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        2 => 250f32 + 0.5f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        3 => 350f32 + 0.5f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static CAITLYN: CachedChampion = CachedChampion {
    name: "Caitlyn",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 580f32,
            per_level: 107f32,
        },
        mana: CachedChampionStatsMap {
            flat: 315f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3.8f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.681f32,
            per_level: 4f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 650f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        caitlyn_q_1,
        caitlyn_q_2,
        caitlyn_w_1,
        caitlyn_e_1,
        caitlyn_r_1,
    ],
};
pub const fn caitlyn_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 50f32 + 1.25f32 * ctx.ad,
        2 => 90f32 + 1.45f32 * ctx.ad,
        3 => 130f32 + 1.65f32 * ctx.ad,
        4 => 170f32 + 1.85f32 * ctx.ad,
        5 => 210f32 + 2.05f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn caitlyn_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.75f32 * ctx.ad,
        2 => 54f32 + 0.87f32 * ctx.ad,
        3 => 78f32 + 0.99f32 * ctx.ad,
        4 => 102f32 + 1.11f32 * ctx.ad,
        5 => 126f32 + 1.23f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn caitlyn_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 35f32 + 0.3f32 * ctx.bonus_ad,
        2 => 80f32 + 0.3f32 * ctx.bonus_ad,
        3 => 125f32 + 0.3f32 * ctx.bonus_ad,
        4 => 170f32 + 0.3f32 * ctx.bonus_ad,
        5 => 215f32 + 0.3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn caitlyn_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 130f32 + 0.8f32 * ctx.ap,
        3 => 180f32 + 0.8f32 * ctx.ap,
        4 => 230f32 + 0.8f32 * ctx.ap,
        5 => 280f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn caitlyn_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + ctx.bonus_ad,
        2 => 475f32 + ctx.bonus_ad,
        3 => 650f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static CAMILLE: CachedChampion = CachedChampion {
    name: "Camille",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 339f32,
            per_level: 52f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 3.8f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 2.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        camille_q_1,
        camille_q_2,
        camille_w_1,
        camille_w_2,
        camille_w_3,
        camille_w_4,
        camille_r_1,
    ],
};
pub const fn camille_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.2f32 * ctx.ad,
        2 => 0.25f32 * ctx.ad,
        3 => 0.3f32 * ctx.ad,
        4 => 0.35f32 * ctx.ad,
        5 => 0.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn camille_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.4f32 * ctx.ad,
        2 => 0.5f32 * ctx.ad,
        3 => 0.6f32 * ctx.ad,
        4 => 0.7f32 * ctx.ad,
        5 => 0.8f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn camille_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.6f32 * ctx.bonus_ad,
        2 => 75f32 + 0.6f32 * ctx.bonus_ad,
        3 => 100f32 + 0.6f32 * ctx.bonus_ad,
        4 => 125f32 + 0.6f32 * ctx.bonus_ad,
        5 => 150f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn camille_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.06f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,
        2 => {
            0.065f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad
        }
        3 => 0.07f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,
        4 => {
            0.075f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad
        }
        5 => 0.08f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn camille_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            0.03f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad
        }
        2 => {
            0.0325f32 * ctx.enemy_max_health
                + 0.0125f32 * 0.01f32 * ctx.bonus_ad
        }
        3 => {
            0.035f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad
        }
        4 => {
            0.0375f32 * ctx.enemy_max_health
                + 0.0125f32 * 0.01f32 * ctx.bonus_ad
        }
        5 => {
            0.04f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad
        }
        _ => 0.0,
    }
}
pub const fn camille_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 25f32 + 0.3f32 * ctx.bonus_ad,
        2 => 37.5f32 + 0.3f32 * ctx.bonus_ad,
        3 => 50f32 + 0.3f32 * ctx.bonus_ad,
        4 => 62.5f32 + 0.3f32 * ctx.bonus_ad,
        5 => 75f32 + 0.3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn camille_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.04f32 * ctx.current_health,
        2 => 0.06f32 * ctx.current_health,
        3 => 0.08f32 * ctx.current_health,
        _ => 0.0,
    }
}
pub static CASSIOPEIA: CachedChampion = CachedChampion {
    name: "Cassiopeia",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 18f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 53f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.647f32,
            per_level: 1.5f32,
        },
        movespeed: 328f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.647000014781951f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        cassiopeia_q_1,
        cassiopeia_q_2,
        cassiopeia_w_1,
        cassiopeia_w_2,
        cassiopeia_e_1,
        cassiopeia_e_2,
        cassiopeia_r_1,
    ],
};
pub const fn cassiopeia_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10.71f32 + 0.0929f32 * ctx.ap,
        2 => 15.71f32 + 0.0929f32 * ctx.ap,
        3 => 20.71f32 + 0.0929f32 * ctx.ap,
        4 => 25.71f32 + 0.0929f32 * ctx.ap,
        5 => 30.71f32 + 0.0929f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn cassiopeia_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.65f32 * ctx.ap,
        2 => 110f32 + 0.65f32 * ctx.ap,
        3 => 145f32 + 0.65f32 * ctx.ap,
        4 => 180f32 + 0.65f32 * ctx.ap,
        5 => 215f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn cassiopeia_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.1f32 * ctx.ap,
        2 => 25f32 + 0.1f32 * ctx.ap,
        3 => 30f32 + 0.1f32 * ctx.ap,
        4 => 35f32 + 0.1f32 * ctx.ap,
        5 => 40f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn cassiopeia_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 100f32 + 0.5f32 * ctx.ap,
        2 => 125f32 + 0.5f32 * ctx.ap,
        3 => 150f32 + 0.5f32 * ctx.ap,
        4 => 175f32 + 0.5f32 * ctx.ap,
        5 => 200f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn cassiopeia_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.55f32 * ctx.ap,
        2 => 40f32 + 0.55f32 * ctx.ap,
        3 => 60f32 + 0.55f32 * ctx.ap,
        4 => 80f32 + 0.55f32 * ctx.ap,
        5 => 100f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn cassiopeia_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 52f32 + 20f32 + 0.65f32 * ctx.ap,
        2 => 56f32 + 40f32 + 0.65f32 * ctx.ap,
        3 => 60f32 + 60f32 + 0.65f32 * ctx.ap,
        4 => 64f32 + 80f32 + 0.65f32 * ctx.ap,
        5 => 68f32 + 100f32 + 0.65f32 * ctx.ap,
        6 => 72f32,
        7 => 76f32,
        8 => 80f32,
        9 => 84f32,
        10 => 88f32,
        11 => 92f32,
        12 => 96f32,
        13 => 100f32,
        14 => 104f32,
        15 => 108f32,
        16 => 112f32,
        17 => 116f32,
        18 => 120f32,
        _ => 0.0,
    }
}
pub const fn cassiopeia_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.ap,
        2 => 250f32 + 0.5f32 * ctx.ap,
        3 => 350f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static CHOGATH: CachedChampion = CachedChampion {
    name: "Cho'Gath",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 644f32,
            per_level: 94f32,
        },
        mana: CachedChampionStatsMap {
            flat: 270f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 69f32,
            per_level: 4.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 1.44f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        chogath_q_1,
        chogath_w_1,
        chogath_e_1,
        chogath_e_2,
        chogath_r_1,
        chogath_r_2,
    ],
};
pub const fn chogath_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + ctx.ap,
        2 => 135f32 + ctx.ap,
        3 => 190f32 + ctx.ap,
        4 => 245f32 + ctx.ap,
        5 => 300f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn chogath_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 130f32 + 0.7f32 * ctx.ap,
        3 => 180f32 + 0.7f32 * ctx.ap,
        4 => 230f32 + 0.7f32 * ctx.ap,
        5 => 280f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn chogath_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => {
            20f32
                + 0.3f32 * ctx.ap
                + 0.025f32 * ctx.enemy_max_health
                + 0.005f32 * ctx.chogath_stacks
        }
        2 => {
            40f32
                + 0.3f32 * ctx.ap
                + 0.0285f32 * ctx.enemy_max_health
                + 0.005f32 * ctx.chogath_stacks
        }
        3 => {
            60f32
                + 0.3f32 * ctx.ap
                + 0.032f32 * ctx.enemy_max_health
                + 0.005f32 * ctx.chogath_stacks
        }
        4 => {
            80f32
                + 0.3f32 * ctx.ap
                + 0.0355f32 * ctx.enemy_max_health
                + 0.005f32 * ctx.chogath_stacks
        }
        5 => {
            100f32
                + 0.3f32 * ctx.ap
                + 0.039f32 * ctx.enemy_max_health
                + 0.005f32 * ctx.chogath_stacks
        }
        _ => 0.0,
    }
}
pub const fn chogath_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => {
            60f32
                + 0.9f32 * ctx.ap
                + 0.075f32 * ctx.enemy_max_health
                + 0.015f32 * ctx.chogath_stacks
        }
        2 => {
            120f32
                + 0.9f32 * ctx.ap
                + 0.0855f32 * ctx.enemy_max_health
                + 0.015f32 * ctx.chogath_stacks
        }
        3 => {
            180f32
                + 0.9f32 * ctx.ap
                + 0.096f32 * ctx.enemy_max_health
                + 0.015f32 * ctx.chogath_stacks
        }
        4 => {
            240f32
                + 0.9f32 * ctx.ap
                + 0.1065f32 * ctx.enemy_max_health
                + 0.015f32 * ctx.chogath_stacks
        }
        5 => {
            300f32
                + 0.9f32 * ctx.ap
                + 0.11699999999999999f32 * ctx.enemy_max_health
                + 0.015f32 * ctx.chogath_stacks
        }
        _ => 0.0,
    }
}
pub const fn chogath_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,
        2 => 475f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,
        3 => 650f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn chogath_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 1200f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,
        2 => 1200f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,
        3 => 1200f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub static CORKI: CachedChampion = CachedChampion {
    name: "Corki",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 52f32,
            per_level: 2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 2.8f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.644f32,
        attack_range: 550f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        corki_q_1, corki_w_1, corki_w_2, corki_e_1, corki_e_2, corki_r_1,
        corki_r_2,
    ],
};
pub const fn corki_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 1.3f32 * ctx.bonus_ad + ctx.ap,
        2 => 115f32 + 1.3f32 * ctx.bonus_ad + ctx.ap,
        3 => 160f32 + 1.3f32 * ctx.bonus_ad + ctx.ap,
        4 => 205f32 + 1.3f32 * ctx.bonus_ad + ctx.ap,
        5 => 250f32 + 1.3f32 * ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn corki_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        2 => 55f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        3 => 70f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        4 => 85f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        5 => 100f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn corki_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 200f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        2 => 275f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        3 => 350f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        4 => 425f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        5 => 500f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn corki_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 5f32 + 0.15f32 * ctx.bonus_ad,
        2 => 8.125f32 + 0.15f32 * ctx.bonus_ad,
        3 => 11.25f32 + 0.15f32 * ctx.bonus_ad,
        4 => 14.375f32 + 0.15f32 * ctx.bonus_ad,
        5 => 17.5f32 + 0.15f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn corki_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 2.4f32 * ctx.bonus_ad,
        2 => 130f32 + 2.4f32 * ctx.bonus_ad,
        3 => 180f32 + 2.4f32 * ctx.bonus_ad,
        4 => 230f32 + 2.4f32 * ctx.bonus_ad,
        5 => 280f32 + 2.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn corki_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 90f32 + 0.85f32 * ctx.bonus_ad,
        2 => 170f32 + 0.85f32 * ctx.bonus_ad,
        3 => 250f32 + 0.85f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn corki_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 180f32 + 1.7f32 * ctx.bonus_ad,
        2 => 340f32 + 1.7f32 * ctx.bonus_ad,
        3 => 500f32 + 1.7f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static DARIUS: CachedChampion = CachedChampion {
    name: "Darius",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 652f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 263f32,
            per_level: 58f32,
        },
        armor: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[
        darius_q_1, darius_q_2, darius_w_1, darius_r_1, darius_r_2, darius_r_3,
    ],
};
pub const fn darius_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 50f32 + ctx.ad,
        2 => 80f32 + 1.1f32 * ctx.ad,
        3 => 110f32 + 1.2f32 * ctx.ad,
        4 => 140f32 + 1.3f32 * ctx.ad,
        5 => 170f32 + 1.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn darius_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 17.5f32 + 0.35f32 * ctx.ad,
        2 => 28f32 + 0.385f32 * ctx.ad,
        3 => 38.5f32 + 0.42f32 * ctx.ad,
        4 => 49f32 + 0.455f32 * ctx.ad,
        5 => 59.5f32 + 0.49f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn darius_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.4f32 * ctx.ad,
        2 => 0.45f32 * ctx.ad,
        3 => 0.5f32 * ctx.ad,
        4 => 0.55f32 * ctx.ad,
        5 => 0.6f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn darius_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 25f32 + 0.15f32 * ctx.bonus_ad,
        2 => 50f32 + 0.15f32 * ctx.bonus_ad,
        3 => 75f32 + 0.15f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn darius_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 250f32 + 1.5f32 * ctx.bonus_ad,
        2 => 500f32 + 1.5f32 * ctx.bonus_ad,
        3 => 750f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn darius_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.75f32 * ctx.bonus_ad,
        2 => 250f32 + 0.75f32 * ctx.bonus_ad,
        3 => 375f32 + 0.75f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static DIANA: CachedChampion = CachedChampion {
    name: "Diana",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 375f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 4.3f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 57f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694f32,
        attack_range: 150f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        diana_q_1, diana_w_1, diana_w_2, diana_e_1, diana_r_1, diana_r_2,
        diana_r_3,
    ],
};
pub const fn diana_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 105f32 + 0.7f32 * ctx.ap,
        3 => 140f32 + 0.7f32 * ctx.ap,
        4 => 175f32 + 0.7f32 * ctx.ap,
        5 => 210f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn diana_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.18f32 * ctx.ap,
        2 => 32f32 + 0.18f32 * ctx.ap,
        3 => 44f32 + 0.18f32 * ctx.ap,
        4 => 56f32 + 0.18f32 * ctx.ap,
        5 => 68f32 + 0.18f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn diana_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.54f32 * ctx.ap,
        2 => 96f32 + 0.54f32 * ctx.ap,
        3 => 132f32 + 0.54f32 * ctx.ap,
        4 => 168f32 + 0.54f32 * ctx.ap,
        5 => 204f32 + 0.54f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn diana_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.6f32 * ctx.ap,
        2 => 70f32 + 0.6f32 * ctx.ap,
        3 => 90f32 + 0.6f32 * ctx.ap,
        4 => 110f32 + 0.6f32 * ctx.ap,
        5 => 130f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn diana_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 35f32 + 0.15f32 * ctx.ap,
        2 => 60f32 + 0.15f32 * ctx.ap,
        3 => 85f32 + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn diana_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.6f32 * ctx.ap,
        2 => 300f32 + 0.6f32 * ctx.ap,
        3 => 400f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn diana_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 340f32 + 1.2f32 * ctx.ap,
        2 => 540f32 + 1.2f32 * ctx.ap,
        3 => 740f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static DRMUNDO: CachedChampion = CachedChampion {
    name: "Dr. Mundo",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 3.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 2.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.67f32,
            per_level: 3.3f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        drmundo_q_1,
        drmundo_q_2,
        drmundo_q_3,
        drmundo_w_1,
        drmundo_w_2,
        drmundo_w_3,
        drmundo_e_1,
        drmundo_e_2,
        drmundo_e_3,
    ],
};
pub const fn drmundo_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.2f32 * ctx.current_health,
        2 => 0.225f32 * ctx.current_health,
        3 => 0.25f32 * ctx.current_health,
        4 => 0.275f32 * ctx.current_health,
        5 => 0.3f32 * ctx.current_health,
        _ => 0.0,
    }
}
pub const fn drmundo_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 350f32,
        2 => 425f32,
        3 => 500f32,
        4 => 575f32,
        5 => 650f32,
        _ => 0.0,
    }
}
pub const fn drmundo_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32,
        2 => 130f32,
        3 => 180f32,
        4 => 230f32,
        5 => 280f32,
        _ => 0.0,
    }
}
pub const fn drmundo_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32,
        2 => 8.75f32,
        3 => 12.5f32,
        4 => 16.25f32,
        5 => 20f32,
        _ => 0.0,
    }
}
pub const fn drmundo_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32,
        2 => 140f32,
        3 => 200f32,
        4 => 260f32,
        5 => 320f32,
        _ => 0.0,
    }
}
pub const fn drmundo_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.07f32 * ctx.bonus_health,
        2 => 35f32 + 0.07f32 * ctx.bonus_health,
        3 => 50f32 + 0.07f32 * ctx.bonus_health,
        4 => 65f32 + 0.07f32 * ctx.bonus_health,
        5 => 80f32 + 0.07f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn drmundo_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 7f32 + 0.098f32 * ctx.bonus_health,
        2 => 21f32 + 0.098f32 * ctx.bonus_health,
        3 => 35f32 + 0.098f32 * ctx.bonus_health,
        4 => 49f32 + 0.098f32 * ctx.bonus_health,
        5 => 63f32 + 0.098f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn drmundo_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 5f32 + 0.07f32 * ctx.bonus_health,
        2 => 15f32 + 0.07f32 * ctx.bonus_health,
        3 => 25f32 + 0.07f32 * ctx.bonus_health,
        4 => 35f32 + 0.07f32 * ctx.bonus_health,
        5 => 45f32 + 0.07f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn drmundo_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.02f32 * ctx.max_health,
        2 => 0.022000000000000002f32 * ctx.max_health,
        3 => 0.024f32 * ctx.max_health,
        4 => 0.026000000000000002f32 * ctx.max_health,
        5 => 0.027999999999999997f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub static DRAVEN: CachedChampion = CachedChampion {
    name: "Draven",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 675f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 361f32,
            per_level: 39f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 3.6f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.679f32,
            per_level: 2.7f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.67900002002716f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        draven_q_1, draven_e_1, draven_r_1, draven_r_2, draven_r_3, draven_r_4,
    ],
};
pub const fn draven_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.75f32 * ctx.bonus_ad,
        2 => 45f32 + 0.85f32 * ctx.bonus_ad,
        3 => 50f32 + 0.95f32 * ctx.bonus_ad,
        4 => 55f32 + 1.05f32 * ctx.bonus_ad,
        5 => 60f32 + 1.15f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn draven_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32 + 0.5f32 * ctx.bonus_ad,
        2 => 110f32 + 0.5f32 * ctx.bonus_ad,
        3 => 145f32 + 0.5f32 * ctx.bonus_ad,
        4 => 180f32 + 0.5f32 * ctx.bonus_ad,
        5 => 215f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn draven_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 1.1f32 * ctx.bonus_ad,
        2 => 275f32 + 1.3f32 * ctx.bonus_ad,
        3 => 375f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn draven_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 350f32 + 2.2f32 * ctx.bonus_ad,
        2 => 550f32 + 2.6f32 * ctx.bonus_ad,
        3 => 750f32 + 3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn draven_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.44f32 * ctx.bonus_ad,
        2 => 110f32 + 0.52f32 * ctx.bonus_ad,
        3 => 150f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn draven_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 140f32 + 0.88f32 * ctx.bonus_ad,
        2 => 220f32 + 1.04f32 * ctx.bonus_ad,
        3 => 300f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static EKKO: CachedChampion = CachedChampion {
    name: "Ekko",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 70f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.688f32,
            per_level: 3.3f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[ekko_q_1, ekko_q_2, ekko_q_3, ekko_e_1, ekko_r_1],
};
pub const fn ekko_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.3f32 * ctx.ap,
        2 => 95f32 + 0.3f32 * ctx.ap,
        3 => 110f32 + 0.3f32 * ctx.ap,
        4 => 125f32 + 0.3f32 * ctx.ap,
        5 => 140f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ekko_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.6f32 * ctx.ap,
        2 => 65f32 + 0.6f32 * ctx.ap,
        3 => 90f32 + 0.6f32 * ctx.ap,
        4 => 115f32 + 0.6f32 * ctx.ap,
        5 => 140f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ekko_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 120f32 + 0.9f32 * ctx.ap,
        2 => 160f32 + 0.9f32 * ctx.ap,
        3 => 200f32 + 0.9f32 * ctx.ap,
        4 => 240f32 + 0.9f32 * ctx.ap,
        5 => 280f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ekko_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.4f32 * ctx.ap,
        2 => 75f32 + 0.4f32 * ctx.ap,
        3 => 100f32 + 0.4f32 * ctx.ap,
        4 => 125f32 + 0.4f32 * ctx.ap,
        5 => 150f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ekko_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.75f32 * ctx.ap,
        2 => 350f32 + 1.75f32 * ctx.ap,
        3 => 500f32 + 1.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ELISE: CachedChampion = CachedChampion {
    name: "Elise",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 324f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.75f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[elise_q_3, elise_q_4, elise_w_1],
};
pub const fn elise_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32,
        2 => 100f32,
        3 => 125f32,
        4 => 150f32,
        5 => 175f32,
        _ => 0.0,
    }
}
pub const fn elise_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.04f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        2 => 70f32 + 0.04f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        3 => 100f32 + 0.04f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        4 => 130f32 + 0.04f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        5 => 160f32 + 0.04f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn elise_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.75f32 * ctx.ap,
        2 => 100f32 + 0.75f32 * ctx.ap,
        3 => 140f32 + 0.75f32 * ctx.ap,
        4 => 180f32 + 0.75f32 * ctx.ap,
        5 => 220f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static EVELYNN: CachedChampion = CachedChampion {
    name: "Evelynn",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 642f32,
            per_level: 98f32,
        },
        mana: CachedChampionStatsMap {
            flat: 315f32,
            per_level: 42f32,
        },
        armor: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.667f32,
            per_level: 2.1f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.666999995708465f32,
        attack_range: 125f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[
        evelynn_q_1,
        evelynn_q_2,
        evelynn_q_3,
        evelynn_q_4,
        evelynn_q_5,
        evelynn_q_6,
        evelynn_w_1,
        evelynn_e_2,
        evelynn_r_1,
        evelynn_r_2,
    ],
};
pub const fn evelynn_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 25f32 + 0.25f32 * ctx.ap,
        2 => 30f32 + 0.25f32 * ctx.ap,
        3 => 35f32 + 0.25f32 * ctx.ap,
        4 => 40f32 + 0.25f32 * ctx.ap,
        5 => 45f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15f32 + 0.25f32 * ctx.ap,
        2 => 25f32 + 0.25f32 * ctx.ap,
        3 => 35f32 + 0.25f32 * ctx.ap,
        4 => 45f32 + 0.25f32 * ctx.ap,
        5 => 55f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.75f32 * ctx.ap,
        2 => 75f32 + 0.75f32 * ctx.ap,
        3 => 105f32 + 0.75f32 * ctx.ap,
        4 => 135f32 + 0.75f32 * ctx.ap,
        5 => 165f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 25f32 + 0.25f32 * ctx.ap,
        2 => 30f32 + 0.25f32 * ctx.ap,
        3 => 35f32 + 0.25f32 * ctx.ap,
        4 => 40f32 + 0.25f32 * ctx.ap,
        5 => 45f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.75f32 * ctx.ap,
        2 => 90f32 + 0.75f32 * ctx.ap,
        3 => 105f32 + 0.75f32 * ctx.ap,
        4 => 120f32 + 0.75f32 * ctx.ap,
        5 => 135f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 145f32 + 1.75f32 * ctx.ap,
        2 => 195f32 + 1.75f32 * ctx.ap,
        3 => 245f32 + 1.75f32 * ctx.ap,
        4 => 295f32 + 1.75f32 * ctx.ap,
        5 => 345f32 + 1.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 250f32 + 0.6f32 * ctx.ap,
        2 => 300f32 + 0.6f32 * ctx.ap,
        3 => 350f32 + 0.6f32 * ctx.ap,
        4 => 400f32 + 0.6f32 * ctx.ap,
        5 => 450f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => {
            60f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap
        }
        2 => {
            90f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap
        }
        3 => {
            120f32
                + 0.03f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        4 => {
            150f32
                + 0.03f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        5 => {
            180f32
                + 0.03f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn evelynn_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.75f32 * ctx.ap,
        2 => 250f32 + 0.75f32 * ctx.ap,
        3 => 375f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn evelynn_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 1.8f32 * ctx.ap,
        2 => 600f32 + 1.8f32 * ctx.ap,
        3 => 900f32 + 1.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static EZREAL: CachedChampion = CachedChampion {
    name: "Ezreal",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 102f32,
        },
        mana: CachedChampionStatsMap {
            flat: 375f32,
            per_level: 70f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.75f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.5f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[ezreal_q_1, ezreal_w_1, ezreal_e_1, ezreal_r_1, ezreal_r_2],
};
pub const fn ezreal_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,
        2 => 45f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,
        3 => 70f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,
        4 => 95f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,
        5 => 120f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ezreal_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,
        2 => 135f32 + ctx.bonus_ad + 0.75f32 * ctx.ap,
        3 => 190f32 + ctx.bonus_ad + 0.8f32 * ctx.ap,
        4 => 245f32 + ctx.bonus_ad + 0.85f32 * ctx.ap,
        5 => 300f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ezreal_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        2 => 130f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        3 => 180f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        4 => 230f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        5 => 280f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ezreal_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 350f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        2 => 550f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        3 => 750f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ezreal_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 0.5f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        2 => 275f32 + 0.5f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        3 => 375f32 + 0.5f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static FIDDLESTICKS: CachedChampion = CachedChampion {
    name: "Fiddlesticks",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 500f32,
            per_level: 28f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 2.65f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.11f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 480f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        fiddlesticks_q_1,
        fiddlesticks_q_2,
        fiddlesticks_q_3,
        fiddlesticks_q_4,
        fiddlesticks_w_1,
        fiddlesticks_w_2,
        fiddlesticks_w_3,
        fiddlesticks_w_4,
        fiddlesticks_e_1,
        fiddlesticks_r_1,
        fiddlesticks_r_2,
    ],
};
pub const fn fiddlesticks_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.04f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        2 => 0.045f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        3 => 0.05f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        4 => 0.055f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        5 => 0.06f32 * ctx.current_health + 0.03f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32,
        2 => 60f32,
        3 => 80f32,
        4 => 100f32,
        5 => 120f32,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.08f32 * ctx.current_health + 0.06f32 * 0.01f32 * ctx.ap,
        2 => 0.09f32 * ctx.current_health + 0.06f32 * 0.01f32 * ctx.ap,
        3 => 0.1f32 * ctx.current_health + 0.06f32 * 0.01f32 * ctx.ap,
        4 => 0.11f32 * ctx.current_health + 0.06f32 * 0.01f32 * ctx.ap,
        5 => 0.12f32 * ctx.current_health + 0.06f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32,
        2 => 120f32,
        3 => 160f32,
        4 => 200f32,
        5 => 240f32,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 15f32 + 0.1125f32 * ctx.ap,
        2 => 22.5f32 + 0.1125f32 * ctx.ap,
        3 => 30f32 + 0.1125f32 * ctx.ap,
        4 => 37.5f32 + 0.1125f32 * ctx.ap,
        5 => 45f32 + 0.1125f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.45f32 * ctx.ap,
        2 => 90f32 + 0.45f32 * ctx.ap,
        3 => 120f32 + 0.45f32 * ctx.ap,
        4 => 150f32 + 0.45f32 * ctx.ap,
        5 => 180f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 15f32 + 0.1125f32 * ctx.ap + 0.12f32 * ctx.missing_health,
        2 => 22.5f32 + 0.1125f32 * ctx.ap + 0.145f32 * ctx.missing_health,
        3 => 30f32 + 0.1125f32 * ctx.ap + 0.17f32 * ctx.missing_health,
        4 => 37.5f32 + 0.1125f32 * ctx.ap + 0.195f32 * ctx.missing_health,
        5 => 45f32 + 0.1125f32 * ctx.ap + 0.22f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 120f32 + 0.9f32 * ctx.ap + 0.12f32 * ctx.missing_health,
        2 => 180f32 + 0.9f32 * ctx.ap + 0.145f32 * ctx.missing_health,
        3 => 240f32 + 0.9f32 * ctx.ap + 0.17f32 * ctx.missing_health,
        4 => 300f32 + 0.9f32 * ctx.ap + 0.195f32 * ctx.missing_health,
        5 => 360f32 + 0.9f32 * ctx.ap + 0.22f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.5f32 * ctx.ap,
        2 => 105f32 + 0.5f32 * ctx.ap,
        3 => 140f32 + 0.5f32 * ctx.ap,
        4 => 175f32 + 0.5f32 * ctx.ap,
        5 => 210f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 37.5f32 + 0.125f32 * ctx.ap,
        2 => 62.5f32 + 0.125f32 * ctx.ap,
        3 => 87.5f32 + 0.125f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiddlesticks_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 750f32 + 2.5f32 * ctx.ap,
        2 => 1250f32 + 2.5f32 * ctx.ap,
        3 => 1750f32 + 2.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static FIORA: CachedChampion = CachedChampion {
    name: "Fiora",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.69f32,
            per_level: 3.2f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.689999997615814f32,
        attack_range: 150f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[fiora_q_1, fiora_w_1, fiora_e_1],
};
pub const fn fiora_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.9f32 * ctx.bonus_ad,
        2 => 80f32 + 0.95f32 * ctx.bonus_ad,
        3 => 90f32 + ctx.bonus_ad,
        4 => 100f32 + 1.05f32 * ctx.bonus_ad,
        5 => 110f32 + 1.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn fiora_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 110f32 + ctx.ap,
        2 => 150f32 + ctx.ap,
        3 => 190f32 + ctx.ap,
        4 => 230f32 + ctx.ap,
        5 => 270f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn fiora_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 1.6f32,
        2 => 1.7f32,
        3 => 1.8f32,
        4 => 1.9f32,
        5 => 2f32,
        _ => 0.0,
    }
}
pub static FIZZ: CachedChampion = CachedChampion {
    name: "Fizz",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 317f32,
            per_level: 52f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.1f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 175f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        fizz_q_1, fizz_w_1, fizz_w_2, fizz_w_3, fizz_w_4, fizz_e_1, fizz_r_1,
        fizz_r_2, fizz_r_3,
    ],
};
pub const fn fizz_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.55f32 * ctx.ap,
        2 => 25f32 + 0.55f32 * ctx.ap,
        3 => 40f32 + 0.55f32 * ctx.ap,
        4 => 55f32 + 0.55f32 * ctx.ap,
        5 => 70f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.45f32 * ctx.ap,
        2 => 70f32 + 0.45f32 * ctx.ap,
        3 => 90f32 + 0.45f32 * ctx.ap,
        4 => 110f32 + 0.45f32 * ctx.ap,
        5 => 130f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.35f32 * ctx.ap,
        2 => 15f32 + 0.35f32 * ctx.ap,
        3 => 20f32 + 0.35f32 * ctx.ap,
        4 => 25f32 + 0.35f32 * ctx.ap,
        5 => 30f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 3.33f32 + 0.05f32 * ctx.ap,
        2 => 5f32 + 0.05f32 * ctx.ap,
        3 => 6.67f32 + 0.05f32 * ctx.ap,
        4 => 8.33f32 + 0.05f32 * ctx.ap,
        5 => 10f32 + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.3f32 * ctx.ap,
        2 => 30f32 + 0.3f32 * ctx.ap,
        3 => 40f32 + 0.3f32 * ctx.ap,
        4 => 50f32 + 0.3f32 * ctx.ap,
        5 => 60f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.95f32 * ctx.ap,
        2 => 130f32 + 0.95f32 * ctx.ap,
        3 => 180f32 + 0.95f32 * ctx.ap,
        4 => 230f32 + 0.95f32 * ctx.ap,
        5 => 280f32 + 0.95f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 225f32 + ctx.ap,
        2 => 325f32 + ctx.ap,
        3 => 425f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 1.2f32 * ctx.ap,
        2 => 400f32 + 1.2f32 * ctx.ap,
        3 => 500f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn fizz_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.8f32 * ctx.ap,
        2 => 250f32 + 0.8f32 * ctx.ap,
        3 => 350f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static GALIO: CachedChampion = CachedChampion {
    name: "Galio",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 126f32,
        },
        mana: CachedChampionStatsMap {
            flat: 410f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 150f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        galio_q_1, galio_w_1, galio_w_2, galio_w_3, galio_w_4, galio_e_1,
        galio_e_2, galio_r_1,
    ],
};
pub const fn galio_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 105f32 + 0.7f32 * ctx.ap,
        3 => 140f32 + 0.7f32 * ctx.ap,
        4 => 175f32 + 0.7f32 * ctx.ap,
        5 => 210f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn galio_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            0.25f32
                + 0.04f32 * 0.01f32 * ctx.ap
                + 0.08f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.01f32 * 0.01f32 * ctx.bonus_health
        }
        2 => {
            0.3f32
                + 0.04f32 * 0.01f32 * ctx.ap
                + 0.08f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.01f32 * 0.01f32 * ctx.bonus_health
        }
        3 => {
            0.35f32
                + 0.04f32 * 0.01f32 * ctx.ap
                + 0.08f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.01f32 * 0.01f32 * ctx.bonus_health
        }
        4 => {
            0.4f32
                + 0.04f32 * 0.01f32 * ctx.ap
                + 0.08f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.01f32 * 0.01f32 * ctx.bonus_health
        }
        5 => {
            0.45f32
                + 0.04f32 * 0.01f32 * ctx.ap
                + 0.08f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.01f32 * 0.01f32 * ctx.bonus_health
        }
        _ => 0.0,
    }
}
pub const fn galio_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            0.125f32
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.04f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.005f32 * 0.01f32 * ctx.bonus_health
        }
        2 => {
            0.15f32
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.04f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.005f32 * 0.01f32 * ctx.bonus_health
        }
        3 => {
            0.175f32
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.04f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.005f32 * 0.01f32 * ctx.bonus_health
        }
        4 => {
            0.2f32
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.04f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.005f32 * 0.01f32 * ctx.bonus_health
        }
        5 => {
            0.225f32
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.04f32 * 0.01f32 * ctx.bonus_magic_resist
                + 0.005f32 * 0.01f32 * ctx.bonus_health
        }
        _ => 0.0,
    }
}
pub const fn galio_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.9f32 * ctx.ap,
        2 => 90f32 + 0.9f32 * ctx.ap,
        3 => 120f32 + 0.9f32 * ctx.ap,
        4 => 150f32 + 0.9f32 * ctx.ap,
        5 => 180f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn galio_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.3f32 * ctx.ap,
        2 => 30f32 + 0.3f32 * ctx.ap,
        3 => 40f32 + 0.3f32 * ctx.ap,
        4 => 50f32 + 0.3f32 * ctx.ap,
        5 => 60f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn galio_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 90f32 + 0.9f32 * ctx.ap,
        2 => 130f32 + 0.9f32 * ctx.ap,
        3 => 170f32 + 0.9f32 * ctx.ap,
        4 => 210f32 + 0.9f32 * ctx.ap,
        5 => 250f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn galio_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 72f32 + 0.72f32 * ctx.ap,
        2 => 104f32 + 0.72f32 * ctx.ap,
        3 => 136f32 + 0.72f32 * ctx.ap,
        4 => 168f32 + 0.72f32 * ctx.ap,
        5 => 200f32 + 0.72f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn galio_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.7f32 * ctx.ap,
        2 => 250f32 + 0.7f32 * ctx.ap,
        3 => 350f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static GANGPLANK: CachedChampion = CachedChampion {
    name: "Gangplank",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_7),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 4.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.2f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.69f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        gangplank_q_1,
        gangplank_e_1,
        gangplank_r_1,
        gangplank_r_2,
        gangplank_r_3,
        gangplank_r_4,
        gangplank_r_5,
        gangplank_r_6,
        gangplank_r_7,
    ],
};
pub const fn gangplank_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + ctx.ad,
        2 => 40f32 + ctx.ad,
        3 => 70f32 + ctx.ad,
        4 => 100f32 + ctx.ad,
        5 => 130f32 + ctx.ad,
        _ => 0.0,
    }
}
pub const fn gangplank_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32,
        2 => 105f32,
        3 => 135f32,
        4 => 165f32,
        5 => 195f32,
        _ => 0.0,
    }
}
pub const fn gangplank_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 120f32 + 0.3f32 * ctx.ap,
        2 => 210f32 + 0.3f32 * ctx.ap,
        3 => 300f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gangplank_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 40f32 + 0.1f32 * ctx.ap,
        2 => 70f32 + 0.1f32 * ctx.ap,
        3 => 100f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gangplank_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 480f32 + 1.2f32 * ctx.ap,
        2 => 840f32 + 1.2f32 * ctx.ap,
        3 => 1200f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gangplank_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 600f32 + 1.5f32 * ctx.ap,
        2 => 1050f32 + 1.5f32 * ctx.ap,
        3 => 1500f32 + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gangplank_r_5(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 120f32 + 0.3f32 * ctx.ap,
        2 => 210f32 + 0.3f32 * ctx.ap,
        3 => 300f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gangplank_r_6(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 840f32 + 2.1f32 * ctx.ap,
        2 => 1470f32 + 2.1f32 * ctx.ap,
        3 => 2100f32 + 2.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gangplank_r_7(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 720f32 + 1.8f32 * ctx.ap,
        2 => 1260f32 + 1.8f32 * ctx.ap,
        3 => 1800f32 + 1.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static GAREN: CachedChampion = CachedChampion {
    name: "Garen",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 690f32,
            per_level: 98f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 69f32,
            per_level: 4.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.65f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[garen_q_1, garen_w_1, garen_e_1, garen_e_2, garen_r_1],
};
pub const fn garen_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.5f32 * ctx.ad,
        2 => 60f32 + 0.5f32 * ctx.ad,
        3 => 90f32 + 0.5f32 * ctx.ad,
        4 => 120f32 + 0.5f32 * ctx.ad,
        5 => 150f32 + 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn garen_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.25f32,
        2 => 0.29f32,
        3 => 0.33f32,
        4 => 0.37f32,
        5 => 0.41f32,
        _ => 0.0,
    }
}
pub const fn garen_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 4f32 + 0.36f32 * ctx.ad,
        2 => 7f32 + 0.39f32 * ctx.ad,
        3 => 10f32 + 0.42f32 * ctx.ad,
        4 => 13f32 + 0.45f32 * ctx.ad,
        5 => 16f32 + 0.48f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn garen_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 5f32 + 0.45f32 * ctx.ad,
        2 => 8.75f32 + 0.4875f32 * ctx.ad,
        3 => 12.5f32 + 0.525f32 * ctx.ad,
        4 => 16.25f32 + 0.5625f32 * ctx.ad,
        5 => 20f32 + 0.6f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn garen_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.25f32 * ctx.missing_health,
        2 => 250f32 + 0.3f32 * ctx.missing_health,
        3 => 350f32 + 0.35f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub static GNAR: CachedChampion = CachedChampion {
    name: "Gnar",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 540f32,
            per_level: 79f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 3.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 6f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        gnar_q_2, gnar_q_3, gnar_w_1, gnar_e_1, gnar_e_2, gnar_r_1, gnar_r_2,
    ],
};
pub const fn gnar_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 2.5f32 + 0.625f32 * ctx.ad,
        2 => 22.5f32 + 0.625f32 * ctx.ad,
        3 => 42.5f32 + 0.625f32 * ctx.ad,
        4 => 62.5f32 + 0.625f32 * ctx.ad,
        5 => 82.5f32 + 0.625f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn gnar_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 1.25f32 * ctx.ad,
        2 => 45f32 + 1.25f32 * ctx.ad,
        3 => 85f32 + 1.25f32 * ctx.ad,
        4 => 125f32 + 1.25f32 * ctx.ad,
        5 => 165f32 + 1.25f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn gnar_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0f32 + 0.06f32 * ctx.enemy_max_health + ctx.ap,
        2 => 10f32 + 0.08f32 * ctx.enemy_max_health + ctx.ap,
        3 => 20f32 + 0.1f32 * ctx.enemy_max_health + ctx.ap,
        4 => 30f32 + 0.12f32 * ctx.enemy_max_health + ctx.ap,
        5 => 40f32 + 0.14f32 * ctx.enemy_max_health + ctx.ap,
        _ => 0.0,
    }
}
pub const fn gnar_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.06f32 * ctx.max_health,
        2 => 85f32 + 0.06f32 * ctx.max_health,
        3 => 120f32 + 0.06f32 * ctx.max_health,
        4 => 155f32 + 0.06f32 * ctx.max_health,
        5 => 190f32 + 0.06f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn gnar_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.4f32,
        2 => 0.45f32,
        3 => 0.5f32,
        4 => 0.55f32,
        5 => 0.6f32,
        _ => 0.0,
    }
}
pub const fn gnar_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 0.75f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        2 => 450f32 + 0.75f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        3 => 600f32 + 0.75f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gnar_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.5f32 * ctx.bonus_ad + ctx.ap,
        2 => 300f32 + 0.5f32 * ctx.bonus_ad + ctx.ap,
        3 => 400f32 + 0.5f32 * ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub static GRAGAS: CachedChampion = CachedChampion {
    name: "Gragas",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 115f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 47f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.675f32,
            per_level: 2.05f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        gragas_q_1, gragas_q_2, gragas_q_3, gragas_q_4, gragas_w_1, gragas_w_2,
        gragas_w_3, gragas_e_1, gragas_r_1,
    ],
};
pub const fn gragas_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 120f32 + 1.2f32 * ctx.ap,
        2 => 180f32 + 1.2f32 * ctx.ap,
        3 => 240f32 + 1.2f32 * ctx.ap,
        4 => 300f32 + 1.2f32 * ctx.ap,
        5 => 360f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 84f32 + 0.84f32 * ctx.ap,
        2 => 126f32 + 0.84f32 * ctx.ap,
        3 => 168f32 + 0.84f32 * ctx.ap,
        4 => 210f32 + 0.84f32 * ctx.ap,
        5 => 252f32 + 0.84f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 120f32 + 0.8f32 * ctx.ap,
        3 => 160f32 + 0.8f32 * ctx.ap,
        4 => 200f32 + 0.8f32 * ctx.ap,
        5 => 240f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 56f32 + 0.56f32 * ctx.ap,
        2 => 84f32 + 0.56f32 * ctx.ap,
        3 => 112f32 + 0.56f32 * ctx.ap,
        4 => 140f32 + 0.56f32 * ctx.ap,
        5 => 168f32 + 0.56f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.1f32 + 0.04f32 * 0.01f32 * ctx.ap,
        2 => 0.12f32 + 0.04f32 * 0.01f32 * ctx.ap,
        3 => 0.14f32 + 0.04f32 * 0.01f32 * ctx.ap,
        4 => 0.16f32 + 0.04f32 * 0.01f32 * ctx.ap,
        5 => 0.18f32 + 0.04f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,
        2 => 50f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,
        3 => 80f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,
        4 => 110f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,
        5 => 140f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 320f32 + 0.7f32 * ctx.ap,
        2 => 350f32 + 0.7f32 * ctx.ap,
        3 => 380f32 + 0.7f32 * ctx.ap,
        4 => 410f32 + 0.7f32 * ctx.ap,
        5 => 440f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.6f32 * ctx.ap,
        2 => 125f32 + 0.6f32 * ctx.ap,
        3 => 170f32 + 0.6f32 * ctx.ap,
        4 => 215f32 + 0.6f32 * ctx.ap,
        5 => 260f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gragas_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.8f32 * ctx.ap,
        2 => 300f32 + 0.8f32 * ctx.ap,
        3 => 400f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static GRAVES: CachedChampion = CachedChampion {
    name: "Graves",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 625f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 325f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.475f32,
            per_level: 3f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.490000009536743f32,
        attack_range: 425f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        graves_q_1, graves_q_2, graves_q_3, graves_w_1, graves_r_1, graves_r_2,
    ],
};
pub const fn graves_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.8f32 * ctx.bonus_ad,
        2 => 65f32 + 0.8f32 * ctx.bonus_ad,
        3 => 85f32 + 0.8f32 * ctx.bonus_ad,
        4 => 105f32 + 0.8f32 * ctx.bonus_ad,
        5 => 125f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn graves_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 85f32 + 0.4f32 * ctx.bonus_ad,
        2 => 120f32 + 0.65f32 * ctx.bonus_ad,
        3 => 155f32 + 0.9f32 * ctx.bonus_ad,
        4 => 190f32 + 1.15f32 * ctx.bonus_ad,
        5 => 225f32 + 1.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn graves_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 130f32 + 1.2f32 * ctx.bonus_ad,
        2 => 185f32 + 1.45f32 * ctx.bonus_ad,
        3 => 240f32 + 1.7f32 * ctx.bonus_ad,
        4 => 295f32 + 1.95f32 * ctx.bonus_ad,
        5 => 350f32 + 2.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn graves_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 110f32 + 0.6f32 * ctx.ap,
        3 => 160f32 + 0.6f32 * ctx.ap,
        4 => 210f32 + 0.6f32 * ctx.ap,
        5 => 260f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn graves_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 275f32 + 1.5f32 * ctx.bonus_ad,
        2 => 425f32 + 1.5f32 * ctx.bonus_ad,
        3 => 575f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn graves_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.2f32 * ctx.bonus_ad,
        2 => 320f32 + 1.2f32 * ctx.bonus_ad,
        3 => 440f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static GWEN: CachedChampion = CachedChampion {
    name: "Gwen",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_7),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_8),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 330f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.9f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.69f32,
            per_level: 2.25f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.69f32,
        attack_range: 150f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.02f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        gwen_q_1, gwen_q_2, gwen_q_3, gwen_q_4, gwen_q_5, gwen_q_6, gwen_q_7,
        gwen_q_8, gwen_e_1, gwen_r_1, gwen_r_2, gwen_r_3, gwen_r_4, gwen_r_5,
    ],
};
pub const fn gwen_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            10f32
                + 0.02f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        2 => {
            15f32
                + 0.02f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        3 => {
            20f32
                + 0.02f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        4 => {
            25f32
                + 0.02f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        5 => {
            30f32
                + 0.02f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.02f32 * ctx.ap,
        2 => 15f32 + 0.02f32 * ctx.ap,
        3 => 20f32 + 0.02f32 * ctx.ap,
        4 => 25f32 + 0.02f32 * ctx.ap,
        5 => 30f32 + 0.02f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gwen_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            60f32
                + 0.35f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        2 => {
            85f32
                + 0.35f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        3 => {
            110f32
                + 0.35f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        4 => {
            135f32
                + 0.35f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        5 => {
            160f32
                + 0.35f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.35f32 * ctx.ap,
        2 => 85f32 + 0.35f32 * ctx.ap,
        3 => 110f32 + 0.35f32 * ctx.ap,
        4 => 135f32 + 0.35f32 * ctx.ap,
        5 => 160f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gwen_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            110f32
                + 0.45f32 * ctx.ap
                + 0.06f32 * ctx.enemy_max_health
                + 0.033f32 * 0.01f32 * ctx.ap
        }
        2 => {
            160f32
                + 0.45f32 * ctx.ap
                + 0.06f32 * ctx.enemy_max_health
                + 0.033f32 * 0.01f32 * ctx.ap
        }
        3 => {
            210f32
                + 0.45f32 * ctx.ap
                + 0.06f32 * ctx.enemy_max_health
                + 0.033f32 * 0.01f32 * ctx.ap
        }
        4 => {
            260f32
                + 0.45f32 * ctx.ap
                + 0.06f32 * ctx.enemy_max_health
                + 0.033f32 * 0.01f32 * ctx.ap
        }
        5 => {
            310f32
                + 0.45f32 * ctx.ap
                + 0.06f32 * ctx.enemy_max_health
                + 0.033f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 110f32 + 0.45f32 * ctx.ap,
        2 => 160f32 + 0.45f32 * ctx.ap,
        3 => 210f32 + 0.45f32 * ctx.ap,
        4 => 260f32 + 0.45f32 * ctx.ap,
        5 => 310f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gwen_q_7(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            70f32
                + 0.37f32 * ctx.ap
                + 0.02f32 * ctx.enemy_max_health
                + 0.011000000000000001f32 * 0.01f32 * ctx.ap
        }
        2 => {
            100f32
                + 0.37f32 * ctx.ap
                + 0.02f32 * ctx.enemy_max_health
                + 0.011000000000000001f32 * 0.01f32 * ctx.ap
        }
        3 => {
            130f32
                + 0.37f32 * ctx.ap
                + 0.02f32 * ctx.enemy_max_health
                + 0.011000000000000001f32 * 0.01f32 * ctx.ap
        }
        4 => {
            160f32
                + 0.37f32 * ctx.ap
                + 0.02f32 * ctx.enemy_max_health
                + 0.011000000000000001f32 * 0.01f32 * ctx.ap
        }
        5 => {
            190f32
                + 0.37f32 * ctx.ap
                + 0.02f32 * ctx.enemy_max_health
                + 0.011000000000000001f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_q_8(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.37f32 * ctx.ap,
        2 => 100f32 + 0.37f32 * ctx.ap,
        3 => 130f32 + 0.37f32 * ctx.ap,
        4 => 160f32 + 0.37f32 * ctx.ap,
        5 => 190f32 + 0.37f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gwen_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 8f32 + 0.25f32 * ctx.ap,
        2 => 11f32 + 0.25f32 * ctx.ap,
        3 => 14f32 + 0.25f32 * ctx.ap,
        4 => 17f32 + 0.25f32 * ctx.ap,
        5 => 20f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gwen_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            30f32
                + 0.08f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        2 => {
            60f32
                + 0.08f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        3 => {
            90f32
                + 0.08f32 * ctx.ap
                + 0.01f32 * ctx.enemy_max_health
                + 0.0055000000000000005f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32 + 0.08f32 * ctx.ap,
        2 => 60f32 + 0.08f32 * ctx.ap,
        3 => 90f32 + 0.08f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn gwen_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            270f32
                + 0.72f32 * ctx.ap
                + 0.09f32 * ctx.enemy_max_health
                + 0.0495f32 * 0.01f32 * ctx.ap
        }
        2 => {
            540f32
                + 0.72f32 * ctx.ap
                + 0.09f32 * ctx.enemy_max_health
                + 0.0495f32 * 0.01f32 * ctx.ap
        }
        3 => {
            810f32
                + 0.72f32 * ctx.ap
                + 0.09f32 * ctx.enemy_max_health
                + 0.0495f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            90f32
                + 0.24f32 * ctx.ap
                + 0.03f32 * ctx.enemy_max_health
                + 0.0165f32 * 0.01f32 * ctx.ap
        }
        2 => {
            180f32
                + 0.24f32 * ctx.ap
                + 0.03f32 * ctx.enemy_max_health
                + 0.0165f32 * 0.01f32 * ctx.ap
        }
        3 => {
            270f32
                + 0.24f32 * ctx.ap
                + 0.03f32 * ctx.enemy_max_health
                + 0.0165f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn gwen_r_5(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            150f32
                + 0.4f32 * ctx.ap
                + 0.05f32 * ctx.enemy_max_health
                + 0.0275f32 * 0.01f32 * ctx.ap
        }
        2 => {
            300f32
                + 0.4f32 * ctx.ap
                + 0.05f32 * ctx.enemy_max_health
                + 0.0275f32 * 0.01f32 * ctx.ap
        }
        3 => {
            450f32
                + 0.4f32 * ctx.ap
                + 0.05f32 * ctx.enemy_max_health
                + 0.0275f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub static HECARIM: CachedChampion = CachedChampion {
    name: "Hecarim",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 625f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 5.45f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 3.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.67f32,
            per_level: 2.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.6700000166893f32,
        attack_range: 175f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        hecarim_q_1,
        hecarim_q_2,
        hecarim_w_1,
        hecarim_w_2,
        hecarim_e_1,
        hecarim_e_2,
        hecarim_r_1,
    ],
};
pub const fn hecarim_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 36f32 + 0.54f32 * ctx.bonus_ad,
        2 => 51f32 + 0.54f32 * ctx.bonus_ad,
        3 => 66f32 + 0.54f32 * ctx.bonus_ad,
        4 => 81f32 + 0.54f32 * ctx.bonus_ad,
        5 => 96f32 + 0.54f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn hecarim_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.9f32 * ctx.bonus_ad,
        2 => 85f32 + 0.9f32 * ctx.bonus_ad,
        3 => 110f32 + 0.9f32 * ctx.bonus_ad,
        4 => 135f32 + 0.9f32 * ctx.bonus_ad,
        5 => 160f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn hecarim_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.2f32 * ctx.ap,
        2 => 30f32 + 0.2f32 * ctx.ap,
        3 => 40f32 + 0.2f32 * ctx.ap,
        4 => 50f32 + 0.2f32 * ctx.ap,
        5 => 60f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn hecarim_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 100f32 + ctx.ap,
        2 => 150f32 + ctx.ap,
        3 => 200f32 + ctx.ap,
        4 => 250f32 + ctx.ap,
        5 => 300f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn hecarim_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + ctx.bonus_ad,
        2 => 90f32 + ctx.bonus_ad,
        3 => 120f32 + ctx.bonus_ad,
        4 => 150f32 + ctx.bonus_ad,
        5 => 180f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn hecarim_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 30f32 + 0.5f32 * ctx.bonus_ad,
        2 => 45f32 + 0.5f32 * ctx.bonus_ad,
        3 => 60f32 + 0.5f32 * ctx.bonus_ad,
        4 => 75f32 + 0.5f32 * ctx.bonus_ad,
        5 => 90f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn hecarim_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + ctx.ap,
        2 => 250f32 + ctx.ap,
        3 => 350f32 + ctx.ap,
        _ => 0.0,
    }
}
pub static HEIMERDINGER: CachedChampion = CachedChampion {
    name: "Heimerdinger",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_7),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_8),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 558f32,
            per_level: 101f32,
        },
        mana: CachedChampionStatsMap {
            flat: 385f32,
            per_level: 20f32,
        },
        armor: CachedChampionStatsMap {
            flat: 19f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 2.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 1.36f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        heimerdinger_w_2,
        heimerdinger_w_3,
        heimerdinger_w_4,
        heimerdinger_w_5,
        heimerdinger_w_6,
        heimerdinger_w_7,
        heimerdinger_w_8,
        heimerdinger_e_1,
    ],
};
pub const fn heimerdinger_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 160f32 + 1.99f32 * ctx.ap,
        2 => 245f32,
        3 => 330f32,
        4 => 415f32,
        5 => 500f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 1.03f32 * ctx.ap,
        2 => 125f32,
        3 => 170f32,
        4 => 215f32,
        5 => 260f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.12f32 * ctx.ap,
        2 => 15f32,
        3 => 20f32,
        4 => 25f32,
        5 => 30f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_w_5(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.36f32 * ctx.ap,
        2 => 45f32,
        3 => 60f32,
        4 => 75f32,
        5 => 90f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_w_6(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 120f32 + 1.44f32 * ctx.ap,
        2 => 180f32,
        3 => 240f32,
        4 => 300f32,
        5 => 360f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_w_7(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.48f32 * ctx.ap,
        2 => 60f32,
        3 => 80f32,
        4 => 100f32,
        5 => 120f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_w_8(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.55f32 * ctx.ap,
        2 => 65f32,
        3 => 90f32,
        4 => 115f32,
        5 => 140f32,
        _ => 0.0,
    }
}
pub const fn heimerdinger_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 100f32,
        3 => 140f32,
        4 => 180f32,
        5 => 220f32,
        _ => 0.0,
    }
}
pub static HWEI: CachedChampion = CachedChampion {
    name: "Hwei",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 580f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 480f32,
            per_level: 30f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.69f32,
            per_level: 2.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[hwei_r_1, hwei_r_2, hwei_r_3, hwei_r_4],
};
pub const fn hwei_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 2.5f32 + 0.0125f32 * ctx.ap,
        2 => 5f32 + 0.0125f32 * ctx.ap,
        3 => 7.5f32 + 0.0125f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn hwei_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32 + 0.15f32 * ctx.ap,
        2 => 60f32 + 0.15f32 * ctx.ap,
        3 => 90f32 + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn hwei_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.8f32 * ctx.ap,
        2 => 325f32 + 0.8f32 * ctx.ap,
        3 => 450f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn hwei_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 230f32 + 0.95f32 * ctx.ap,
        2 => 385f32 + 0.95f32 * ctx.ap,
        3 => 540f32 + 0.95f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ILLAOI: CachedChampion = CachedChampion {
    name: "Illaoi",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Mixed,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 656f32,
            per_level: 115f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 65f32,
            per_level: 5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.5f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[illaoi_q_1, illaoi_w_1, illaoi_w_2, illaoi_e_1, illaoi_r_1],
};
pub const fn illaoi_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.1f32,
        2 => 0.15f32,
        3 => 0.2f32,
        4 => 0.25f32,
        5 => 0.3f32,
        _ => 0.0,
    }
}
pub const fn illaoi_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.03f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.ad,
        2 => 0.035f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.ad,
        3 => 0.04f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.ad,
        4 => 0.045f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.ad,
        5 => 0.05f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn illaoi_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32,
        2 => 30f32,
        3 => 40f32,
        4 => 50f32,
        5 => 60f32,
        _ => 0.0,
    }
}
pub const fn illaoi_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.25f32 + 0.08f32 * 0.01f32 * ctx.ad,
        2 => 0.3f32 + 0.08f32 * 0.01f32 * ctx.ad,
        3 => 0.35f32 + 0.08f32 * 0.01f32 * ctx.ad,
        4 => 0.4f32 + 0.08f32 * 0.01f32 * ctx.ad,
        5 => 0.45f32 + 0.08f32 * 0.01f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn illaoi_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.bonus_ad,
        2 => 250f32 + 0.5f32 * ctx.bonus_ad,
        3 => 350f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static IRELIA: CachedChampion = CachedChampion {
    name: "Irelia",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 115f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 65f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.656f32,
            per_level: 2.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.656000018119812f32,
        attack_range: 200f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[irelia_q_1, irelia_w_1, irelia_w_2, irelia_e_1, irelia_r_1],
};
pub const fn irelia_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.7f32 * ctx.ad,
        2 => 25f32 + 0.7f32 * ctx.ad,
        3 => 45f32 + 0.7f32 * ctx.ad,
        4 => 65f32 + 0.7f32 * ctx.ad,
        5 => 85f32 + 0.7f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn irelia_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 1.2f32 * ctx.ad + 1.2f32 * ctx.ap,
        2 => 60f32 + 1.2f32 * ctx.ad + 1.2f32 * ctx.ap,
        3 => 90f32 + 1.2f32 * ctx.ad + 1.2f32 * ctx.ap,
        4 => 120f32 + 1.2f32 * ctx.ad + 1.2f32 * ctx.ap,
        5 => 150f32 + 1.2f32 * ctx.ad + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn irelia_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.4f32 * ctx.ad + 0.4f32 * ctx.ap,
        2 => 20f32 + 0.4f32 * ctx.ad + 0.4f32 * ctx.ap,
        3 => 30f32 + 0.4f32 * ctx.ad + 0.4f32 * ctx.ap,
        4 => 40f32 + 0.4f32 * ctx.ad + 0.4f32 * ctx.ap,
        5 => 50f32 + 0.4f32 * ctx.ad + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn irelia_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.8f32 * ctx.ap,
        2 => 110f32 + 0.8f32 * ctx.ap,
        3 => 150f32 + 0.8f32 * ctx.ap,
        4 => 190f32 + 0.8f32 * ctx.ap,
        5 => 230f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn irelia_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.7f32 * ctx.ap,
        2 => 200f32 + 0.7f32 * ctx.ap,
        3 => 275f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static IVERN: CachedChampion = CachedChampion {
    name: "Ivern",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 450f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 50f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 3.4f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 475f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[ivern_q_1, ivern_w_1, ivern_w_2, ivern_e_1],
};
pub const fn ivern_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 125f32 + 0.7f32 * ctx.ap,
        3 => 170f32 + 0.7f32 * ctx.ap,
        4 => 215f32 + 0.7f32 * ctx.ap,
        5 => 260f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ivern_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.2f32 * ctx.ap,
        2 => 27.5f32 + 0.2f32 * ctx.ap,
        3 => 35f32 + 0.2f32 * ctx.ap,
        4 => 42.5f32 + 0.2f32 * ctx.ap,
        5 => 50f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ivern_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.1f32 * ctx.ap,
        2 => 15f32 + 0.1f32 * ctx.ap,
        3 => 20f32 + 0.1f32 * ctx.ap,
        4 => 25f32 + 0.1f32 * ctx.ap,
        5 => 30f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ivern_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.8f32 * ctx.ap,
        2 => 90f32 + 0.8f32 * ctx.ap,
        3 => 110f32 + 0.8f32 * ctx.ap,
        4 => 130f32 + 0.8f32 * ctx.ap,
        5 => 150f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static JANNA: CachedChampion = CachedChampion {
    name: "Janna",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 570f32,
            per_level: 90f32,
        },
        mana: CachedChampionStatsMap {
            flat: 360f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 47f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[janna_q_1, janna_q_2, janna_q_3, janna_w_1, janna_e_1],
};
pub const fn janna_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.1f32 * ctx.ap,
        2 => 15f32 + 0.1f32 * ctx.ap,
        3 => 20f32 + 0.1f32 * ctx.ap,
        4 => 25f32 + 0.1f32 * ctx.ap,
        5 => 30f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn janna_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 85f32 + 0.8f32 * ctx.ap,
        2 => 135f32 + 0.8f32 * ctx.ap,
        3 => 185f32 + 0.8f32 * ctx.ap,
        4 => 235f32 + 0.8f32 * ctx.ap,
        5 => 285f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn janna_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 55f32 + 0.5f32 * ctx.ap,
        2 => 90f32 + 0.5f32 * ctx.ap,
        3 => 125f32 + 0.5f32 * ctx.ap,
        4 => 160f32 + 0.5f32 * ctx.ap,
        5 => 195f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn janna_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 55f32 + 0.5f32 * ctx.ap + 0.3f32 * ctx.bonus_move_speed,
        2 => 85f32 + 0.5f32 * ctx.ap + 0.3f32 * ctx.bonus_move_speed,
        3 => 115f32 + 0.5f32 * ctx.ap + 0.3f32 * ctx.bonus_move_speed,
        4 => 145f32 + 0.5f32 * ctx.ap + 0.3f32 * ctx.bonus_move_speed,
        5 => 175f32 + 0.5f32 * ctx.ap + 0.3f32 * ctx.bonus_move_speed,
        _ => 0.0,
    }
}
pub const fn janna_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 10f32 + 0.1f32 * ctx.ap,
        2 => 15f32 + 0.1f32 * ctx.ap,
        3 => 20f32 + 0.1f32 * ctx.ap,
        4 => 25f32 + 0.1f32 * ctx.ap,
        5 => 30f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static JARVANIV: CachedChampion = CachedChampion {
    name: "Jarvan IV",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 55f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[jarvaniv_q_1, jarvaniv_e_1, jarvaniv_r_1],
};
pub const fn jarvaniv_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.45f32 * ctx.bonus_ad,
        2 => 130f32 + 1.45f32 * ctx.bonus_ad,
        3 => 170f32 + 1.45f32 * ctx.bonus_ad,
        4 => 210f32 + 1.45f32 * ctx.bonus_ad,
        5 => 250f32 + 1.45f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn jarvaniv_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 120f32 + 0.8f32 * ctx.ap,
        3 => 160f32 + 0.8f32 * ctx.ap,
        4 => 200f32 + 0.8f32 * ctx.ap,
        5 => 240f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jarvaniv_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.8f32 * ctx.bonus_ad,
        2 => 325f32 + 1.8f32 * ctx.bonus_ad,
        3 => 450f32 + 1.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static JAX: CachedChampion = CachedChampion {
    name: "Jax",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 665f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 339f32,
            per_level: 52f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 4.25f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 3.4f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 125f32,
        aram_damage_taken: 0.97f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.15f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[jax_q_1, jax_w_1, jax_e_1, jax_e_2, jax_r_1, jax_r_2],
};
pub const fn jax_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + ctx.bonus_ad,
        2 => 105f32 + ctx.bonus_ad,
        3 => 145f32 + ctx.bonus_ad,
        4 => 185f32 + ctx.bonus_ad,
        5 => 225f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn jax_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.6f32 * ctx.ap,
        2 => 85f32 + 0.6f32 * ctx.ap,
        3 => 120f32 + 0.6f32 * ctx.ap,
        4 => 155f32 + 0.6f32 * ctx.ap,
        5 => 190f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jax_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,
        2 => 140f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,
        3 => 200f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,
        4 => 260f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,
        5 => 320f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn jax_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 40f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,
        2 => 70f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,
        3 => 100f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,
        4 => 130f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,
        5 => 160f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn jax_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + ctx.ap,
        2 => 175f32 + ctx.ap,
        3 => 250f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn jax_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 75f32 + 0.6f32 * ctx.ap,
        2 => 130f32 + 0.6f32 * ctx.ap,
        3 => 185f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static JAYCE: CachedChampion = CachedChampion {
    name: "Jayce",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 375f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 4.25f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.92f32,
    },
    merge_data: &[],
    closures: &[jayce_q_2, jayce_w_2, jayce_w_3, jayce_e_1, jayce_e_2],
};
pub const fn jayce_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 1.35f32 * ctx.bonus_ad,
        2 => 105f32 + 1.35f32 * ctx.bonus_ad,
        3 => 150f32 + 1.35f32 * ctx.bonus_ad,
        4 => 195f32 + 1.35f32 * ctx.bonus_ad,
        5 => 240f32 + 1.35f32 * ctx.bonus_ad,
        6 => 285f32 + 1.35f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn jayce_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 140f32 + ctx.ap,
        2 => 200f32 + ctx.ap,
        3 => 260f32 + ctx.ap,
        4 => 320f32 + ctx.ap,
        5 => 380f32 + ctx.ap,
        6 => 440f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn jayce_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 35f32 + 0.25f32 * ctx.ap,
        2 => 50f32 + 0.25f32 * ctx.ap,
        3 => 65f32 + 0.25f32 * ctx.ap,
        4 => 80f32 + 0.25f32 * ctx.ap,
        5 => 95f32 + 0.25f32 * ctx.ap,
        6 => 110f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jayce_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.08f32 * ctx.enemy_max_health + ctx.bonus_ad,
        2 => 0.10800000000000001f32 * ctx.enemy_max_health + ctx.bonus_ad,
        3 => 0.136f32 * ctx.enemy_max_health + ctx.bonus_ad,
        4 => 0.16399999999999998f32 * ctx.enemy_max_health + ctx.bonus_ad,
        5 => 0.192f32 * ctx.enemy_max_health + ctx.bonus_ad,
        6 => 0.22f32 * ctx.enemy_max_health + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn jayce_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 200f32,
        2 => 300f32,
        3 => 400f32,
        4 => 500f32,
        5 => 600f32,
        6 => 700f32,
        _ => 0.0,
    }
}
pub static JHIN: CachedChampion = CachedChampion {
    name: "Jhin",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 107f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 4.4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 0.86f32,
        attack_speed_ratio: 0f32,
        attack_range: 550f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.01f32,
    },
    merge_data: &[],
    closures: &[
        jhin_q_1, jhin_q_2, jhin_q_3, jhin_w_1, jhin_w_2, jhin_e_1, jhin_e_2,
        jhin_r_1, jhin_r_2, jhin_r_3, jhin_r_4,
    ],
};
pub const fn jhin_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 44f32 + 0.44f32 * ctx.ad + 0.6f32 * ctx.ap,
        2 => 69f32 + 0.515f32 * ctx.ad + 0.6f32 * ctx.ap,
        3 => 94f32 + 0.59f32 * ctx.ad + 0.6f32 * ctx.ap,
        4 => 119f32 + 0.665f32 * ctx.ad + 0.6f32 * ctx.ap,
        5 => 144f32 + 0.74f32 * ctx.ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jhin_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15.4f32 + 0.154f32 * ctx.ad + 0.21f32 * ctx.ap,
        2 => 24.15f32 + 0.18025f32 * ctx.ad + 0.21f32 * ctx.ap,
        3 => 32.9f32 + 0.2065f32 * ctx.ad + 0.21f32 * ctx.ap,
        4 => 41.65f32 + 0.23274999999999998f32 * ctx.ad + 0.21f32 * ctx.ap,
        5 => 50.4f32 + 0.259f32 * ctx.ad + 0.21f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jhin_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90.2f32 + 0.902f32 * ctx.ad + 1.23f32 * ctx.ap,
        2 => 141.45f32 + 1.05575f32 * ctx.ad + 1.23f32 * ctx.ap,
        3 => 192.7f32 + 1.2095f32 * ctx.ad + 1.23f32 * ctx.ap,
        4 => 243.95f32 + 1.3632499999999999f32 * ctx.ad + 1.23f32 * ctx.ap,
        5 => 295.2f32 + 1.517f32 * ctx.ad + 1.23f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jhin_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 45f32 + 0.375f32 * ctx.ad,
        2 => 71.25f32 + 0.375f32 * ctx.ad,
        3 => 97.5f32 + 0.375f32 * ctx.ad,
        4 => 123.75f32 + 0.375f32 * ctx.ad,
        5 => 150f32 + 0.375f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn jhin_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.5f32 * ctx.ad,
        2 => 95f32 + 0.5f32 * ctx.ad,
        3 => 130f32 + 0.5f32 * ctx.ad,
        4 => 165f32 + 0.5f32 * ctx.ad,
        5 => 200f32 + 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn jhin_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 1.2f32 * ctx.ad + ctx.ap,
        2 => 80f32 + 1.2f32 * ctx.ad + ctx.ap,
        3 => 140f32 + 1.2f32 * ctx.ad + ctx.ap,
        4 => 200f32 + 1.2f32 * ctx.ad + ctx.ap,
        5 => 260f32 + 1.2f32 * ctx.ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn jhin_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 13f32 + 0.78f32 * ctx.ad + 0.65f32 * ctx.ap,
        2 => 52f32 + 0.78f32 * ctx.ad + 0.65f32 * ctx.ap,
        3 => 91f32 + 0.78f32 * ctx.ad + 0.65f32 * ctx.ap,
        4 => 130f32 + 0.78f32 * ctx.ad + 0.65f32 * ctx.ap,
        5 => 169f32 + 0.78f32 * ctx.ad + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn jhin_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 256f32 + ctx.ad,
        2 => 512f32 + ctx.ad,
        3 => 768f32 + ctx.ad,
        _ => 0.0,
    }
}
pub const fn jhin_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 64f32 + 0.25f32 * ctx.ad,
        2 => 128f32 + 0.25f32 * ctx.ad,
        3 => 192f32 + 0.25f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn jhin_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 512f32 + 2f32 * ctx.ad,
        2 => 1024f32 + 2f32 * ctx.ad,
        3 => 1536f32 + 2f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn jhin_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 128f32 + 0.5f32 * ctx.ad,
        2 => 256f32 + 0.5f32 * ctx.ad,
        3 => 384f32 + 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static JINX: CachedChampion = CachedChampion {
    name: "Jinx",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 260f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 3.25f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.4f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[jinx_w_1, jinx_e_1, jinx_r_1, jinx_r_2, jinx_r_3, jinx_r_4],
};
pub const fn jinx_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 1.4f32 * ctx.ad,
        2 => 60f32 + 1.4f32 * ctx.ad,
        3 => 110f32 + 1.4f32 * ctx.ad,
        4 => 160f32 + 1.4f32 * ctx.ad,
        5 => 210f32 + 1.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn jinx_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + ctx.ap,
        2 => 120f32 + ctx.ap,
        3 => 170f32 + ctx.ap,
        4 => 220f32 + ctx.ap,
        5 => 270f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn jinx_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 240f32 + 1.24f32 * ctx.bonus_ad + 0.2f32 * ctx.missing_health,
        2 => 360f32 + 1.24f32 * ctx.bonus_ad + 0.24f32 * ctx.missing_health,
        3 => 480f32 + 1.24f32 * ctx.bonus_ad + 0.28f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn jinx_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 24f32 + 0.124f32 * ctx.bonus_ad + 0.2f32 * ctx.missing_health,
        2 => 36f32 + 0.124f32 * ctx.bonus_ad + 0.24f32 * ctx.missing_health,
        3 => 48f32 + 0.124f32 * ctx.bonus_ad + 0.28f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn jinx_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 1.55f32 * ctx.bonus_ad + 0.25f32 * ctx.missing_health,
        2 => 450f32 + 1.55f32 * ctx.bonus_ad + 0.3f32 * ctx.missing_health,
        3 => 600f32 + 1.55f32 * ctx.bonus_ad + 0.35f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn jinx_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32 + 0.155f32 * ctx.bonus_ad + 0.25f32 * ctx.missing_health,
        2 => 45f32 + 0.155f32 * ctx.bonus_ad + 0.3f32 * ctx.missing_health,
        3 => 60f32 + 0.155f32 * ctx.bonus_ad + 0.35f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub static KSANTE: CachedChampion = CachedChampion {
    name: "K'Sante",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 625f32,
            per_level: 120f32,
        },
        mana: CachedChampionStatsMap {
            flat: 320f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 2.1f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.688f32,
            per_level: 2.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 150f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[
        ksante_q_1, ksante_w_1, ksante_w_2, ksante_w_3, ksante_w_4, ksante_w_5,
        ksante_r_1, ksante_r_2, ksante_r_3,
    ],
};
pub const fn ksante_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn ksante_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn ksante_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn ksante_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn ksante_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 180f32,
        2 => 260f32,
        3 => 340f32,
        4 => 420f32,
        5 => 500f32,
        _ => 0.0,
    }
}
pub const fn ksante_w_5(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn ksante_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 80f32,
        2 => 115f32,
        3 => 150f32,
        _ => 0.0,
    }
}
pub const fn ksante_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 80f32 + 0.05f32 * ctx.bonus_health,
        2 => 115f32 + 0.05f32 * ctx.bonus_health,
        3 => 150f32 + 0.05f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn ksante_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 160f32 + 0.05f32 * ctx.bonus_health,
        2 => 230f32 + 0.05f32 * ctx.bonus_health,
        3 => 300f32 + 0.05f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub static KAISA: CachedChampion = CachedChampion {
    name: "Kai'Sa",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 102f32,
        },
        mana: CachedChampionStatsMap {
            flat: 345f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 2.6f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 1.8f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 525f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[kaisa_q_1, kaisa_q_2, kaisa_q_3, kaisa_q_4, kaisa_w_1],
};
pub const fn kaisa_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.55f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        2 => 55f32 + 0.55f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        3 => 70f32 + 0.55f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        4 => 85f32 + 0.55f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        5 => 100f32 + 0.55f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kaisa_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 150f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        2 => 206.25f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        3 => 262.5f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        4 => 318.75f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        5 => 375f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kaisa_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        2 => 13.75f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        3 => 17.5f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        4 => 21.25f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        5 => 25f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kaisa_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.2375f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        2 => 123.75f32 + 1.2375f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        3 => 157.5f32 + 1.2375f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        4 => 191.25f32 + 1.2375f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        5 => 225f32 + 1.2375f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kaisa_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,
        2 => 55f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,
        3 => 80f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,
        4 => 105f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,
        5 => 130f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static KALISTA: CachedChampion = CachedChampion {
    name: "Kalista",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 560f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 57f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.694f32,
            per_level: 4.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694000005722045f32,
        attack_range: 525f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        kalista_q_1,
        kalista_w_1,
        kalista_w_2,
        kalista_e_1,
        kalista_e_2,
    ],
};
pub const fn kalista_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 1.05f32 * ctx.ad,
        2 => 75f32 + 1.05f32 * ctx.ad,
        3 => 140f32 + 1.05f32 * ctx.ad,
        4 => 205f32 + 1.05f32 * ctx.ad,
        5 => 270f32 + 1.05f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn kalista_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.1f32 * ctx.enemy_max_health,
        2 => 0.12f32 * ctx.enemy_max_health,
        3 => 0.14f32 * ctx.enemy_max_health,
        4 => 0.16f32 * ctx.enemy_max_health,
        5 => 0.18f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn kalista_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 100f32,
        2 => 125f32,
        3 => 150f32,
        4 => 175f32,
        5 => 200f32,
        _ => 0.0,
    }
}
pub const fn kalista_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 7f32 + 0.2f32 * ctx.ad + 0.2f32 * ctx.ap,
        2 => 14f32 + 0.25f32 * ctx.ad + 0.2f32 * ctx.ap,
        3 => 21f32 + 0.3f32 * ctx.ad + 0.2f32 * ctx.ap,
        4 => 28f32 + 0.35f32 * ctx.ad + 0.2f32 * ctx.ap,
        5 => 35f32 + 0.4f32 * ctx.ad + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kalista_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 5f32 + 0.7f32 * ctx.ad + 0.2f32 * ctx.ap,
        2 => 15f32 + 0.7f32 * ctx.ad + 0.2f32 * ctx.ap,
        3 => 25f32 + 0.7f32 * ctx.ad + 0.2f32 * ctx.ap,
        4 => 35f32 + 0.7f32 * ctx.ad + 0.2f32 * ctx.ap,
        5 => 45f32 + 0.7f32 * ctx.ad + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static KARMA: CachedChampion = CachedChampion {
    name: "Karma",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 374f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 51f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.3f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[karma_q_2, karma_w_1, karma_w_2],
};
pub const fn karma_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.7f32 * ctx.ap,
        2 => 110f32,
        3 => 160f32,
        4 => 210f32,
        5 => 260f32,
        _ => 0.0,
    }
}
pub const fn karma_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.45f32 * ctx.ap,
        2 => 65f32,
        3 => 90f32,
        4 => 115f32,
        5 => 140f32,
        _ => 0.0,
    }
}
pub const fn karma_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 0.9f32 * ctx.ap,
        2 => 130f32,
        3 => 180f32,
        4 => 230f32,
        5 => 280f32,
        _ => 0.0,
    }
}
pub static KARTHUS: CachedChampion = CachedChampion {
    name: "Karthus",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 467f32,
            per_level: 31f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 46f32,
            per_level: 3.25f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.11f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 450f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.93f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        karthus_q_1,
        karthus_q_2,
        karthus_e_1,
        karthus_e_2,
        karthus_r_1,
    ],
};
pub const fn karthus_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 118f32 + 0.7f32 * ctx.ap,
        3 => 156f32 + 0.7f32 * ctx.ap,
        4 => 194f32 + 0.7f32 * ctx.ap,
        5 => 232f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn karthus_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.35f32 * ctx.ap,
        2 => 59f32 + 0.35f32 * ctx.ap,
        3 => 78f32 + 0.35f32 * ctx.ap,
        4 => 97f32 + 0.35f32 * ctx.ap,
        5 => 116f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn karthus_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 30f32 + 0.2f32 * ctx.ap,
        2 => 50f32 + 0.2f32 * ctx.ap,
        3 => 70f32 + 0.2f32 * ctx.ap,
        4 => 90f32 + 0.2f32 * ctx.ap,
        5 => 110f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn karthus_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 7.5f32 + 0.05f32 * ctx.ap,
        2 => 12.5f32 + 0.05f32 * ctx.ap,
        3 => 17.5f32 + 0.05f32 * ctx.ap,
        4 => 22.5f32 + 0.05f32 * ctx.ap,
        5 => 27.5f32 + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn karthus_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.7f32 * ctx.ap,
        2 => 350f32 + 0.7f32 * ctx.ap,
        3 => 500f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static KASSADIN: CachedChampion = CachedChampion {
    name: "Kassadin",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 646f32,
            per_level: 119f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 87f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 3.9f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.64f32,
            per_level: 3.7f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.639999985694885f32,
        attack_range: 150f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        kassadin_q_1,
        kassadin_w_1,
        kassadin_e_1,
        kassadin_r_1,
        kassadin_r_2,
        kassadin_r_3,
        kassadin_r_4,
    ],
};
pub const fn kassadin_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + 0.6f32 * ctx.ap,
        2 => 95f32 + 0.6f32 * ctx.ap,
        3 => 125f32 + 0.6f32 * ctx.ap,
        4 => 155f32 + 0.6f32 * ctx.ap,
        5 => 185f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kassadin_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.8f32 * ctx.ap,
        2 => 75f32 + 0.8f32 * ctx.ap,
        3 => 100f32 + 0.8f32 * ctx.ap,
        4 => 125f32 + 0.8f32 * ctx.ap,
        5 => 150f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kassadin_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.65f32 * ctx.ap,
        2 => 100f32 + 0.65f32 * ctx.ap,
        3 => 130f32 + 0.65f32 * ctx.ap,
        4 => 160f32 + 0.65f32 * ctx.ap,
        5 => 190f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kassadin_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,
        2 => 90f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,
        3 => 110f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,
        _ => 0.0,
    }
}
pub const fn kassadin_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 35f32 + 0.07f32 * ctx.ap + 0.01f32 * ctx.max_mana,
        2 => 45f32 + 0.07f32 * ctx.ap + 0.01f32 * ctx.max_mana,
        3 => 55f32 + 0.07f32 * ctx.ap + 0.01f32 * ctx.max_mana,
        _ => 0.0,
    }
}
pub const fn kassadin_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 140f32 + 0.28f32 * ctx.ap + 0.04f32 * ctx.max_mana,
        2 => 180f32 + 0.28f32 * ctx.ap + 0.04f32 * ctx.max_mana,
        3 => 220f32 + 0.28f32 * ctx.ap + 0.04f32 * ctx.max_mana,
        _ => 0.0,
    }
}
pub const fn kassadin_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 210f32 + 0.78f32 * ctx.ap + 0.06f32 * ctx.max_mana,
        2 => 270f32 + 0.78f32 * ctx.ap + 0.06f32 * ctx.max_mana,
        3 => 330f32 + 0.78f32 * ctx.ap + 0.06f32 * ctx.max_mana,
        _ => 0.0,
    }
}
pub static KATARINA: CachedChampion = CachedChampion {
    name: "Katarina",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 672f32,
            per_level: 108f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.74f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        katarina_q_1,
        katarina_e_1,
        katarina_r_1,
        katarina_r_2,
        katarina_r_3,
        katarina_r_4,
        katarina_r_5,
    ],
};
pub const fn katarina_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.4f32 * ctx.ap,
        2 => 115f32 + 0.4f32 * ctx.ap,
        3 => 150f32 + 0.4f32 * ctx.ap,
        4 => 185f32 + 0.4f32 * ctx.ap,
        5 => 220f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn katarina_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        2 => 30f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        3 => 40f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        4 => 50f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        5 => 60f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn katarina_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 25f32 + 0.19f32 * ctx.ap,
        2 => 37.5f32 + 0.19f32 * ctx.ap,
        3 => 50f32 + 0.19f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn katarina_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 375f32 + 2.85f32 * ctx.ap,
        2 => 562.5f32 + 2.85f32 * ctx.ap,
        3 => 750f32 + 2.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn katarina_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn katarina_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.25f32,
        2 => 0.3f32,
        3 => 0.35f32,
        _ => 0.0,
    }
}
pub const fn katarina_r_5(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        _ => 0.0,
    }
}
pub static KAYLE: CachedChampion = CachedChampion {
    name: "Kayle",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 670f32,
            per_level: 92f32,
        },
        mana: CachedChampionStatsMap {
            flat: 330f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 50f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.666999995708465f32,
        attack_range: 175f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[kayle_q_1, kayle_e_1, kayle_e_2, kayle_r_1],
};
pub const fn kayle_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 100f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 140f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 180f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 220f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kayle_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.08f32 * ctx.missing_health + 0.015f32 * 0.01f32 * ctx.ap,
        2 => 0.085f32 * ctx.missing_health + 0.015f32 * 0.01f32 * ctx.ap,
        3 => 0.09f32 * ctx.missing_health + 0.015f32 * 0.01f32 * ctx.ap,
        4 => 0.095f32 * ctx.missing_health + 0.015f32 * 0.01f32 * ctx.ap,
        5 => 0.1f32 * ctx.missing_health + 0.015f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kayle_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 15f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        2 => 20f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        3 => 25f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        4 => 30f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        5 => 35f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kayle_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,
        2 => 300f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,
        3 => 400f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static KAYN: CachedChampion = CachedChampion {
    name: "Kayn",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 410f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.669f32,
            per_level: 2.7f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.669000029563903f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        kayn_q_1, kayn_q_2, kayn_q_3, kayn_q_4, kayn_q_5, kayn_q_6, kayn_w_1,
        kayn_r_1,
    ],
};
pub const fn kayn_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.85f32 * ctx.bonus_ad,
        2 => 100f32 + 0.85f32 * ctx.bonus_ad,
        3 => 125f32 + 0.85f32 * ctx.bonus_ad,
        4 => 150f32 + 0.85f32 * ctx.bonus_ad,
        5 => 175f32 + 0.85f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kayn_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 150f32 + 1.7f32 * ctx.bonus_ad,
        2 => 200f32 + 1.7f32 * ctx.bonus_ad,
        3 => 250f32 + 1.7f32 * ctx.bonus_ad,
        4 => 300f32 + 1.7f32 * ctx.bonus_ad,
        5 => 350f32 + 1.7f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kayn_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 115f32 + 0.85f32 * ctx.bonus_ad,
        2 => 140f32 + 0.85f32 * ctx.bonus_ad,
        3 => 165f32 + 0.85f32 * ctx.bonus_ad,
        4 => 190f32 + 0.85f32 * ctx.bonus_ad,
        5 => 215f32 + 0.85f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kayn_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 230f32 + 1.7f32 * ctx.bonus_ad,
        2 => 280f32 + 1.7f32 * ctx.bonus_ad,
        3 => 330f32 + 1.7f32 * ctx.bonus_ad,
        4 => 380f32 + 1.7f32 * ctx.bonus_ad,
        5 => 430f32 + 1.7f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kayn_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 200f32,
        2 => 250f32,
        3 => 300f32,
        4 => 350f32,
        5 => 400f32,
        _ => 0.0,
    }
}
pub const fn kayn_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 400f32,
        2 => 500f32,
        3 => 600f32,
        4 => 700f32,
        5 => 800f32,
        _ => 0.0,
    }
}
pub const fn kayn_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 85f32 + 1.1f32 * ctx.bonus_ad,
        2 => 130f32 + 1.1f32 * ctx.bonus_ad,
        3 => 175f32 + 1.1f32 * ctx.bonus_ad,
        4 => 220f32 + 1.1f32 * ctx.bonus_ad,
        5 => 265f32 + 1.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kayn_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 1.5f32 * ctx.bonus_ad,
        2 => 250f32 + 1.5f32 * ctx.bonus_ad,
        3 => 350f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static KENNEN: CachedChampion = CachedChampion {
    name: "Kennen",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 580f32,
            per_level: 98f32,
        },
        mana: CachedChampionStatsMap {
            flat: 200f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 4.95f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 48f32,
            per_level: 3.75f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.4f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.689999997615814f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        kennen_q_1, kennen_w_1, kennen_w_2, kennen_e_1, kennen_e_2, kennen_r_1,
        kennen_r_2,
    ],
};
pub const fn kennen_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.75f32 * ctx.ap,
        2 => 125f32 + 0.75f32 * ctx.ap,
        3 => 175f32 + 0.75f32 * ctx.ap,
        4 => 225f32 + 0.75f32 * ctx.ap,
        5 => 275f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kennen_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.8f32 * ctx.ap,
        2 => 95f32 + 0.8f32 * ctx.ap,
        3 => 120f32 + 0.8f32 * ctx.ap,
        4 => 145f32 + 0.8f32 * ctx.ap,
        5 => 170f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kennen_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 35f32 + 0.8f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        2 => 45f32 + 0.9f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        3 => 55f32 + ctx.bonus_ad + 0.35f32 * ctx.ap,
        4 => 65f32 + 1.1f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        5 => 75f32 + 1.2f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kennen_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 120f32 + 0.8f32 * ctx.ap,
        3 => 160f32 + 0.8f32 * ctx.ap,
        4 => 200f32 + 0.8f32 * ctx.ap,
        5 => 240f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kennen_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 52f32 + 0.52f32 * ctx.ap,
        2 => 78f32 + 0.52f32 * ctx.ap,
        3 => 104f32 + 0.52f32 * ctx.ap,
        4 => 130f32 + 0.52f32 * ctx.ap,
        5 => 156f32 + 0.52f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kennen_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 1.6875f32 * ctx.ap,
        2 => 562.5f32 + 1.6875f32 * ctx.ap,
        3 => 825f32 + 1.6875f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kennen_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 40f32 + 0.225f32 * ctx.ap,
        2 => 75f32 + 0.225f32 * ctx.ap,
        3 => 110f32 + 0.225f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static KHAZIX: CachedChampion = CachedChampion {
    name: "Kha'Zix",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 643f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 327f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.668f32,
            per_level: 2.7f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.667999982833862f32,
        attack_range: 125f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[khazix_q_1, khazix_q_2, khazix_w_1, khazix_e_1],
};
pub const fn khazix_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 168f32 + 2.31f32 * ctx.bonus_ad,
        2 => 220.5f32 + 2.31f32 * ctx.bonus_ad,
        3 => 273f32 + 2.31f32 * ctx.bonus_ad,
        4 => 325.5f32 + 2.31f32 * ctx.bonus_ad,
        5 => 378f32 + 2.31f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn khazix_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 1.1f32 * ctx.bonus_ad,
        2 => 105f32 + 1.1f32 * ctx.bonus_ad,
        3 => 130f32 + 1.1f32 * ctx.bonus_ad,
        4 => 155f32 + 1.1f32 * ctx.bonus_ad,
        5 => 180f32 + 1.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn khazix_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 85f32 + ctx.bonus_ad,
        2 => 115f32 + ctx.bonus_ad,
        3 => 145f32 + ctx.bonus_ad,
        4 => 175f32 + ctx.bonus_ad,
        5 => 205f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn khazix_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 65f32 + 0.4f32 * ctx.bonus_ad,
        2 => 100f32 + 0.4f32 * ctx.bonus_ad,
        3 => 135f32 + 0.4f32 * ctx.bonus_ad,
        4 => 170f32 + 0.4f32 * ctx.bonus_ad,
        5 => 205f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static KINDRED: CachedChampion = CachedChampion {
    name: "Kindred",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 595f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 65f32,
            per_level: 3.25f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.5f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 500f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        kindred_q_1,
        kindred_w_1,
        kindred_w_2,
        kindred_e_1,
        kindred_e_2,
    ],
};
pub const fn kindred_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.75f32 * ctx.bonus_ad,
        2 => 65f32 + 0.75f32 * ctx.bonus_ad,
        3 => 90f32 + 0.75f32 * ctx.bonus_ad,
        4 => 115f32 + 0.75f32 * ctx.bonus_ad,
        5 => 140f32 + 0.75f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kindred_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            25f32
                + 0.2f32 * ctx.bonus_ad
                + 0.2f32 * ctx.ap
                + 0.015f32 * ctx.current_health
                + 0.01f32 * ctx.kindred_stacks
        }
        2 => {
            30f32
                + 0.2f32 * ctx.bonus_ad
                + 0.2f32 * ctx.ap
                + 0.015f32 * ctx.current_health
                + 0.01f32 * ctx.kindred_stacks
        }
        3 => {
            35f32
                + 0.2f32 * ctx.bonus_ad
                + 0.2f32 * ctx.ap
                + 0.015f32 * ctx.current_health
                + 0.01f32 * ctx.kindred_stacks
        }
        4 => {
            40f32
                + 0.2f32 * ctx.bonus_ad
                + 0.2f32 * ctx.ap
                + 0.015f32 * ctx.current_health
                + 0.01f32 * ctx.kindred_stacks
        }
        5 => {
            45f32
                + 0.2f32 * ctx.bonus_ad
                + 0.2f32 * ctx.ap
                + 0.015f32 * ctx.current_health
                + 0.01f32 * ctx.kindred_stacks
        }
        _ => 0.0,
    }
}
pub const fn kindred_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            37.5f32
                + 0.3f32 * ctx.bonus_ad
                + 0.3f32 * ctx.ap
                + 0.0225f32 * ctx.current_health
                + 0.015f32 * ctx.kindred_stacks
        }
        2 => {
            45f32
                + 0.3f32 * ctx.bonus_ad
                + 0.3f32 * ctx.ap
                + 0.0225f32 * ctx.current_health
                + 0.015f32 * ctx.kindred_stacks
        }
        3 => {
            52.5f32
                + 0.3f32 * ctx.bonus_ad
                + 0.3f32 * ctx.ap
                + 0.0225f32 * ctx.current_health
                + 0.015f32 * ctx.kindred_stacks
        }
        4 => {
            60f32
                + 0.3f32 * ctx.bonus_ad
                + 0.3f32 * ctx.ap
                + 0.0225f32 * ctx.current_health
                + 0.015f32 * ctx.kindred_stacks
        }
        5 => {
            67.5f32
                + 0.3f32 * ctx.bonus_ad
                + 0.3f32 * ctx.ap
                + 0.0225f32 * ctx.current_health
                + 0.015f32 * ctx.kindred_stacks
        }
        _ => 0.0,
    }
}
pub const fn kindred_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn kindred_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub static KLED: CachedChampion = CachedChampion {
    name: "Kled",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_7),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 410f32,
            per_level: 84f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 65f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.5f32,
        },
        movespeed: 305f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 250f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        kled_q_2, kled_q_4, kled_q_5, kled_q_6, kled_q_7, kled_w_1, kled_w_2,
        kled_e_1, kled_e_2, kled_r_1, kled_r_2,
    ],
};
pub const fn kled_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.6f32 * ctx.bonus_ad,
        2 => 55f32 + 0.6f32 * ctx.bonus_ad,
        3 => 80f32 + 0.6f32 * ctx.bonus_ad,
        4 => 105f32 + 0.6f32 * ctx.bonus_ad,
        5 => 130f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.8f32 * ctx.bonus_ad,
        2 => 165f32 + 1.8f32 * ctx.bonus_ad,
        3 => 240f32 + 1.8f32 * ctx.bonus_ad,
        4 => 315f32 + 1.8f32 * ctx.bonus_ad,
        5 => 390f32 + 1.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.9f32 * ctx.bonus_ad,
        2 => 82.5f32 + 0.9f32 * ctx.bonus_ad,
        3 => 120f32 + 0.9f32 * ctx.bonus_ad,
        4 => 157.5f32 + 0.9f32 * ctx.bonus_ad,
        5 => 195f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 1.2f32 * ctx.bonus_ad,
        2 => 110f32 + 1.2f32 * ctx.bonus_ad,
        3 => 160f32 + 1.2f32 * ctx.bonus_ad,
        4 => 210f32 + 1.2f32 * ctx.bonus_ad,
        5 => 260f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_q_7(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.3f32,
        2 => 0.35f32,
        3 => 0.4f32,
        4 => 0.45f32,
        5 => 0.5f32,
        _ => 0.0,
    }
}
pub const fn kled_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 62f32 + 0f32,
        2 => 63.75f32 + 15f32,
        3 => 65.5f32 + 30f32,
        4 => 67.25f32 + 45f32,
        5 => 69f32 + 60f32,
        6 => 70.75f32,
        7 => 72.5f32,
        8 => 74.25f32,
        9 => 76f32,
        10 => 77.75f32,
        11 => 79.5f32,
        12 => 81.25f32,
        13 => 83f32,
        14 => 84.75f32,
        15 => 86.5f32,
        16 => 88.25f32,
        17 => 90f32,
        18 => 91.75f32,
        _ => 0.0,
    }
}
pub const fn kled_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            20f32
                + 0.045f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.bonus_ad
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        2 => {
            30f32
                + 0.05f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.bonus_ad
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        3 => {
            40f32
                + 0.055f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.bonus_ad
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        4 => {
            50f32
                + 0.06f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.bonus_ad
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        5 => {
            60f32
                + 0.065f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.bonus_ad
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        _ => 0.0,
    }
}
pub const fn kled_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 35f32 + 0.55f32 * ctx.bonus_ad,
        2 => 60f32 + 0.55f32 * ctx.bonus_ad,
        3 => 85f32 + 0.55f32 * ctx.bonus_ad,
        4 => 110f32 + 0.55f32 * ctx.bonus_ad,
        5 => 135f32 + 0.55f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 1.1f32 * ctx.bonus_ad,
        2 => 120f32 + 1.1f32 * ctx.bonus_ad,
        3 => 170f32 + 1.1f32 * ctx.bonus_ad,
        4 => 220f32 + 1.1f32 * ctx.bonus_ad,
        5 => 270f32 + 1.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.12f32 * ctx.enemy_max_health + 0.09f32 * 0.01f32 * ctx.bonus_ad,
        2 => 0.18f32 * ctx.enemy_max_health + 0.09f32 * 0.01f32 * ctx.bonus_ad,
        3 => 0.24f32 * ctx.enemy_max_health + 0.09f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn kled_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.04f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,
        2 => 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,
        3 => 0.08f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static KOGMAW: CachedChampion = CachedChampion {
    name: "Kog'Maw",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 635f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 325f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.45f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.665f32,
            per_level: 2.65f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.665000021457672f32,
        attack_range: 500f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.92f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[kogmaw_q_1, kogmaw_w_1, kogmaw_e_1, kogmaw_r_1, kogmaw_r_2],
};
pub const fn kogmaw_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 125f32 + 0.8f32 * ctx.ap,
        3 => 170f32 + 0.8f32 * ctx.ap,
        4 => 215f32 + 0.8f32 * ctx.ap,
        5 => 260f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kogmaw_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.03f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        2 => 0.0375f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        3 => 0.045f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        4 => 0.0525f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        5 => 0.06f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kogmaw_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.65f32 * ctx.ap,
        2 => 110f32 + 0.65f32 * ctx.ap,
        3 => 150f32 + 0.65f32 * ctx.ap,
        4 => 190f32 + 0.65f32 * ctx.ap,
        5 => 230f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kogmaw_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        2 => 280f32 + 1.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        3 => 360f32 + 1.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn kogmaw_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.75f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        2 => 140f32 + 0.75f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        3 => 180f32 + 0.75f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static LEBLANC: CachedChampion = CachedChampion {
    name: "LeBlanc",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_7),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 598f32,
            per_level: 111f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 2.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.35f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.4f32,
        attack_range: 525f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        leblanc_q_1,
        leblanc_q_2,
        leblanc_w_1,
        leblanc_e_1,
        leblanc_e_2,
        leblanc_e_3,
        leblanc_r_1,
        leblanc_r_2,
        leblanc_r_3,
        leblanc_r_4,
        leblanc_r_5,
        leblanc_r_6,
        leblanc_r_7,
    ],
};
pub const fn leblanc_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + 0.4f32 * ctx.ap,
        2 => 90f32 + 0.4f32 * ctx.ap,
        3 => 115f32 + 0.4f32 * ctx.ap,
        4 => 140f32 + 0.4f32 * ctx.ap,
        5 => 165f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 130f32 + 0.8f32 * ctx.ap,
        2 => 180f32 + 0.8f32 * ctx.ap,
        3 => 230f32 + 0.8f32 * ctx.ap,
        4 => 280f32 + 0.8f32 * ctx.ap,
        5 => 330f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 75f32 + 0.7f32 * ctx.ap,
        2 => 115f32 + 0.7f32 * ctx.ap,
        3 => 155f32 + 0.7f32 * ctx.ap,
        4 => 195f32 + 0.7f32 * ctx.ap,
        5 => 235f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.4f32 * ctx.ap,
        2 => 70f32 + 0.4f32 * ctx.ap,
        3 => 90f32 + 0.4f32 * ctx.ap,
        4 => 110f32 + 0.4f32 * ctx.ap,
        5 => 130f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.85f32 * ctx.ap,
        2 => 120f32 + 0.85f32 * ctx.ap,
        3 => 160f32 + 0.85f32 * ctx.ap,
        4 => 200f32 + 0.85f32 * ctx.ap,
        5 => 240f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 130f32 + 1.25f32 * ctx.ap,
        2 => 190f32 + 1.25f32 * ctx.ap,
        3 => 250f32 + 1.25f32 * ctx.ap,
        4 => 310f32 + 1.25f32 * ctx.ap,
        5 => 370f32 + 1.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.75f32 * ctx.ap,
        2 => 300f32 + 0.75f32 * ctx.ap,
        3 => 450f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.4f32 * ctx.ap,
        2 => 140f32 + 0.4f32 * ctx.ap,
        3 => 210f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 140f32 + 0.85f32 * ctx.ap,
        2 => 280f32 + 0.85f32 * ctx.ap,
        3 => 420f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 210f32 + 1.25f32 * ctx.ap,
        2 => 420f32 + 1.25f32 * ctx.ap,
        3 => 630f32 + 1.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_5(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 140f32 + 0.8f32 * ctx.ap,
        2 => 280f32 + 0.8f32 * ctx.ap,
        3 => 420f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_6(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.4f32 * ctx.ap,
        2 => 140f32 + 0.4f32 * ctx.ap,
        3 => 210f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leblanc_r_7(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 210f32 + 1.2f32 * ctx.ap,
        2 => 420f32 + 1.2f32 * ctx.ap,
        3 => 630f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static LEESIN: CachedChampion = CachedChampion {
    name: "Lee Sin",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 645f32,
            per_level: 108f32,
        },
        mana: CachedChampionStatsMap {
            flat: 200f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.9f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 3.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.651f32,
            per_level: 3f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.651000022888183f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[leesin_q_2, leesin_e_1, leesin_r_1, leesin_r_2],
};
pub const fn leesin_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 55f32 + 1.15f32 * ctx.bonus_ad,
        2 => 80f32 + 1.15f32 * ctx.bonus_ad,
        3 => 105f32 + 1.15f32 * ctx.bonus_ad,
        4 => 130f32 + 1.15f32 * ctx.bonus_ad,
        5 => 155f32 + 1.15f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn leesin_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 35f32 + ctx.ad,
        2 => 60f32 + ctx.ad,
        3 => 85f32 + ctx.ad,
        4 => 110f32 + ctx.ad,
        5 => 135f32 + ctx.ad,
        _ => 0.0,
    }
}
pub const fn leesin_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 2f32 * ctx.bonus_ad,
        2 => 400f32 + 2f32 * ctx.bonus_ad,
        3 => 625f32 + 2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn leesin_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 2f32 * ctx.bonus_ad + 0.12f32 * ctx.enemy_bonus_health,
        2 => 400f32 + 2f32 * ctx.bonus_ad + 0.15f32 * ctx.enemy_bonus_health,
        3 => 625f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_bonus_health,
        _ => 0.0,
    }
}
pub static LEONA: CachedChampion = CachedChampion {
    name: "Leona",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 646f32,
            per_level: 101f32,
        },
        mana: CachedChampionStatsMap {
            flat: 302f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 43f32,
            per_level: 4.8f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.9f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[leona_q_1, leona_w_1, leona_w_2, leona_e_1, leona_r_1],
};
pub const fn leona_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.3f32 * ctx.ap,
        2 => 35f32 + 0.3f32 * ctx.ap,
        3 => 60f32 + 0.3f32 * ctx.ap,
        4 => 85f32 + 0.3f32 * ctx.ap,
        5 => 110f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leona_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 8f32,
        2 => 12f32,
        3 => 16f32,
        4 => 20f32,
        5 => 24f32,
        _ => 0.0,
    }
}
pub const fn leona_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 55f32 + 0.4f32 * ctx.ap,
        2 => 85f32 + 0.4f32 * ctx.ap,
        3 => 115f32 + 0.4f32 * ctx.ap,
        4 => 145f32 + 0.4f32 * ctx.ap,
        5 => 175f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leona_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.4f32 * ctx.ap,
        2 => 90f32 + 0.4f32 * ctx.ap,
        3 => 130f32 + 0.4f32 * ctx.ap,
        4 => 170f32 + 0.4f32 * ctx.ap,
        5 => 210f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn leona_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.8f32 * ctx.ap,
        2 => 225f32 + 0.8f32 * ctx.ap,
        3 => 300f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static LILLIA: CachedChampion = CachedChampion {
    name: "Lillia",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 605f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 410f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.7f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 325f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        lillia_q_1, lillia_q_2, lillia_w_1, lillia_w_2, lillia_w_3, lillia_w_4,
        lillia_e_1, lillia_r_1,
    ],
};
pub const fn lillia_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + 0.35f32 * ctx.ap,
        2 => 45f32 + 0.35f32 * ctx.ap,
        3 => 55f32 + 0.35f32 * ctx.ap,
        4 => 65f32 + 0.35f32 * ctx.ap,
        5 => 75f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 90f32 + 0.7f32 * ctx.ap,
        3 => 110f32 + 0.7f32 * ctx.ap,
        4 => 130f32 + 0.7f32 * ctx.ap,
        5 => 150f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 240f32 + 1.05f32 * ctx.ap,
        2 => 300f32 + 1.05f32 * ctx.ap,
        3 => 360f32 + 1.05f32 * ctx.ap,
        4 => 420f32 + 1.05f32 * ctx.ap,
        5 => 480f32 + 1.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 0.35f32 * ctx.ap,
        2 => 100f32 + 0.35f32 * ctx.ap,
        3 => 120f32 + 0.35f32 * ctx.ap,
        4 => 140f32 + 0.35f32 * ctx.ap,
        5 => 160f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 120f32 + 0.525f32 * ctx.ap,
        2 => 150f32 + 0.525f32 * ctx.ap,
        3 => 180f32 + 0.525f32 * ctx.ap,
        4 => 210f32 + 0.525f32 * ctx.ap,
        5 => 240f32 + 0.525f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.175f32 * ctx.ap,
        2 => 50f32 + 0.175f32 * ctx.ap,
        3 => 60f32 + 0.175f32 * ctx.ap,
        4 => 70f32 + 0.175f32 * ctx.ap,
        5 => 80f32 + 0.175f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.5f32 * ctx.ap,
        2 => 85f32 + 0.5f32 * ctx.ap,
        3 => 110f32 + 0.5f32 * ctx.ap,
        4 => 135f32 + 0.5f32 * ctx.ap,
        5 => 160f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lillia_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.4f32 * ctx.ap,
        2 => 150f32 + 0.4f32 * ctx.ap,
        3 => 200f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static LISSANDRA: CachedChampion = CachedChampion {
    name: "Lissandra",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 475f32,
            per_level: 30f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.9f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 2.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.656f32,
            per_level: 1.5f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[lissandra_q_1, lissandra_w_1, lissandra_e_1, lissandra_r_1],
};
pub const fn lissandra_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.75f32 * ctx.ap,
        2 => 115f32 + 0.75f32 * ctx.ap,
        3 => 150f32 + 0.75f32 * ctx.ap,
        4 => 185f32 + 0.75f32 * ctx.ap,
        5 => 220f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lissandra_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 105f32 + 0.7f32 * ctx.ap,
        3 => 140f32 + 0.7f32 * ctx.ap,
        4 => 175f32 + 0.7f32 * ctx.ap,
        5 => 210f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lissandra_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.6f32 * ctx.ap,
        2 => 105f32 + 0.6f32 * ctx.ap,
        3 => 140f32 + 0.6f32 * ctx.ap,
        4 => 175f32 + 0.6f32 * ctx.ap,
        5 => 210f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lissandra_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.75f32 * ctx.ap,
        2 => 250f32 + 0.75f32 * ctx.ap,
        3 => 350f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static LUCIAN: CachedChampion = CachedChampion {
    name: "Lucian",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 641f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 320f32,
            per_level: 43f32,
        },
        armor: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.9f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 3.3f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 500f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        lucian_q_1, lucian_w_1, lucian_r_1, lucian_r_2, lucian_r_3, lucian_r_4,
        lucian_r_5, lucian_r_6,
    ],
};
pub const fn lucian_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 85f32 + 0.6f32 * ctx.bonus_ad,
        2 => 115f32 + 0.75f32 * ctx.bonus_ad,
        3 => 145f32 + 0.9f32 * ctx.bonus_ad,
        4 => 175f32 + 1.05f32 * ctx.bonus_ad,
        5 => 205f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn lucian_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 75f32 + 0.9f32 * ctx.ap,
        2 => 110f32 + 0.9f32 * ctx.ap,
        3 => 145f32 + 0.9f32 * ctx.ap,
        4 => 180f32 + 0.9f32 * ctx.ap,
        5 => 215f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lucian_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 75f32 + 1.25f32 * ctx.ad + 0.75f32 * ctx.ap,
        2 => 150f32 + 1.25f32 * ctx.ad + 0.75f32 * ctx.ap,
        3 => 225f32 + 1.25f32 * ctx.ad + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lucian_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 2.5f32 * ctx.ad + 1.5f32 * ctx.ap,
        2 => 300f32 + 2.5f32 * ctx.ad + 1.5f32 * ctx.ap,
        3 => 450f32 + 2.5f32 * ctx.ad + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lucian_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32 + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,
        2 => 60f32 + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,
        3 => 90f32 + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lucian_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 15f32 + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,
        2 => 30f32 + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,
        3 => 45f32 + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lucian_r_5(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            660f32
                + 0.3f32 * ctx.crit_chance
                + 11f32 * ctx.ad
                + 6.6f32 * ctx.ap
                + 0.5f32 * ctx.ad
                + 0.3f32 * ctx.ap
        }
        2 => {
            1320f32
                + 0.6f32 * ctx.crit_chance
                + 11f32 * ctx.ad
                + 6.6f32 * ctx.ap
                + 0.5f32 * ctx.ad
                + 0.3f32 * ctx.ap
        }
        3 => {
            1980f32
                + 0.9f32 * ctx.crit_chance
                + 11f32 * ctx.ad
                + 6.6f32 * ctx.ap
                + 0.5f32 * ctx.ad
                + 0.3f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn lucian_r_6(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            330f32
                + 0.15f32 * ctx.crit_chance
                + 5.5f32 * ctx.ad
                + 3.3f32 * ctx.ap
                + 0.25f32 * ctx.ad
                + 0.15f32 * ctx.ap
        }
        2 => {
            660f32
                + 0.3f32 * ctx.crit_chance
                + 5.5f32 * ctx.ad
                + 3.3f32 * ctx.ap
                + 0.25f32 * ctx.ad
                + 0.15f32 * ctx.ap
        }
        3 => {
            990f32
                + 0.45f32 * ctx.crit_chance
                + 5.5f32 * ctx.ad
                + 3.3f32 * ctx.ap
                + 0.25f32 * ctx.ad
                + 0.15f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub static LULU: CachedChampion = CachedChampion {
    name: "Lulu",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 565f32,
            per_level: 92f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 55f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 47f32,
            per_level: 2.6f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.25f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        lulu_q_1, lulu_q_2, lulu_q_3, lulu_q_4, lulu_q_5, lulu_q_6, lulu_e_1,
    ],
};
pub const fn lulu_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.5f32 * ctx.ap,
        2 => 95f32 + 0.5f32 * ctx.ap,
        3 => 130f32 + 0.5f32 * ctx.ap,
        4 => 165f32 + 0.5f32 * ctx.ap,
        5 => 200f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lulu_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 42f32 + 0.35f32 * ctx.ap,
        2 => 66.5f32 + 0.35f32 * ctx.ap,
        3 => 91f32 + 0.35f32 * ctx.ap,
        4 => 115.5f32 + 0.35f32 * ctx.ap,
        5 => 140f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lulu_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.25f32 * ctx.ap,
        2 => 47.5f32 + 0.25f32 * ctx.ap,
        3 => 65f32 + 0.25f32 * ctx.ap,
        4 => 82.5f32 + 0.25f32 * ctx.ap,
        5 => 100f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lulu_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 21f32 + 0.175f32 * ctx.ap,
        2 => 33.25f32 + 0.175f32 * ctx.ap,
        3 => 45.5f32 + 0.175f32 * ctx.ap,
        4 => 57.75f32 + 0.175f32 * ctx.ap,
        5 => 70f32 + 0.175f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lulu_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 0.75f32 * ctx.ap,
        2 => 142.5f32 + 0.75f32 * ctx.ap,
        3 => 195f32 + 0.75f32 * ctx.ap,
        4 => 247.5f32 + 0.75f32 * ctx.ap,
        5 => 300f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lulu_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 63f32 + 0.525f32 * ctx.ap,
        2 => 99.75f32 + 0.525f32 * ctx.ap,
        3 => 136.5f32 + 0.525f32 * ctx.ap,
        4 => 173.25f32 + 0.525f32 * ctx.ap,
        5 => 210f32 + 0.525f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lulu_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.5f32 * ctx.ap,
        2 => 120f32 + 0.5f32 * ctx.ap,
        3 => 160f32 + 0.5f32 * ctx.ap,
        4 => 200f32 + 0.5f32 * ctx.ap,
        5 => 240f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static LUX: CachedChampion = CachedChampion {
    name: "Lux",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 580f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 480f32,
            per_level: 23.5f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.669f32,
            per_level: 3f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[lux_q_1, lux_e_1, lux_r_1],
};
pub const fn lux_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.65f32 * ctx.ap,
        2 => 120f32 + 0.65f32 * ctx.ap,
        3 => 160f32 + 0.65f32 * ctx.ap,
        4 => 200f32 + 0.65f32 * ctx.ap,
        5 => 240f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lux_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 65f32 + 0.8f32 * ctx.ap,
        2 => 115f32 + 0.8f32 * ctx.ap,
        3 => 165f32 + 0.8f32 * ctx.ap,
        4 => 215f32 + 0.8f32 * ctx.ap,
        5 => 265f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn lux_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 1.2f32 * ctx.ap,
        2 => 400f32 + 1.2f32 * ctx.ap,
        3 => 500f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MALPHITE: CachedChampion = CachedChampion {
    name: "Malphite",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 665f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 4.95f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.736f32,
            per_level: 3.4f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        malphite_q_1,
        malphite_w_1,
        malphite_w_2,
        malphite_e_1,
        malphite_r_1,
    ],
};
pub const fn malphite_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.6f32 * ctx.ap,
        2 => 120f32 + 0.6f32 * ctx.ap,
        3 => 170f32 + 0.6f32 * ctx.ap,
        4 => 220f32 + 0.6f32 * ctx.ap,
        5 => 270f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malphite_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,
        2 => 40f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,
        3 => 50f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,
        4 => 60f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,
        5 => 70f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,
        _ => 0.0,
    }
}
pub const fn malphite_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 15f32 + 0.3f32 * ctx.ap + 0.15f32 * ctx.armor,
        2 => 25f32 + 0.3f32 * ctx.ap + 0.15f32 * ctx.armor,
        3 => 35f32 + 0.3f32 * ctx.ap + 0.15f32 * ctx.armor,
        4 => 45f32 + 0.3f32 * ctx.ap + 0.15f32 * ctx.armor,
        5 => 55f32 + 0.3f32 * ctx.ap + 0.15f32 * ctx.armor,
        _ => 0.0,
    }
}
pub const fn malphite_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.6f32 * ctx.ap + 0.4f32 * ctx.armor,
        2 => 110f32 + 0.6f32 * ctx.ap + 0.4f32 * ctx.armor,
        3 => 150f32 + 0.6f32 * ctx.ap + 0.4f32 * ctx.armor,
        4 => 190f32 + 0.6f32 * ctx.ap + 0.4f32 * ctx.armor,
        5 => 230f32 + 0.6f32 * ctx.ap + 0.4f32 * ctx.armor,
        _ => 0.0,
    }
}
pub const fn malphite_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.9f32 * ctx.ap,
        2 => 300f32 + 0.9f32 * ctx.ap,
        3 => 400f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MALZAHAR: CachedChampion = CachedChampion {
    name: "Malzahar",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 580f32,
            per_level: 101f32,
        },
        mana: CachedChampionStatsMap {
            flat: 375f32,
            per_level: 28f32,
        },
        armor: CachedChampionStatsMap {
            flat: 18f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 500f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1.08f32,
        urf_damage_dealt: 0.92f32,
    },
    merge_data: &[],
    closures: &[
        malzahar_q_1,
        malzahar_w_1,
        malzahar_w_2,
        malzahar_e_1,
        malzahar_e_2,
        malzahar_r_1,
        malzahar_r_2,
        malzahar_r_3,
        malzahar_r_4,
    ],
};
pub const fn malzahar_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.55f32 * ctx.ap,
        2 => 105f32 + 0.55f32 * ctx.ap,
        3 => 140f32 + 0.55f32 * ctx.ap,
        4 => 175f32 + 0.55f32 * ctx.ap,
        5 => 210f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malzahar_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 12f32 + 0.4f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        2 => 8.5f32 + 14f32 + 0.4f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        3 => 12f32 + 16f32 + 0.4f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        4 => 15.5f32 + 18f32 + 0.4f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        5 => 19f32 + 20f32 + 0.4f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,
        6 => 22.5f32,
        7 => 26f32,
        8 => 29.5f32,
        9 => 33f32,
        10 => 36.5f32,
        11 => 40f32,
        12 => 43.5f32,
        13 => 47f32,
        14 => 50.5f32,
        15 => 54f32,
        16 => 57.5f32,
        17 => 61f32,
        18 => 64.5f32,
        _ => 0.0,
    }
}
pub const fn malzahar_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 15f32 + 36f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        2 => 25.5f32 + 42f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        3 => 36f32 + 48f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        4 => 46.5f32 + 54f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        5 => 57f32 + 60f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        6 => 67.5f32,
        7 => 78f32,
        8 => 88.5f32,
        9 => 99f32,
        10 => 109.5f32,
        11 => 120f32,
        12 => 130.5f32,
        13 => 141f32,
        14 => 151.5f32,
        15 => 162f32,
        16 => 172.5f32,
        17 => 183f32,
        18 => 193.5f32,
        _ => 0.0,
    }
}
pub const fn malzahar_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 5f32 + 0.05f32 * ctx.ap,
        2 => 7.1875f32 + 0.05f32 * ctx.ap,
        3 => 9.375f32 + 0.05f32 * ctx.ap,
        4 => 11.5625f32 + 0.05f32 * ctx.ap,
        5 => 13.75f32 + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malzahar_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 115f32 + 0.8f32 * ctx.ap,
        3 => 150f32 + 0.8f32 * ctx.ap,
        4 => 185f32 + 0.8f32 * ctx.ap,
        5 => 220f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malzahar_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 12.5f32 + 0.08f32 * ctx.ap,
        2 => 20f32 + 0.08f32 * ctx.ap,
        3 => 27.5f32 + 0.08f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malzahar_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.8f32 * ctx.ap,
        2 => 200f32 + 0.8f32 * ctx.ap,
        3 => 275f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malzahar_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.01f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,
        2 => 0.015f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,
        3 => 0.02f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn malzahar_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.1f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.ap,
        2 => 0.15f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.ap,
        3 => 0.2f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MAOKAI: CachedChampion = CachedChampion {
    name: "Maokai",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 665f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 375f32,
            per_level: 43f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.8f32,
            per_level: 2.125f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694999992847442f32,
        attack_range: 125f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.85f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        maokai_q_1, maokai_q_2, maokai_q_3, maokai_w_1, maokai_e_1, maokai_e_2,
        maokai_e_3, maokai_r_1,
    ],
};
pub const fn maokai_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 120f32,
        2 => 130f32,
        3 => 140f32,
        4 => 150f32,
        5 => 160f32,
        _ => 0.0,
    }
}
pub const fn maokai_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + 0.02f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        2 => 110f32 + 0.025f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        3 => 155f32 + 0.03f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        4 => 200f32 + 0.035f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        5 => 245f32 + 0.04f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn maokai_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 185f32 + 0.02f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        2 => 240f32 + 0.025f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        3 => 295f32 + 0.03f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        4 => 350f32 + 0.035f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        5 => 405f32 + 0.04f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn maokai_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.4f32 * ctx.ap,
        2 => 85f32 + 0.4f32 * ctx.ap,
        3 => 110f32 + 0.4f32 * ctx.ap,
        4 => 135f32 + 0.4f32 * ctx.ap,
        5 => 160f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn maokai_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => {
            33.33f32
                + 0.0333f32 * ctx.bonus_health
                + 0.16670000000000001f32 * ctx.ap
        }
        2 => {
            50f32
                + 0.0333f32 * ctx.bonus_health
                + 0.16670000000000001f32 * ctx.ap
        }
        3 => {
            66.67f32
                + 0.0333f32 * ctx.bonus_health
                + 0.16670000000000001f32 * ctx.ap
        }
        4 => {
            83.33f32
                + 0.0333f32 * ctx.bonus_health
                + 0.16670000000000001f32 * ctx.ap
        }
        5 => {
            100f32
                + 0.0333f32 * ctx.bonus_health
                + 0.16670000000000001f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn maokai_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 100f32 + 0.1f32 * ctx.bonus_health + 0.5f32 * ctx.ap,
        2 => 150f32 + 0.1f32 * ctx.bonus_health + 0.5f32 * ctx.ap,
        3 => 200f32 + 0.1f32 * ctx.bonus_health + 0.5f32 * ctx.ap,
        4 => 250f32 + 0.1f32 * ctx.bonus_health + 0.5f32 * ctx.ap,
        5 => 300f32 + 0.1f32 * ctx.bonus_health + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn maokai_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.05f32 * ctx.bonus_health + 0.25f32 * ctx.ap,
        2 => 75f32 + 0.05f32 * ctx.bonus_health + 0.25f32 * ctx.ap,
        3 => 100f32 + 0.05f32 * ctx.bonus_health + 0.25f32 * ctx.ap,
        4 => 125f32 + 0.05f32 * ctx.bonus_health + 0.25f32 * ctx.ap,
        5 => 150f32 + 0.05f32 * ctx.bonus_health + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn maokai_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.75f32 * ctx.ap,
        2 => 225f32 + 0.75f32 * ctx.ap,
        3 => 300f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MASTERYI: CachedChampion = CachedChampion {
    name: "Master Yi",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_7),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 669f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 251f32,
            per_level: 42f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 65f32,
            per_level: 2.8f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.679f32,
            per_level: 2.5f32,
        },
        movespeed: 355f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.67900002002716f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        masteryi_q_1,
        masteryi_q_2,
        masteryi_q_3,
        masteryi_q_4,
        masteryi_q_5,
        masteryi_q_6,
        masteryi_q_7,
        masteryi_w_1,
        masteryi_w_2,
        masteryi_e_1,
    ],
};
pub const fn masteryi_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.7f32 * ctx.ad,
        2 => 125f32 + 0.7f32 * ctx.ad,
        3 => 170f32 + 0.7f32 * ctx.ad,
        4 => 215f32 + 0.7f32 * ctx.ad,
        5 => 260f32 + 0.7f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn masteryi_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 140f32 + 1.225f32 * ctx.ad,
        2 => 218.75f32 + 1.225f32 * ctx.ad,
        3 => 297.5f32 + 1.225f32 * ctx.ad,
        4 => 376.25f32 + 1.225f32 * ctx.ad,
        5 => 455f32 + 1.225f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn masteryi_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + 1.225f32 * ctx.ad,
        2 => 70f32 + 1.225f32 * ctx.ad,
        3 => 105f32 + 1.225f32 * ctx.ad,
        4 => 140f32 + 1.225f32 * ctx.ad,
        5 => 175f32 + 1.225f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn masteryi_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32,
        2 => 85f32,
        3 => 110f32,
        4 => 135f32,
        5 => 160f32,
        _ => 0.0,
    }
}
pub const fn masteryi_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 0.7f32 * ctx.ad,
        2 => 40f32 + 0.7f32 * ctx.ad,
        3 => 60f32 + 0.7f32 * ctx.ad,
        4 => 80f32 + 0.7f32 * ctx.ad,
        5 => 100f32 + 0.7f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn masteryi_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.175f32 * ctx.ad,
        2 => 10f32 + 0.175f32 * ctx.ad,
        3 => 15f32 + 0.175f32 * ctx.ad,
        4 => 20f32 + 0.175f32 * ctx.ad,
        5 => 25f32 + 0.175f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn masteryi_q_7(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 0.175f32 * ctx.ad,
        2 => 31.25f32 + 0.175f32 * ctx.ad,
        3 => 42.5f32 + 0.175f32 * ctx.ad,
        4 => 53.75f32 + 0.175f32 * ctx.ad,
        5 => 65f32 + 0.175f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn masteryi_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.45f32,
        2 => 0.475f32,
        3 => 0.5f32,
        4 => 0.525f32,
        5 => 0.55f32,
        _ => 0.0,
    }
}
pub const fn masteryi_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.225f32,
        2 => 0.2375f32,
        3 => 0.25f32,
        4 => 0.2625f32,
        5 => 0.275f32,
        _ => 0.0,
    }
}
pub const fn masteryi_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.35f32 * ctx.bonus_ad,
        2 => 25f32 + 0.35f32 * ctx.bonus_ad,
        3 => 30f32 + 0.35f32 * ctx.bonus_ad,
        4 => 35f32 + 0.35f32 * ctx.bonus_ad,
        5 => 40f32 + 0.35f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static MEL: CachedChampion = CachedChampion {
    name: "Mel",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 93f32,
        },
        mana: CachedChampionStatsMap {
            flat: 480f32,
            per_level: 28f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.4f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        mel_q_1, mel_q_2, mel_q_3, mel_q_4, mel_w_1, mel_e_1, mel_e_2, mel_e_3,
        mel_e_4, mel_e_5, mel_e_6, mel_r_1, mel_r_2, mel_r_3,
    ],
};
pub const fn mel_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 13f32 + 0.085f32 * ctx.ap,
        2 => 15.5f32 + 0.085f32 * ctx.ap,
        3 => 18f32 + 0.085f32 * ctx.ap,
        4 => 20.5f32 + 0.085f32 * ctx.ap,
        5 => 23f32 + 0.085f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 9.75f32 + 0.06375f32 * ctx.ap,
        2 => 11.625f32 + 0.06375f32 * ctx.ap,
        3 => 13.5f32 + 0.06375f32 * ctx.ap,
        4 => 15.375f32 + 0.06375f32 * ctx.ap,
        5 => 17.25f32 + 0.06375f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 78f32 + 0.51f32 * ctx.ap,
        2 => 108.5f32 + 0.595f32 * ctx.ap,
        3 => 144f32 + 0.68f32 * ctx.ap,
        4 => 184.5f32 + 0.765f32 * ctx.ap,
        5 => 230f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 58.5f32 + 0.3825f32 * ctx.ap,
        2 => 81.375f32 + 0.44625f32 * ctx.ap,
        3 => 108f32 + 0.51f32 * ctx.ap,
        4 => 138.375f32 + 0.57375f32 * ctx.ap,
        5 => 172.5f32 + 0.6375f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.4f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,
        2 => 0.45f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,
        3 => 0.5f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,
        4 => 0.55f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,
        5 => 0.6f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 105f32 + 0.6f32 * ctx.ap,
        3 => 150f32 + 0.6f32 * ctx.ap,
        4 => 195f32 + 0.6f32 * ctx.ap,
        5 => 240f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 8f32 + 0.04f32 * ctx.ap,
        2 => 16f32 + 0.04f32 * ctx.ap,
        3 => 24f32 + 0.04f32 * ctx.ap,
        4 => 32f32 + 0.04f32 * ctx.ap,
        5 => 40f32 + 0.04f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 1f32 + 0.005f32 * ctx.ap,
        2 => 2f32 + 0.005f32 * ctx.ap,
        3 => 3f32 + 0.005f32 * ctx.ap,
        4 => 4f32 + 0.005f32 * ctx.ap,
        5 => 5f32 + 0.005f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 30f32 + 0.3f32 * ctx.ap,
        2 => 52.5f32 + 0.3f32 * ctx.ap,
        3 => 75f32 + 0.3f32 * ctx.ap,
        4 => 97.5f32 + 0.3f32 * ctx.ap,
        5 => 120f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_e_5(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 16f32 + 0.08f32 * ctx.ap,
        2 => 32f32 + 0.08f32 * ctx.ap,
        3 => 48f32 + 0.08f32 * ctx.ap,
        4 => 64f32 + 0.08f32 * ctx.ap,
        5 => 80f32 + 0.08f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_e_6(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 2f32 + 0.01f32 * ctx.ap,
        2 => 4f32 + 0.01f32 * ctx.ap,
        3 => 6f32 + 0.01f32 * ctx.ap,
        4 => 8f32 + 0.01f32 * ctx.ap,
        5 => 10f32 + 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn mel_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.3f32 * ctx.ap + 4f32,
        2 => 150f32 + 0.3f32 * ctx.ap + 7f32,
        3 => 200f32 + 0.3f32 * ctx.ap + 10f32,
        _ => 0.0,
    }
}
pub const fn mel_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 10f32,
        2 => 20f32,
        3 => 30f32,
        _ => 0.0,
    }
}
pub const fn mel_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 1f32,
        2 => 2f32,
        3 => 3f32,
        _ => 0.0,
    }
}
pub static MILIO: CachedChampion = CachedChampion {
    name: "Milio",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[TypeMetadata {
        kind: AbilityId::Q(AbilityName::_1),
        damage_type: DamageType::Magic,
        attributes: Attrs::Undefined,
    }],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 560f32,
            per_level: 88f32,
        },
        mana: CachedChampionStatsMap {
            flat: 365f32,
            per_level: 43f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 48f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[milio_q_1],
};
pub const fn milio_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 1.2f32 * ctx.ap,
        2 => 140f32 + 1.2f32 * ctx.ap,
        3 => 200f32 + 1.2f32 * ctx.ap,
        4 => 260f32 + 1.2f32 * ctx.ap,
        5 => 320f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MISSFORTUNE: CachedChampion = CachedChampion {
    name: "Miss Fortune",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 53f32,
            per_level: 2.4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.656f32,
            per_level: 3f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.656000018119812f32,
        attack_range: 550f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        missfortune_q_1,
        missfortune_e_1,
        missfortune_e_2,
        missfortune_r_1,
    ],
};
pub const fn missfortune_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + ctx.ad + 0.35f32 * ctx.ap,
        2 => 45f32 + ctx.ad + 0.35f32 * ctx.ap,
        3 => 70f32 + ctx.ad + 0.35f32 * ctx.ap,
        4 => 95f32 + ctx.ad + 0.35f32 * ctx.ap,
        5 => 120f32 + ctx.ad + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn missfortune_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 8.75f32 + 0.15f32 * ctx.ap,
        2 => 12.5f32 + 0.15f32 * ctx.ap,
        3 => 16.25f32 + 0.15f32 * ctx.ap,
        4 => 20f32 + 0.15f32 * ctx.ap,
        5 => 23.75f32 + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn missfortune_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 1.2f32 * ctx.ap,
        2 => 100f32 + 1.2f32 * ctx.ap,
        3 => 130f32 + 1.2f32 * ctx.ap,
        4 => 160f32 + 1.2f32 * ctx.ap,
        5 => 190f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn missfortune_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 10.5f32 * ctx.ad + 3.5f32 * ctx.ap,
        2 => 12f32 * ctx.ad + 4f32 * ctx.ap,
        3 => 13.5f32 * ctx.ad + 4.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MONKEYKING: CachedChampion = CachedChampion {
    name: "Wukong",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 330f32,
            per_level: 65f32,
        },
        armor: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.69f32,
            per_level: 3f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        monkeyking_q_1,
        monkeyking_w_1,
        monkeyking_e_1,
        monkeyking_r_1,
        monkeyking_r_2,
        monkeyking_r_3,
    ],
};
pub const fn monkeyking_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 0.55f32 * ctx.bonus_ad,
        2 => 45f32 + 0.55f32 * ctx.bonus_ad,
        3 => 70f32 + 0.55f32 * ctx.bonus_ad,
        4 => 95f32 + 0.55f32 * ctx.bonus_ad,
        5 => 120f32 + 0.55f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn monkeyking_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.35f32,
        2 => 0.4f32,
        3 => 0.45f32,
        4 => 0.5f32,
        5 => 0.55f32,
        _ => 0.0,
    }
}
pub const fn monkeyking_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + ctx.ap,
        2 => 110f32 + ctx.ap,
        3 => 140f32 + ctx.ap,
        4 => 170f32 + ctx.ap,
        5 => 200f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn monkeyking_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.01f32 * ctx.enemy_max_health + 0.34375f32 * ctx.ad,
        2 => 0.015f32 * ctx.enemy_max_health + 0.34375f32 * ctx.ad,
        3 => 0.02f32 * ctx.enemy_max_health + 0.34375f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn monkeyking_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.08f32 * ctx.enemy_max_health + 2.75f32 * ctx.ad,
        2 => 0.12f32 * ctx.enemy_max_health + 2.75f32 * ctx.ad,
        3 => 0.16f32 * ctx.enemy_max_health + 2.75f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn monkeyking_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.16f32 * ctx.enemy_max_health + 5.5f32 * ctx.ad,
        2 => 0.24f32 * ctx.enemy_max_health + 5.5f32 * ctx.ad,
        3 => 0.32f32 * ctx.enemy_max_health + 5.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static MORDEKAISER: CachedChampion = CachedChampion {
    name: "Mordekaiser",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 645f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 1.02f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[mordekaiser_q_1, mordekaiser_q_2, mordekaiser_e_1],
};
pub const fn mordekaiser_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.3f32,
        2 => 0.35f32,
        3 => 0.4f32,
        4 => 0.45f32,
        5 => 0.5f32,
        _ => 0.0,
    }
}
pub const fn mordekaiser_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0f32 + 80f32 + 0.7f32 * ctx.ap,
        2 => 2.6470588235294117f32 + 115f32 + 0.7f32 * ctx.ap,
        3 => 5.294117647058823f32 + 150f32 + 0.7f32 * ctx.ap,
        4 => 7.9411764705882355f32 + 185f32 + 0.7f32 * ctx.ap,
        5 => 10.588235294117649f32 + 220f32 + 0.7f32 * ctx.ap,
        6 => 13.235294117647058f32,
        7 => 15.882352941176473f32,
        8 => 18.52941176470588f32,
        9 => 21.176470588235293f32,
        10 => 23.823529411764707f32,
        11 => 26.470588235294116f32,
        12 => 29.11764705882353f32,
        13 => 31.764705882352946f32,
        14 => 34.411764705882355f32,
        15 => 37.05882352941176f32,
        16 => 39.705882352941174f32,
        17 => 42.35294117647059f32,
        18 => 45f32,
        _ => 0.0,
    }
}
pub const fn mordekaiser_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.4f32 * ctx.ap,
        2 => 75f32 + 0.4f32 * ctx.ap,
        3 => 90f32 + 0.4f32 * ctx.ap,
        4 => 105f32 + 0.4f32 * ctx.ap,
        5 => 120f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static MORGANA: CachedChampion = CachedChampion {
    name: "Morgana",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 340f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.53f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 450f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        morgana_q_1,
        morgana_w_1,
        morgana_w_2,
        morgana_w_3,
        morgana_w_4,
        morgana_r_1,
        morgana_r_2,
    ],
};
pub const fn morgana_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.9f32 * ctx.ap,
        2 => 135f32 + 0.9f32 * ctx.ap,
        3 => 190f32 + 0.9f32 * ctx.ap,
        4 => 245f32 + 0.9f32 * ctx.ap,
        5 => 300f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn morgana_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 16.2f32 + 0.22949999999999998f32 * ctx.ap,
        2 => 31.05f32 + 0.22949999999999998f32 * ctx.ap,
        3 => 45.9f32 + 0.22949999999999998f32 * ctx.ap,
        4 => 60.75f32 + 0.22949999999999998f32 * ctx.ap,
        5 => 75.6f32 + 0.22949999999999998f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn morgana_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 162f32 + 2.295f32 * ctx.ap,
        2 => 310.5f32 + 2.295f32 * ctx.ap,
        3 => 459f32 + 2.295f32 * ctx.ap,
        4 => 607.5f32 + 2.295f32 * ctx.ap,
        5 => 756f32 + 2.295f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn morgana_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 6f32 + 0.085f32 * ctx.ap,
        2 => 11.5f32 + 0.085f32 * ctx.ap,
        3 => 17f32 + 0.085f32 * ctx.ap,
        4 => 22.5f32 + 0.085f32 * ctx.ap,
        5 => 28f32 + 0.085f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn morgana_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.85f32 * ctx.ap,
        2 => 115f32 + 0.85f32 * ctx.ap,
        3 => 170f32 + 0.85f32 * ctx.ap,
        4 => 225f32 + 0.85f32 * ctx.ap,
        5 => 280f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn morgana_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.8f32 * ctx.ap,
        2 => 275f32 + 0.8f32 * ctx.ap,
        3 => 350f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn morgana_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 400f32 + 1.6f32 * ctx.ap,
        2 => 550f32 + 1.6f32 * ctx.ap,
        3 => 700f32 + 1.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static NAAFIRI: CachedChampion = CachedChampion {
    name: "Naafiri",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_7),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_8),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 55f32,
        },
        armor: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.663f32,
            per_level: 2.1f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        naafiri_q_1,
        naafiri_q_2,
        naafiri_q_3,
        naafiri_q_4,
        naafiri_q_5,
        naafiri_q_6,
        naafiri_q_7,
        naafiri_q_8,
        naafiri_e_1,
        naafiri_e_2,
        naafiri_e_3,
        naafiri_r_1,
        naafiri_r_2,
    ],
};
pub const fn naafiri_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 3.5f32 + 0.08f32 * ctx.bonus_ad,
        2 => 6f32 + 0.08f32 * ctx.bonus_ad,
        3 => 8.5f32 + 0.08f32 * ctx.bonus_ad,
        4 => 11f32 + 0.08f32 * ctx.bonus_ad,
        5 => 13.5f32 + 0.08f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + 0.2f32 * ctx.bonus_ad,
        2 => 40f32 + 0.2f32 * ctx.bonus_ad,
        3 => 45f32 + 0.2f32 * ctx.bonus_ad,
        4 => 50f32 + 0.2f32 * ctx.bonus_ad,
        5 => 55f32 + 0.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + 0.8f32 * ctx.bonus_ad,
        2 => 60f32 + 0.8f32 * ctx.bonus_ad,
        3 => 85f32 + 0.8f32 * ctx.bonus_ad,
        4 => 110f32 + 0.8f32 * ctx.bonus_ad,
        5 => 135f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + ctx.bonus_ad,
        2 => 100f32 + ctx.bonus_ad,
        3 => 130f32 + ctx.bonus_ad,
        4 => 160f32 + ctx.bonus_ad,
        5 => 190f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 1.4f32 * ctx.bonus_ad,
        2 => 90f32 + 1.4f32 * ctx.bonus_ad,
        3 => 120f32 + 1.4f32 * ctx.bonus_ad,
        4 => 150f32 + 1.4f32 * ctx.bonus_ad,
        5 => 180f32 + 1.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 130f32 + 2.4f32 * ctx.bonus_ad,
        2 => 190f32 + 2.4f32 * ctx.bonus_ad,
        3 => 250f32 + 2.4f32 * ctx.bonus_ad,
        4 => 310f32 + 2.4f32 * ctx.bonus_ad,
        5 => 370f32 + 2.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_7(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.4f32 * ctx.bonus_ad,
        2 => 45f32 + 0.4f32 * ctx.bonus_ad,
        3 => 60f32 + 0.4f32 * ctx.bonus_ad,
        4 => 75f32 + 0.4f32 * ctx.bonus_ad,
        5 => 90f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_q_8(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 100f32 + 1.4f32 * ctx.bonus_ad,
        2 => 145f32 + 1.4f32 * ctx.bonus_ad,
        3 => 190f32 + 1.4f32 * ctx.bonus_ad,
        4 => 235f32 + 1.4f32 * ctx.bonus_ad,
        5 => 280f32 + 1.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 15f32 + 0.4f32 * ctx.bonus_ad,
        2 => 25f32 + 0.4f32 * ctx.bonus_ad,
        3 => 35f32 + 0.4f32 * ctx.bonus_ad,
        4 => 45f32 + 0.4f32 * ctx.bonus_ad,
        5 => 55f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.8f32 * ctx.bonus_ad,
        2 => 85f32 + 0.8f32 * ctx.bonus_ad,
        3 => 110f32 + 0.8f32 * ctx.bonus_ad,
        4 => 135f32 + 0.8f32 * ctx.bonus_ad,
        5 => 160f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32 + 1.2f32 * ctx.bonus_ad,
        2 => 110f32 + 1.2f32 * ctx.bonus_ad,
        3 => 145f32 + 1.2f32 * ctx.bonus_ad,
        4 => 180f32 + 1.2f32 * ctx.bonus_ad,
        5 => 215f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 1.2f32 * ctx.bonus_ad,
        2 => 250f32 + 1.2f32 * ctx.bonus_ad,
        3 => 350f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn naafiri_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 15f32 + 0.12f32 * ctx.bonus_ad,
        2 => 25f32 + 0.12f32 * ctx.bonus_ad,
        3 => 35f32 + 0.12f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static NAMI: CachedChampion = CachedChampion {
    name: "Nami",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 560f32,
            per_level: 88f32,
        },
        mana: CachedChampionStatsMap {
            flat: 365f32,
            per_level: 43f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 2.61f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 550f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[nami_q_1, nami_w_1, nami_w_2, nami_e_1, nami_e_2, nami_r_1],
};
pub const fn nami_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 0.5f32 * ctx.ap,
        2 => 145f32 + 0.5f32 * ctx.ap,
        3 => 200f32 + 0.5f32 * ctx.ap,
        4 => 255f32 + 0.5f32 * ctx.ap,
        5 => 310f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nami_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.5f32 * ctx.ap,
        2 => 95f32 + 0.5f32 * ctx.ap,
        3 => 130f32 + 0.5f32 * ctx.ap,
        4 => 165f32 + 0.5f32 * ctx.ap,
        5 => 200f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nami_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 48f32 + 0.4f32 * ctx.ap,
        2 => 76f32 + 0.4f32 * ctx.ap,
        3 => 104f32 + 0.4f32 * ctx.ap,
        4 => 132f32 + 0.4f32 * ctx.ap,
        5 => 160f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nami_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.2f32 * ctx.ap,
        2 => 30f32 + 0.2f32 * ctx.ap,
        3 => 40f32 + 0.2f32 * ctx.ap,
        4 => 50f32 + 0.2f32 * ctx.ap,
        5 => 60f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nami_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 90f32 + 0.6f32 * ctx.ap,
        3 => 120f32 + 0.6f32 * ctx.ap,
        4 => 150f32 + 0.6f32 * ctx.ap,
        5 => 180f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nami_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.6f32 * ctx.ap,
        2 => 250f32 + 0.6f32 * ctx.ap,
        3 => 350f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static NASUS: CachedChampion = CachedChampion {
    name: "Nasus",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 631f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 326f32,
            per_level: 62f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 67f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 3.48f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        nasus_q_1, nasus_e_1, nasus_e_2, nasus_e_3, nasus_r_1, nasus_r_2,
    ],
};
pub const fn nasus_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + ctx.nasus_stacks,
        2 => 55f32 + ctx.nasus_stacks,
        3 => 75f32 + ctx.nasus_stacks,
        4 => 95f32 + ctx.nasus_stacks,
        5 => 115f32 + ctx.nasus_stacks,
        _ => 0.0,
    }
}
pub const fn nasus_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.6f32 * ctx.ap,
        2 => 80f32 + 0.6f32 * ctx.ap,
        3 => 110f32 + 0.6f32 * ctx.ap,
        4 => 140f32 + 0.6f32 * ctx.ap,
        5 => 170f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nasus_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 10f32 + 0.12f32 * ctx.ap,
        2 => 16f32 + 0.12f32 * ctx.ap,
        3 => 22f32 + 0.12f32 * ctx.ap,
        4 => 28f32 + 0.12f32 * ctx.ap,
        5 => 34f32 + 0.12f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nasus_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 100f32 + 1.2f32 * ctx.ap,
        2 => 160f32 + 1.2f32 * ctx.ap,
        3 => 220f32 + 1.2f32 * ctx.ap,
        4 => 280f32 + 1.2f32 * ctx.ap,
        5 => 340f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nasus_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.015f32 * ctx.enemy_max_health + 0.005f32 * 0.01f32 * ctx.ap,
        2 => 0.02f32 * ctx.enemy_max_health + 0.005f32 * 0.01f32 * ctx.ap,
        3 => 0.025f32 * ctx.enemy_max_health + 0.005f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nasus_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.45f32 * ctx.enemy_max_health + 0.15f32 * 0.01f32 * ctx.ap,
        2 => 0.6f32 * ctx.enemy_max_health + 0.15f32 * 0.01f32 * ctx.ap,
        3 => 0.75f32 * ctx.enemy_max_health + 0.15f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static NAUTILUS: CachedChampion = CachedChampion {
    name: "Nautilus",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 646f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 47f32,
        },
        armor: CachedChampionStatsMap {
            flat: 39f32,
            per_level: 4.95f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.706f32,
            per_level: 1f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.611999988555908f32,
        attack_range: 175f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        nautilus_q_1,
        nautilus_w_1,
        nautilus_e_1,
        nautilus_e_2,
        nautilus_e_3,
        nautilus_e_4,
        nautilus_e_5,
        nautilus_e_6,
        nautilus_r_1,
        nautilus_r_2,
    ],
};
pub const fn nautilus_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 85f32 + 0.9f32 * ctx.ap,
        2 => 130f32 + 0.9f32 * ctx.ap,
        3 => 175f32 + 0.9f32 * ctx.ap,
        4 => 220f32 + 0.9f32 * ctx.ap,
        5 => 265f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.4f32 * ctx.ap,
        2 => 40f32 + 0.4f32 * ctx.ap,
        3 => 50f32 + 0.4f32 * ctx.ap,
        4 => 60f32 + 0.4f32 * ctx.ap,
        5 => 70f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 0.5f32 * ctx.ap,
        2 => 90f32 + 0.5f32 * ctx.ap,
        3 => 125f32 + 0.5f32 * ctx.ap,
        4 => 160f32 + 0.5f32 * ctx.ap,
        5 => 195f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 110f32 + ctx.ap,
        2 => 180f32 + ctx.ap,
        3 => 250f32 + ctx.ap,
        4 => 320f32 + ctx.ap,
        5 => 390f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 27.5f32 + 0.25f32 * ctx.ap,
        2 => 45f32 + 0.25f32 * ctx.ap,
        3 => 62.5f32 + 0.25f32 * ctx.ap,
        4 => 80f32 + 0.25f32 * ctx.ap,
        5 => 97.5f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 165f32 + 1.5f32 * ctx.ap,
        2 => 270f32 + 1.5f32 * ctx.ap,
        3 => 375f32 + 1.5f32 * ctx.ap,
        4 => 480f32 + 1.5f32 * ctx.ap,
        5 => 585f32 + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_e_5(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 82.5f32 + 0.75f32 * ctx.ap,
        2 => 135f32 + 0.75f32 * ctx.ap,
        3 => 187.5f32 + 0.75f32 * ctx.ap,
        4 => 240f32 + 0.75f32 * ctx.ap,
        5 => 292.5f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_e_6(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 41.25f32 + 0.375f32 * ctx.ap,
        2 => 67.5f32 + 0.375f32 * ctx.ap,
        3 => 93.75f32 + 0.375f32 * ctx.ap,
        4 => 120f32 + 0.375f32 * ctx.ap,
        5 => 146.25f32 + 0.375f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.4f32 * ctx.ap,
        2 => 175f32 + 0.4f32 * ctx.ap,
        3 => 225f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nautilus_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.8f32 * ctx.ap,
        2 => 275f32 + 0.8f32 * ctx.ap,
        3 => 400f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static NEEKO: CachedChampion = CachedChampion {
    name: "Neeko",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 450f32,
            per_level: 30f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 48f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.67f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        neeko_q_1, neeko_q_2, neeko_q_3, neeko_q_4, neeko_w_1, neeko_e_1,
        neeko_r_1,
    ],
};
pub const fn neeko_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 110f32 + 0.6f32 * ctx.ap,
        3 => 160f32 + 0.6f32 * ctx.ap,
        4 => 210f32 + 0.6f32 * ctx.ap,
        5 => 260f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn neeko_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32,
        2 => 50f32,
        3 => 65f32,
        4 => 80f32,
        5 => 95f32,
        _ => 0.0,
    }
}
pub const fn neeko_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + 0.25f32 * ctx.ap,
        2 => 60f32 + 0.25f32 * ctx.ap,
        3 => 85f32 + 0.25f32 * ctx.ap,
        4 => 110f32 + 0.25f32 * ctx.ap,
        5 => 135f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn neeko_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 130f32 + 1.1f32 * ctx.ap,
        2 => 230f32 + 1.1f32 * ctx.ap,
        3 => 330f32 + 1.1f32 * ctx.ap,
        4 => 430f32 + 1.1f32 * ctx.ap,
        5 => 530f32 + 1.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn neeko_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.6f32 * ctx.ap,
        2 => 65f32 + 0.6f32 * ctx.ap,
        3 => 100f32 + 0.6f32 * ctx.ap,
        4 => 135f32 + 0.6f32 * ctx.ap,
        5 => 170f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn neeko_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.65f32 * ctx.ap,
        2 => 105f32 + 0.65f32 * ctx.ap,
        3 => 140f32 + 0.65f32 * ctx.ap,
        4 => 175f32 + 0.65f32 * ctx.ap,
        5 => 210f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn neeko_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 1.2f32 * ctx.ap,
        2 => 350f32 + 1.2f32 * ctx.ap,
        3 => 550f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static NIDALEE: CachedChampion = CachedChampion {
    name: "Nidalee",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 295f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.45f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 3.22f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        nidalee_q_3,
        nidalee_q_4,
        nidalee_w_2,
        nidalee_w_3,
        nidalee_e_1,
    ],
};
pub const fn nidalee_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 227.5f32 + 1.625f32 * ctx.ap,
        2 => 292.5f32,
        3 => 357.5f32,
        4 => 422.5f32,
        5 => 487.5f32,
        _ => 0.0,
    }
}
pub const fn nidalee_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.5f32 * ctx.ap,
        2 => 90f32,
        3 => 110f32,
        4 => 130f32,
        5 => 150f32,
        _ => 0.0,
    }
}
pub const fn nidalee_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.2f32 * ctx.ap,
        2 => 80f32,
        3 => 120f32,
        4 => 160f32,
        5 => 200f32,
        _ => 0.0,
    }
}
pub const fn nidalee_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.05f32 * ctx.ap,
        2 => 20f32,
        3 => 30f32,
        4 => 40f32,
        5 => 50f32,
        _ => 0.0,
    }
}
pub const fn nidalee_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.3f32,
        2 => 0.4f32,
        3 => 0.5f32,
        4 => 0.6f32,
        5 => 0.7f32,
        _ => 0.0,
    }
}
pub static NILAH: CachedChampion = CachedChampion {
    name: "Nilah",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 570f32,
            per_level: 101f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.697f32,
            per_level: 2.25f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.67f32,
        attack_range: 225f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        nilah_q_1, nilah_q_2, nilah_e_1, nilah_r_1, nilah_r_2, nilah_r_3,
        nilah_r_4,
    ],
};
pub const fn nilah_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 1.8f32 * ctx.ad,
        2 => 20f32 + 1.9f32 * ctx.ad,
        3 => 30f32 + 2f32 * ctx.ad,
        4 => 40f32 + 2.1f32 * ctx.ad,
        5 => 50f32 + 2.2f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn nilah_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.9f32 * ctx.ad,
        2 => 10f32 + 0.95f32 * ctx.ad,
        3 => 15f32 + ctx.ad,
        4 => 20f32 + 1.05f32 * ctx.ad,
        5 => 25f32 + 1.1f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn nilah_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.2f32 * ctx.bonus_ad,
        2 => 70f32 + 0.2f32 * ctx.bonus_ad,
        3 => 80f32 + 0.2f32 * ctx.bonus_ad,
        4 => 90f32 + 0.2f32 * ctx.bonus_ad,
        5 => 100f32 + 0.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn nilah_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + ctx.bonus_ad,
        2 => 225f32 + ctx.bonus_ad,
        3 => 325f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn nilah_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 185f32 + 1.4f32 * ctx.bonus_ad,
        2 => 325f32 + 1.4f32 * ctx.bonus_ad,
        3 => 465f32 + 1.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn nilah_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 15f32 + 0.1f32 * ctx.bonus_ad,
        2 => 25f32 + 0.1f32 * ctx.bonus_ad,
        3 => 35f32 + 0.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn nilah_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 60f32 + 0.4f32 * ctx.bonus_ad,
        2 => 100f32 + 0.4f32 * ctx.bonus_ad,
        3 => 140f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static NOCTURNE: CachedChampion = CachedChampion {
    name: "Nocturne",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 275f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 2.6f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.721f32,
            per_level: 2.7f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.721f32,
        attack_range: 125f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        nocturne_q_1,
        nocturne_q_2,
        nocturne_e_1,
        nocturne_e_2,
        nocturne_r_1,
    ],
};
pub const fn nocturne_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + 0.85f32 * ctx.bonus_ad,
        2 => 110f32 + 0.85f32 * ctx.bonus_ad,
        3 => 155f32 + 0.85f32 * ctx.bonus_ad,
        4 => 200f32 + 0.85f32 * ctx.bonus_ad,
        5 => 245f32 + 0.85f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn nocturne_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32,
        2 => 30f32,
        3 => 40f32,
        4 => 50f32,
        5 => 60f32,
        _ => 0.0,
    }
}
pub const fn nocturne_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.25f32 * ctx.ap,
        2 => 31.25f32 + 0.25f32 * ctx.ap,
        3 => 42.5f32 + 0.25f32 * ctx.ap,
        4 => 53.75f32 + 0.25f32 * ctx.ap,
        5 => 65f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nocturne_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + ctx.ap,
        2 => 125f32 + ctx.ap,
        3 => 170f32 + ctx.ap,
        4 => 215f32 + ctx.ap,
        5 => 260f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn nocturne_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 1.2f32 * ctx.bonus_ad,
        2 => 275f32 + 1.2f32 * ctx.bonus_ad,
        3 => 400f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static NUNU: CachedChampion = CachedChampion {
    name: "Nunu & Willump",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 90f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 42f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.25f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1.08f32,
    },
    merge_data: &[],
    closures: &[
        nunu_q_1, nunu_q_2, nunu_w_1, nunu_w_2, nunu_w_3, nunu_w_4, nunu_e_1,
        nunu_e_2, nunu_e_3, nunu_e_4, nunu_r_1,
    ],
};
pub const fn nunu_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.65f32 * ctx.ap + 0.05f32 * ctx.bonus_health,
        2 => 100f32 + 0.65f32 * ctx.ap + 0.05f32 * ctx.bonus_health,
        3 => 140f32 + 0.65f32 * ctx.ap + 0.05f32 * ctx.bonus_health,
        4 => 180f32 + 0.65f32 * ctx.ap + 0.05f32 * ctx.bonus_health,
        5 => 220f32 + 0.65f32 * ctx.ap + 0.05f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn nunu_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 400f32,
        2 => 600f32,
        3 => 800f32,
        4 => 1000f32,
        5 => 1200f32,
        _ => 0.0,
    }
}
pub const fn nunu_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 180f32 + 1.5f32 * ctx.ap,
        2 => 225f32 + 1.5f32 * ctx.ap,
        3 => 270f32 + 1.5f32 * ctx.ap,
        4 => 315f32 + 1.5f32 * ctx.ap,
        5 => 360f32 + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 36f32 + 0.3f32 * ctx.ap,
        2 => 45f32 + 0.3f32 * ctx.ap,
        3 => 54f32 + 0.3f32 * ctx.ap,
        4 => 63f32 + 0.3f32 * ctx.ap,
        5 => 72f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 59.94f32 + 0.49950000000000006f32 * ctx.ap,
        2 => 74.925f32 + 0.49950000000000006f32 * ctx.ap,
        3 => 89.91f32 + 0.49950000000000006f32 * ctx.ap,
        4 => 104.895f32 + 0.49950000000000006f32 * ctx.ap,
        5 => 119.88f32 + 0.49950000000000006f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 11.988f32 + 0.0999f32 * ctx.ap,
        2 => 14.985f32 + 0.0999f32 * ctx.ap,
        3 => 17.982f32 + 0.0999f32 * ctx.ap,
        4 => 20.979f32 + 0.0999f32 * ctx.ap,
        5 => 23.976f32 + 0.0999f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 16f32 + 0.15f32 * ctx.ap,
        2 => 24f32 + 0.15f32 * ctx.ap,
        3 => 32f32 + 0.15f32 * ctx.ap,
        4 => 40f32 + 0.15f32 * ctx.ap,
        5 => 48f32 + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 48f32 + 0.45f32 * ctx.ap,
        2 => 72f32 + 0.45f32 * ctx.ap,
        3 => 96f32 + 0.45f32 * ctx.ap,
        4 => 120f32 + 0.45f32 * ctx.ap,
        5 => 144f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.8f32 * ctx.ap,
        2 => 30f32 + 0.8f32 * ctx.ap,
        3 => 40f32 + 0.8f32 * ctx.ap,
        4 => 50f32 + 0.8f32 * ctx.ap,
        5 => 60f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 144f32 + 1.35f32 * ctx.ap,
        2 => 216f32 + 1.35f32 * ctx.ap,
        3 => 288f32 + 1.35f32 * ctx.ap,
        4 => 360f32 + 1.35f32 * ctx.ap,
        5 => 432f32 + 1.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn nunu_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 625f32 + 3f32 * ctx.ap,
        2 => 950f32 + 3f32 * ctx.ap,
        3 => 1275f32 + 3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static OLAF: CachedChampion = CachedChampion {
    name: "Olaf",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 645f32,
            per_level: 119f32,
        },
        mana: CachedChampionStatsMap {
            flat: 316f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 4.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.694f32,
            per_level: 2.7f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694000005722045f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[olaf_q_1, olaf_q_2, olaf_q_3, olaf_e_1, olaf_r_1],
};
pub const fn olaf_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + ctx.bonus_ad,
        2 => 110f32 + ctx.bonus_ad,
        3 => 160f32 + ctx.bonus_ad,
        4 => 210f32 + ctx.bonus_ad,
        5 => 260f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn olaf_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32,
        2 => 25f32,
        3 => 40f32,
        4 => 55f32,
        5 => 70f32,
        _ => 0.0,
    }
}
pub const fn olaf_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + ctx.bonus_ad,
        2 => 135f32 + ctx.bonus_ad,
        3 => 200f32 + ctx.bonus_ad,
        4 => 265f32 + ctx.bonus_ad,
        5 => 330f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn olaf_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.5f32 * ctx.ad,
        2 => 115f32 + 0.5f32 * ctx.ad,
        3 => 160f32 + 0.5f32 * ctx.ad,
        4 => 205f32 + 0.5f32 * ctx.ad,
        5 => 250f32 + 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn olaf_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 10f32 + 0.25f32 * ctx.ad,
        2 => 20f32 + 0.25f32 * ctx.ad,
        3 => 30f32 + 0.25f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static ORIANNA: CachedChampion = CachedChampion {
    name: "Orianna",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 585f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 418f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 20f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 44f32,
            per_level: 2.6f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.5f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        orianna_q_1,
        orianna_q_2,
        orianna_w_1,
        orianna_e_1,
        orianna_r_1,
    ],
};
pub const fn orianna_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.55f32 * ctx.ap,
        2 => 90f32 + 0.55f32 * ctx.ap,
        3 => 120f32 + 0.55f32 * ctx.ap,
        4 => 150f32 + 0.55f32 * ctx.ap,
        5 => 180f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn orianna_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 42f32 + 0.385f32 * ctx.ap,
        2 => 63f32 + 0.385f32 * ctx.ap,
        3 => 84f32 + 0.385f32 * ctx.ap,
        4 => 105f32 + 0.385f32 * ctx.ap,
        5 => 126f32 + 0.385f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn orianna_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 120f32 + 0.7f32 * ctx.ap,
        3 => 170f32 + 0.7f32 * ctx.ap,
        4 => 220f32 + 0.7f32 * ctx.ap,
        5 => 270f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn orianna_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.3f32 * ctx.ap,
        2 => 90f32 + 0.3f32 * ctx.ap,
        3 => 120f32 + 0.3f32 * ctx.ap,
        4 => 150f32 + 0.3f32 * ctx.ap,
        5 => 180f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn orianna_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 250f32 + 0.95f32 * ctx.ap,
        2 => 400f32 + 0.95f32 * ctx.ap,
        3 => 550f32 + 0.95f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ORNN: CachedChampion = CachedChampion {
    name: "Ornn",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 660f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 341f32,
            per_level: 65f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 69f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[
        ornn_q_1, ornn_w_1, ornn_w_2, ornn_w_3, ornn_w_4, ornn_w_5, ornn_w_6,
        ornn_e_1, ornn_r_1, ornn_r_2,
    ],
};
pub const fn ornn_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 1.1f32 * ctx.ad,
        2 => 45f32 + 1.1f32 * ctx.ad,
        3 => 70f32 + 1.1f32 * ctx.ad,
        4 => 95f32 + 1.1f32 * ctx.ad,
        5 => 120f32 + 1.1f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn ornn_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 16f32,
        2 => 26f32,
        3 => 36f32,
        4 => 46f32,
        5 => 56f32,
        _ => 0.0,
    }
}
pub const fn ornn_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 31f32,
        2 => 36f32,
        3 => 41f32,
        4 => 46f32,
        5 => 51f32,
        _ => 0.0,
    }
}
pub const fn ornn_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32,
        2 => 130f32,
        3 => 180f32,
        4 => 230f32,
        5 => 280f32,
        _ => 0.0,
    }
}
pub const fn ornn_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 155f32,
        2 => 180f32,
        3 => 205f32,
        4 => 230f32,
        5 => 255f32,
        _ => 0.0,
    }
}
pub const fn ornn_w_5(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.024f32 * ctx.enemy_max_health,
        2 => 0.026000000000000002f32 * ctx.enemy_max_health,
        3 => 0.027999999999999997f32 * ctx.enemy_max_health,
        4 => 0.03f32 * ctx.enemy_max_health,
        5 => 0.032f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn ornn_w_6(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.12f32 * ctx.enemy_max_health,
        2 => 0.13f32 * ctx.enemy_max_health,
        3 => 0.14f32 * ctx.enemy_max_health,
        4 => 0.15f32 * ctx.enemy_max_health,
        5 => 0.16f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn ornn_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn ornn_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.2f32 * ctx.ap,
        2 => 175f32 + 0.2f32 * ctx.ap,
        3 => 225f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ornn_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 250f32 + 0.4f32 * ctx.ap,
        2 => 350f32 + 0.4f32 * ctx.ap,
        3 => 450f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static PANTHEON: CachedChampion = CachedChampion {
    name: "Pantheon",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[
        Position::Jungle,
        Position::Middle,
        Position::Support,
        Position::Top,
    ],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 317f32,
            per_level: 31f32,
        },
        armor: CachedChampionStatsMap {
            flat: 40f32,
            per_level: 4.95f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.95f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        pantheon_q_1,
        pantheon_q_2,
        pantheon_q_3,
        pantheon_q_4,
        pantheon_q_5,
        pantheon_q_6,
        pantheon_w_1,
        pantheon_e_1,
        pantheon_r_1,
        pantheon_r_2,
    ],
};
pub const fn pantheon_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 100f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 130f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 160f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 190f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn pantheon_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 35f32 + 0.575f32 * ctx.bonus_ad + 0.25f32 * ctx.ap,
        2 => 50f32 + 0.575f32 * ctx.bonus_ad + 0.25f32 * ctx.ap,
        3 => 65f32 + 0.575f32 * ctx.bonus_ad + 0.25f32 * ctx.ap,
        4 => 80f32 + 0.575f32 * ctx.bonus_ad + 0.25f32 * ctx.ap,
        5 => 95f32 + 0.575f32 * ctx.bonus_ad + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn pantheon_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 155f32 + 2.3f32 * ctx.bonus_ad + ctx.ap,
        2 => 230f32 + 2.3f32 * ctx.bonus_ad + ctx.ap,
        3 => 305f32 + 2.3f32 * ctx.bonus_ad + ctx.ap,
        4 => 380f32 + 2.3f32 * ctx.bonus_ad + ctx.ap,
        5 => 455f32 + 2.3f32 * ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn pantheon_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 77.5f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 115f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 152.5f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 190f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 227.5f32 + 1.15f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn pantheon_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 155f32 + 2.3f32 * ctx.bonus_ad,
        2 => 230f32 + 2.3f32 * ctx.bonus_ad,
        3 => 305f32 + 2.3f32 * ctx.bonus_ad,
        4 => 380f32 + 2.3f32 * ctx.bonus_ad,
        5 => 455f32 + 2.3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn pantheon_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 1.15f32 * ctx.bonus_ad,
        2 => 100f32 + 1.15f32 * ctx.bonus_ad,
        3 => 130f32 + 1.15f32 * ctx.bonus_ad,
        4 => 160f32 + 1.15f32 * ctx.bonus_ad,
        5 => 190f32 + 1.15f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn pantheon_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            0.06f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        2 => {
            0.065f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        3 => {
            0.07f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        4 => {
            0.075f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        5 => {
            0.08f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
                + 0.004f32 * 0.01f32 * ctx.bonus_health
        }
        _ => 0.0,
    }
}
pub const fn pantheon_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 1.5f32 * ctx.bonus_ad,
        2 => 105f32 + 1.5f32 * ctx.bonus_ad,
        3 => 155f32 + 1.5f32 * ctx.bonus_ad,
        4 => 205f32 + 1.5f32 * ctx.bonus_ad,
        5 => 255f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn pantheon_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + ctx.ap,
        2 => 500f32 + ctx.ap,
        3 => 700f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn pantheon_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.ap,
        2 => 250f32 + 0.5f32 * ctx.ap,
        3 => 350f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static POPPY: CachedChampion = CachedChampion {
    name: "Poppy",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        poppy_q_1, poppy_q_2, poppy_q_3, poppy_q_4, poppy_q_5, poppy_w_1,
        poppy_e_1, poppy_e_2, poppy_r_1, poppy_r_2,
    ],
};
pub const fn poppy_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32,
        2 => 105f32,
        3 => 135f32,
        4 => 165f32,
        5 => 195f32,
        _ => 0.0,
    }
}
pub const fn poppy_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 105f32 + ctx.bonus_ad,
        2 => 160f32 + ctx.bonus_ad,
        3 => 215f32 + ctx.bonus_ad,
        4 => 270f32 + ctx.bonus_ad,
        5 => 325f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn poppy_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,
        2 => 55f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,
        3 => 80f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,
        4 => 105f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,
        5 => 130f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn poppy_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 210f32 + 2f32 * ctx.bonus_ad,
        2 => 320f32 + 2f32 * ctx.bonus_ad,
        3 => 430f32 + 2f32 * ctx.bonus_ad,
        4 => 540f32 + 2f32 * ctx.bonus_ad,
        5 => 650f32 + 2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn poppy_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,
        2 => 110f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,
        3 => 160f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,
        4 => 210f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,
        5 => 260f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn poppy_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 110f32 + 0.7f32 * ctx.ap,
        3 => 150f32 + 0.7f32 * ctx.ap,
        4 => 190f32 + 0.7f32 * ctx.ap,
        5 => 230f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn poppy_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 40f32 + 0.6f32 * ctx.bonus_ad,
        2 => 60f32 + 0.6f32 * ctx.bonus_ad,
        3 => 80f32 + 0.6f32 * ctx.bonus_ad,
        4 => 100f32 + 0.6f32 * ctx.bonus_ad,
        5 => 120f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn poppy_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 1.2f32 * ctx.bonus_ad,
        2 => 120f32 + 1.2f32 * ctx.bonus_ad,
        3 => 160f32 + 1.2f32 * ctx.bonus_ad,
        4 => 200f32 + 1.2f32 * ctx.bonus_ad,
        5 => 240f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn poppy_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.9f32 * ctx.bonus_ad,
        2 => 300f32 + 0.9f32 * ctx.bonus_ad,
        3 => 400f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn poppy_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.45f32 * ctx.bonus_ad,
        2 => 150f32 + 0.45f32 * ctx.bonus_ad,
        3 => 200f32 + 0.45f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static PYKE: CachedChampion = CachedChampion {
    name: "Pyke",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 670f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 415f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 43f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.667f32,
            per_level: 2.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.666999995708465f32,
        attack_range: 150f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[pyke_q_1, pyke_e_1],
};
pub const fn pyke_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 100f32 + 0.75f32 * ctx.bonus_ad,
        2 => 150f32 + 0.75f32 * ctx.bonus_ad,
        3 => 200f32 + 0.75f32 * ctx.bonus_ad,
        4 => 250f32 + 0.75f32 * ctx.bonus_ad,
        5 => 300f32 + 0.75f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn pyke_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 100f32 + ctx.bonus_ad,
        2 => 150f32 + ctx.bonus_ad,
        3 => 200f32 + ctx.bonus_ad,
        4 => 250f32 + ctx.bonus_ad,
        5 => 300f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static QIYANA: CachedChampion = CachedChampion {
    name: "Qiyana",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 124f32,
        },
        mana: CachedChampionStatsMap {
            flat: 375f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.688f32,
            per_level: 2.1f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 150f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.15f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        qiyana_q_1, qiyana_q_2, qiyana_w_1, qiyana_e_1, qiyana_r_1, qiyana_r_2,
    ],
};
pub const fn qiyana_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.9f32 * ctx.bonus_ad,
        2 => 90f32 + 0.9f32 * ctx.bonus_ad,
        3 => 120f32 + 0.9f32 * ctx.bonus_ad,
        4 => 150f32 + 0.9f32 * ctx.bonus_ad,
        5 => 180f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn qiyana_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.675f32 * ctx.bonus_ad,
        2 => 67.5f32 + 0.675f32 * ctx.bonus_ad,
        3 => 90f32 + 0.675f32 * ctx.bonus_ad,
        4 => 112.5f32 + 0.675f32 * ctx.bonus_ad,
        5 => 135f32 + 0.675f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn qiyana_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 8f32 + 0.2f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        2 => 16f32 + 0.2f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        3 => 24f32 + 0.2f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        4 => 32f32 + 0.2f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        5 => 40f32 + 0.2f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn qiyana_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.5f32 * ctx.bonus_ad,
        2 => 90f32 + 0.5f32 * ctx.bonus_ad,
        3 => 130f32 + 0.5f32 * ctx.bonus_ad,
        4 => 170f32 + 0.5f32 * ctx.bonus_ad,
        5 => 210f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn qiyana_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 500f32 + 1.25f32 * ctx.bonus_ad,
        2 => 750f32 + 1.25f32 * ctx.bonus_ad,
        3 => 1000f32 + 1.25f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn qiyana_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 1.25f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_max_health,
        2 => 200f32 + 1.25f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_max_health,
        3 => 300f32 + 1.25f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub static QUINN: CachedChampion = CachedChampion {
    name: "Quinn",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 565f32,
            per_level: 107f32,
        },
        mana: CachedChampionStatsMap {
            flat: 269f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 2.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.668f32,
            per_level: 3.1f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.667999982833862f32,
        attack_range: 525f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[quinn_q_1, quinn_e_1],
};
pub const fn quinn_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + 0.8f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 100f32 + 0.9f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 135f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 170f32 + 1.1f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 205f32 + 1.2f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn quinn_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 40f32 + 0.2f32 * ctx.bonus_ad,
        2 => 65f32 + 0.2f32 * ctx.bonus_ad,
        3 => 90f32 + 0.2f32 * ctx.bonus_ad,
        4 => 115f32 + 0.2f32 * ctx.bonus_ad,
        5 => 140f32 + 0.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static RAKAN: CachedChampion = CachedChampion {
    name: "Rakan",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 315f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4.9f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.635f32,
            per_level: 3f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.634999990463256f32,
        attack_range: 300f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[rakan_q_1, rakan_w_1, rakan_r_1],
};
pub const fn rakan_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 115f32 + 0.7f32 * ctx.ap,
        3 => 160f32 + 0.7f32 * ctx.ap,
        4 => 205f32 + 0.7f32 * ctx.ap,
        5 => 250f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rakan_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.8f32 * ctx.ap,
        2 => 120f32 + 0.8f32 * ctx.ap,
        3 => 170f32 + 0.8f32 * ctx.ap,
        4 => 220f32 + 0.8f32 * ctx.ap,
        5 => 270f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rakan_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.5f32 * ctx.ap,
        2 => 200f32 + 0.5f32 * ctx.ap,
        3 => 300f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static RAMMUS: CachedChampion = CachedChampion {
    name: "Rammus",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 675f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 310f32,
            per_level: 33f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 65f32,
            per_level: 2.75f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.7f32,
            per_level: 2.215f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[rammus_q_1, rammus_e_1, rammus_r_1],
};
pub const fn rammus_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + ctx.ap,
        2 => 120f32 + ctx.ap,
        3 => 160f32 + ctx.ap,
        4 => 200f32 + ctx.ap,
        5 => 240f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn rammus_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 100f32 + 0.7f32 * ctx.ap,
        3 => 120f32 + 0.7f32 * ctx.ap,
        4 => 140f32 + 0.7f32 * ctx.ap,
        5 => 160f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rammus_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.6f32 * ctx.ap,
        2 => 250f32 + 0.6f32 * ctx.ap,
        3 => 350f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static REKSAI: CachedChampion = CachedChampion {
    name: "Rek'Sai",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.667f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.666999995708465f32,
        attack_range: 175f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.93f32,
        urf_damage_dealt: 1.07f32,
    },
    merge_data: &[],
    closures: &[reksai_q_2, reksai_w_1, reksai_e_1, reksai_e_2, reksai_r_1],
};
pub const fn reksai_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.3f32 * ctx.ad,
        2 => 0.35f32 * ctx.ad,
        3 => 0.4f32 * ctx.ad,
        4 => 0.45f32 * ctx.ad,
        5 => 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn reksai_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32,
        2 => 10f32,
        3 => 15f32,
        4 => 20f32,
        5 => 25f32,
        _ => 0.0,
    }
}
pub const fn reksai_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.64f32 * ctx.bonus_ad,
        2 => 108f32 + 0.64f32 * ctx.bonus_ad,
        3 => 136f32 + 0.64f32 * ctx.bonus_ad,
        4 => 164f32 + 0.64f32 * ctx.bonus_ad,
        5 => 192f32 + 0.64f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn reksai_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 100f32 + 0.8f32 * ctx.bonus_ad,
        2 => 135f32 + 0.8f32 * ctx.bonus_ad,
        3 => 170f32 + 0.8f32 * ctx.bonus_ad,
        4 => 205f32 + 0.8f32 * ctx.bonus_ad,
        5 => 240f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn reksai_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + ctx.bonus_ad + 0.15f32 * ctx.enemy_max_health,
        2 => 250f32 + ctx.bonus_ad + 0.2f32 * ctx.enemy_max_health,
        3 => 350f32 + ctx.bonus_ad + 0.25f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub static RELL: CachedChampion = CachedChampion {
    name: "Rell",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 320f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 1.8f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 315f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[rell_q_1, rell_w_2, rell_e_1, rell_r_1, rell_r_2],
};
pub const fn rell_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 100f32 + 0.6f32 * ctx.ap,
        3 => 140f32 + 0.6f32 * ctx.ap,
        4 => 180f32 + 0.6f32 * ctx.ap,
        5 => 220f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rell_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 90f32 + 0.6f32 * ctx.ap,
        3 => 120f32 + 0.6f32 * ctx.ap,
        4 => 150f32 + 0.6f32 * ctx.ap,
        5 => 180f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rell_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.05f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        2 => 0.055f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        3 => 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        4 => 0.065f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        5 => 0.07f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rell_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 15f32 + 0.1375f32 * ctx.ap,
        2 => 25f32 + 0.1375f32 * ctx.ap,
        3 => 35f32 + 0.1375f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rell_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 120f32 + 1.1f32 * ctx.ap,
        2 => 200f32 + 1.1f32 * ctx.ap,
        3 => 280f32 + 1.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static RENATA: CachedChampion = CachedChampion {
    name: "Renata Glasc",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 545f32,
            per_level: 94f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 49f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.11f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[renata_q_1, renata_e_1],
};
pub const fn renata_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.8f32 * ctx.ap,
        2 => 125f32 + 0.8f32 * ctx.ap,
        3 => 170f32 + 0.8f32 * ctx.ap,
        4 => 215f32 + 0.8f32 * ctx.ap,
        5 => 260f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn renata_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 65f32 + 0.55f32 * ctx.ap,
        2 => 95f32 + 0.55f32 * ctx.ap,
        3 => 125f32 + 0.55f32 * ctx.ap,
        4 => 155f32 + 0.55f32 * ctx.ap,
        5 => 185f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static RENEKTON: CachedChampion = CachedChampion {
    name: "Renekton",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 660f32,
            per_level: 111f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 69f32,
            per_level: 4.15f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.665f32,
            per_level: 2.75f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.665000021457672f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        renekton_q_1,
        renekton_q_2,
        renekton_w_1,
        renekton_w_2,
        renekton_w_3,
        renekton_e_2,
        renekton_r_1,
        renekton_r_2,
    ],
};
pub const fn renekton_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.4f32 * ctx.bonus_ad,
        2 => 135f32 + 1.4f32 * ctx.bonus_ad,
        3 => 180f32 + 1.4f32 * ctx.bonus_ad,
        4 => 225f32 + 1.4f32 * ctx.bonus_ad,
        5 => 270f32 + 1.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn renekton_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + ctx.bonus_ad,
        2 => 90f32 + ctx.bonus_ad,
        3 => 120f32 + ctx.bonus_ad,
        4 => 150f32 + ctx.bonus_ad,
        5 => 180f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn renekton_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.75f32 * ctx.ad,
        2 => 20f32 + 0.75f32 * ctx.ad,
        3 => 35f32 + 0.75f32 * ctx.ad,
        4 => 50f32 + 0.75f32 * ctx.ad,
        5 => 65f32 + 0.75f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn renekton_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 1.5f32 * ctx.ad,
        2 => 40f32 + 1.5f32 * ctx.ad,
        3 => 70f32 + 1.5f32 * ctx.ad,
        4 => 100f32 + 1.5f32 * ctx.ad,
        5 => 130f32 + 1.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn renekton_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 15f32 + 2.25f32 * ctx.ad,
        2 => 60f32 + 2.25f32 * ctx.ad,
        3 => 105f32 + 2.25f32 * ctx.ad,
        4 => 150f32 + 2.25f32 * ctx.ad,
        5 => 195f32 + 2.25f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn renekton_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 40f32 + 0.9f32 * ctx.bonus_ad,
        2 => 70f32 + 0.9f32 * ctx.bonus_ad,
        3 => 100f32 + 0.9f32 * ctx.bonus_ad,
        4 => 130f32 + 0.9f32 * ctx.bonus_ad,
        5 => 160f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn renekton_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32 + 0.05f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        2 => 75f32 + 0.05f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        3 => 120f32 + 0.05f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn renekton_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 900f32 + 1.5f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        2 => 2250f32 + 1.5f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        3 => 3600f32 + 1.5f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static RENGAR: CachedChampion = CachedChampion {
    name: "Rengar",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 4f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.667f32,
            per_level: 3f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.666999995708465f32,
        attack_range: 125f32,
        aram_damage_taken: 0.92f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[rengar_q_1, rengar_w_1, rengar_e_1],
};
pub const fn rengar_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0f32 * ctx.ad,
        2 => 60f32 + 0.0375f32 * ctx.ad,
        3 => 90f32 + 0.075f32 * ctx.ad,
        4 => 120f32 + 0.1125f32 * ctx.ad,
        5 => 150f32 + 0.15f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn rengar_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.8f32 * ctx.ap,
        2 => 80f32 + 0.8f32 * ctx.ap,
        3 => 110f32 + 0.8f32 * ctx.ap,
        4 => 140f32 + 0.8f32 * ctx.ap,
        5 => 170f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rengar_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 0.8f32 * ctx.bonus_ad,
        2 => 100f32 + 0.8f32 * ctx.bonus_ad,
        3 => 145f32 + 0.8f32 * ctx.bonus_ad,
        4 => 190f32 + 0.8f32 * ctx.bonus_ad,
        5 => 235f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static RIVEN: CachedChampion = CachedChampion {
    name: "Riven",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 0.92f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[riven_q_1, riven_q_2, riven_w_1],
};
pub const fn riven_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.65f32 * ctx.bonus_ad,
        2 => 75f32 + 0.7f32 * ctx.bonus_ad,
        3 => 105f32 + 0.75f32 * ctx.bonus_ad,
        4 => 135f32 + 0.8f32 * ctx.bonus_ad,
        5 => 165f32 + 0.85f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn riven_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 135f32 + 1.95f32 * ctx.bonus_ad,
        2 => 225f32 + 2.1f32 * ctx.bonus_ad,
        3 => 315f32 + 2.25f32 * ctx.bonus_ad,
        4 => 405f32 + 2.4f32 * ctx.bonus_ad,
        5 => 495f32 + 2.55f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn riven_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 65f32 + ctx.bonus_ad,
        2 => 95f32 + ctx.bonus_ad,
        3 => 125f32 + ctx.bonus_ad,
        4 => 155f32 + ctx.bonus_ad,
        5 => 185f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static RUMBLE: CachedChampion = CachedChampion {
    name: "Rumble",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_7),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_8),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 150f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 1.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 1.85f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        rumble_q_1, rumble_q_2, rumble_q_3, rumble_q_4, rumble_q_5, rumble_q_6,
        rumble_q_7, rumble_q_8, rumble_e_1, rumble_e_2, rumble_e_3, rumble_e_4,
        rumble_r_1, rumble_r_2, rumble_r_3,
    ],
};
pub const fn rumble_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.0917f32 * ctx.ap + 0.005f32 * ctx.enemy_max_health,
        2 => 7.5f32 + 0.0917f32 * ctx.ap + 0.00567f32 * ctx.enemy_max_health,
        3 => 10f32 + 0.0917f32 * ctx.ap + 0.00633f32 * ctx.enemy_max_health,
        4 => {
            12.5f32
                + 0.0917f32 * ctx.ap
                + 0.006999999999999999f32 * ctx.enemy_max_health
        }
        5 => 15f32 + 0.0917f32 * ctx.ap + 0.00767f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            3.5f32
                + 0.06417f32 * ctx.ap
                + 0.0034999999999999996f32 * ctx.enemy_max_health
        }
        2 => {
            5.25f32
                + 0.06417f32 * ctx.ap
                + 0.0039700000000000004f32 * ctx.enemy_max_health
        }
        3 => 7f32 + 0.06417f32 * ctx.ap + 0.00443f32 * ctx.enemy_max_health,
        4 => 8.75f32 + 0.06417f32 * ctx.ap + 0.0049f32 * ctx.enemy_max_health,
        5 => {
            10.5f32
                + 0.06417f32 * ctx.ap
                + 0.005370000000000001f32 * ctx.enemy_max_health
        }
        _ => 0.0,
    }
}
pub const fn rumble_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 1.1f32 * ctx.ap + 0.06f32 * ctx.enemy_max_health,
        2 => 90f32 + 1.1f32 * ctx.ap + 0.068f32 * ctx.enemy_max_health,
        3 => 120f32 + 1.1f32 * ctx.ap + 0.076f32 * ctx.enemy_max_health,
        4 => 150f32 + 1.1f32 * ctx.ap + 0.084f32 * ctx.enemy_max_health,
        5 => 180f32 + 1.1f32 * ctx.ap + 0.092f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 42f32 + 0.77f32 * ctx.ap + 0.042f32 * ctx.enemy_max_health,
        2 => {
            63f32
                + 0.77f32 * ctx.ap
                + 0.047599999999999996f32 * ctx.enemy_max_health
        }
        3 => {
            84f32
                + 0.77f32 * ctx.ap
                + 0.053200000000000004f32 * ctx.enemy_max_health
        }
        4 => 105f32 + 0.77f32 * ctx.ap + 0.0588f32 * ctx.enemy_max_health,
        5 => 126f32 + 0.77f32 * ctx.ap + 0.0644f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 7.5f32 + 0.1375f32 * ctx.ap + 0.0075f32 * ctx.enemy_max_health,
        2 => 11.25f32 + 0.1375f32 * ctx.ap + 0.0085f32 * ctx.enemy_max_health,
        3 => 15f32 + 0.1375f32 * ctx.ap + 0.0095f32 * ctx.enemy_max_health,
        4 => 18.75f32 + 0.1375f32 * ctx.ap + 0.0105f32 * ctx.enemy_max_health,
        5 => 22.5f32 + 0.1375f32 * ctx.ap + 0.0115f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5.25f32 + 0.09625f32 * ctx.ap + 0.00525f32 * ctx.enemy_max_health,
        2 => {
            7.875f32
                + 0.09625f32 * ctx.ap
                + 0.0059499999999999996f32 * ctx.enemy_max_health
        }
        3 => {
            10.5f32
                + 0.09625f32 * ctx.ap
                + 0.0066500000000000005f32 * ctx.enemy_max_health
        }
        4 => {
            13.125f32 + 0.09625f32 * ctx.ap + 0.00735f32 * ctx.enemy_max_health
        }
        5 => 15.75f32 + 0.09625f32 * ctx.ap + 0.00805f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_q_7(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.65f32 * ctx.ap + 0.09f32 * ctx.enemy_max_health,
        2 => 135f32 + 1.65f32 * ctx.ap + 0.102f32 * ctx.enemy_max_health,
        3 => 180f32 + 1.65f32 * ctx.ap + 0.114f32 * ctx.enemy_max_health,
        4 => 225f32 + 1.65f32 * ctx.ap + 0.126f32 * ctx.enemy_max_health,
        5 => 270f32 + 1.65f32 * ctx.ap + 0.138f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_q_8(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 63f32 + 1.155f32 * ctx.ap + 0.063f32 * ctx.enemy_max_health,
        2 => {
            94.5f32
                + 1.155f32 * ctx.ap
                + 0.07139999999999999f32 * ctx.enemy_max_health
        }
        3 => {
            126f32
                + 1.155f32 * ctx.ap
                + 0.07980000000000001f32 * ctx.enemy_max_health
        }
        4 => 157.5f32 + 1.155f32 * ctx.ap + 0.0882f32 * ctx.enemy_max_health,
        5 => 189f32 + 1.155f32 * ctx.ap + 0.0966f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn rumble_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 0.5f32 * ctx.ap,
        2 => 80f32 + 0.5f32 * ctx.ap,
        3 => 105f32 + 0.5f32 * ctx.ap,
        4 => 130f32 + 0.5f32 * ctx.ap,
        5 => 155f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rumble_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 110f32 + ctx.ap,
        2 => 160f32 + ctx.ap,
        3 => 210f32 + ctx.ap,
        4 => 260f32 + ctx.ap,
        5 => 310f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn rumble_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 82.5f32 + 0.75f32 * ctx.ap,
        2 => 120f32 + 0.75f32 * ctx.ap,
        3 => 157.5f32 + 0.75f32 * ctx.ap,
        4 => 195f32 + 0.75f32 * ctx.ap,
        5 => 232.5f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rumble_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 165f32 + 1.5f32 * ctx.ap,
        2 => 240f32 + 1.5f32 * ctx.ap,
        3 => 315f32 + 1.5f32 * ctx.ap,
        4 => 390f32 + 1.5f32 * ctx.ap,
        5 => 465f32 + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rumble_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.175f32 * ctx.ap,
        2 => 105f32 + 0.175f32 * ctx.ap,
        3 => 140f32 + 0.175f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rumble_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 700f32 + 1.75f32 * ctx.ap,
        2 => 1050f32 + 1.75f32 * ctx.ap,
        3 => 1400f32 + 1.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn rumble_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 140f32 + 0.35f32 * ctx.ap,
        2 => 210f32 + 0.35f32 * ctx.ap,
        3 => 280f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static RYZE: CachedChampion = CachedChampion {
    name: "Ryze",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 645f32,
            per_level: 124f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 70f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.11f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[ryze_q_1, ryze_w_1, ryze_e_1, ryze_r_1],
};
pub const fn ryze_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.55f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        2 => 95f32 + 0.55f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        3 => 115f32 + 0.55f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        4 => 135f32 + 0.55f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        5 => 155f32 + 0.55f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        _ => 0.0,
    }
}
pub const fn ryze_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,
        2 => 90f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,
        3 => 120f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,
        4 => 150f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,
        5 => 180f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,
        _ => 0.0,
    }
}
pub const fn ryze_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        2 => 90f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        3 => 120f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        4 => 150f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        5 => 180f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,
        _ => 0.0,
    }
}
pub const fn ryze_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.5f32,
        2 => 0.75f32,
        3 => 1f32,
        _ => 0.0,
    }
}
pub static SAMIRA: CachedChampion = CachedChampion {
    name: "Samira",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 108f32,
        },
        mana: CachedChampionStatsMap {
            flat: 349f32,
            per_level: 38f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 57f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.3f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 500f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        samira_q_1, samira_w_1, samira_w_2, samira_e_1, samira_r_1, samira_r_2,
        samira_r_3, samira_r_4,
    ],
};
pub const fn samira_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.95f32 * ctx.ad,
        2 => 10f32 + 1.025f32 * ctx.ad,
        3 => 15f32 + 1.1f32 * ctx.ad,
        4 => 20f32 + 1.175f32 * ctx.ad,
        5 => 25f32 + 1.25f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn samira_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.5f32 * ctx.bonus_ad,
        2 => 35f32 + 0.5f32 * ctx.bonus_ad,
        3 => 50f32 + 0.5f32 * ctx.bonus_ad,
        4 => 65f32 + 0.5f32 * ctx.bonus_ad,
        5 => 80f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn samira_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + ctx.bonus_ad,
        2 => 70f32 + ctx.bonus_ad,
        3 => 100f32 + ctx.bonus_ad,
        4 => 130f32 + ctx.bonus_ad,
        5 => 160f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn samira_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.2f32 * ctx.bonus_ad,
        2 => 60f32 + 0.2f32 * ctx.bonus_ad,
        3 => 70f32 + 0.2f32 * ctx.bonus_ad,
        4 => 80f32 + 0.2f32 * ctx.bonus_ad,
        5 => 90f32 + 0.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn samira_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 1.25f32 + 0.1125f32 * ctx.ad,
        2 => 3.75f32 + 0.1125f32 * ctx.ad,
        3 => 6.25f32 + 0.1125f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn samira_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 5f32 + 0.45f32 * ctx.ad,
        2 => 15f32 + 0.45f32 * ctx.ad,
        3 => 25f32 + 0.45f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn samira_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 12.5f32 + 1.125f32 * ctx.ad,
        2 => 37.5f32 + 1.125f32 * ctx.ad,
        3 => 62.5f32 + 1.125f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn samira_r_4(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 50f32 + 4.5f32 * ctx.ad,
        2 => 150f32 + 4.5f32 * ctx.ad,
        3 => 250f32 + 4.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static SEJUANI: CachedChampion = CachedChampion {
    name: "Sejuani",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 5.45f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.688f32,
            per_level: 3.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 150f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        sejuani_q_1,
        sejuani_w_1,
        sejuani_w_2,
        sejuani_w_3,
        sejuani_e_1,
        sejuani_r_1,
        sejuani_r_2,
    ],
};
pub const fn sejuani_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 0.6f32 * ctx.ap,
        2 => 140f32 + 0.6f32 * ctx.ap,
        3 => 190f32 + 0.6f32 * ctx.ap,
        4 => 240f32 + 0.6f32 * ctx.ap,
        5 => 290f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sejuani_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,
        2 => 15f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,
        3 => 25f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,
        4 => 35f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,
        5 => 45f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn sejuani_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,
        2 => 25f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,
        3 => 45f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,
        4 => 65f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,
        5 => 85f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn sejuani_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,
        2 => 40f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,
        3 => 70f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,
        4 => 100f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,
        5 => 130f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn sejuani_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 0.6f32 * ctx.ap,
        2 => 105f32 + 0.6f32 * ctx.ap,
        3 => 155f32 + 0.6f32 * ctx.ap,
        4 => 205f32 + 0.6f32 * ctx.ap,
        5 => 255f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sejuani_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 125f32 + 0.4f32 * ctx.ap,
        2 => 150f32 + 0.4f32 * ctx.ap,
        3 => 175f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sejuani_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.8f32 * ctx.ap,
        2 => 300f32 + 0.8f32 * ctx.ap,
        3 => 400f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SENNA: CachedChampion = CachedChampion {
    name: "Senna",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 530f32,
            per_level: 89f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 50f32,
            per_level: 0f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.4f32,
        attack_range: 600f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.92f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[senna_q_1, senna_w_1, senna_r_1],
};
pub const fn senna_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.4f32 * ctx.bonus_ad,
        2 => 60f32 + 0.4f32 * ctx.bonus_ad,
        3 => 90f32 + 0.4f32 * ctx.bonus_ad,
        4 => 120f32 + 0.4f32 * ctx.bonus_ad,
        5 => 150f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn senna_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.7f32 * ctx.bonus_ad,
        2 => 115f32 + 0.7f32 * ctx.bonus_ad,
        3 => 160f32 + 0.7f32 * ctx.bonus_ad,
        4 => 205f32 + 0.7f32 * ctx.bonus_ad,
        5 => 250f32 + 0.7f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn senna_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 250f32 + 1.15f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        2 => 400f32 + 1.15f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        3 => 550f32 + 1.15f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SERAPHINE: CachedChampion = CachedChampion {
    name: "Seraphine",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 570f32,
            per_level: 90f32,
        },
        mana: CachedChampionStatsMap {
            flat: 360f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 50f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.669f32,
            per_level: 2f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1.2f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.92f32,
    },
    merge_data: &[],
    closures: &[
        seraphine_q_1,
        seraphine_q_2,
        seraphine_e_1,
        seraphine_e_2,
        seraphine_r_1,
    ],
};
pub const fn seraphine_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.5f32 * ctx.ap,
        2 => 85f32 + 0.5f32 * ctx.ap,
        3 => 110f32 + 0.5f32 * ctx.ap,
        4 => 135f32 + 0.5f32 * ctx.ap,
        5 => 160f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn seraphine_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 105f32 + 0.875f32 * ctx.ap,
        2 => 148.75f32 + 0.875f32 * ctx.ap,
        3 => 192.5f32 + 0.875f32 * ctx.ap,
        4 => 236.25f32 + 0.875f32 * ctx.ap,
        5 => 280f32 + 0.875f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn seraphine_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.5f32 * ctx.ap,
        2 => 100f32 + 0.5f32 * ctx.ap,
        3 => 130f32 + 0.5f32 * ctx.ap,
        4 => 160f32 + 0.5f32 * ctx.ap,
        5 => 190f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn seraphine_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 49f32 + 0.35f32 * ctx.ap,
        2 => 70f32 + 0.35f32 * ctx.ap,
        3 => 91f32 + 0.35f32 * ctx.ap,
        4 => 112f32 + 0.35f32 * ctx.ap,
        5 => 133f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn seraphine_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.4f32 * ctx.ap,
        2 => 200f32 + 0.4f32 * ctx.ap,
        3 => 250f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SETT: CachedChampion = CachedChampion {
    name: "Sett",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 670f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1.75f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.92f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[
        sett_q_1, sett_q_2, sett_w_1, sett_e_1, sett_e_2, sett_r_1, sett_r_2,
    ],
};
pub const fn sett_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn sett_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn sett_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 0.25f32 * 0.0f32 + 0.25f32 * 0.01f32 * ctx.bonus_ad,
        2 => 100f32 + 0.25f32 * 0.0f32 + 0.25f32 * 0.01f32 * ctx.bonus_ad,
        3 => 120f32 + 0.25f32 * 0.0f32 + 0.25f32 * 0.01f32 * ctx.bonus_ad,
        4 => 140f32 + 0.25f32 * 0.0f32 + 0.25f32 * 0.01f32 * ctx.bonus_ad,
        5 => 160f32 + 0.25f32 * 0.0f32 + 0.25f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn sett_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 150f32 + 0.6f32 * ctx.ad,
        2 => 170f32 + 0.6f32 * ctx.ad,
        3 => 190f32 + 0.6f32 * ctx.ad,
        4 => 210f32 + 0.6f32 * ctx.ad,
        5 => 230f32 + 0.6f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sett_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.6f32 * ctx.ad,
        2 => 70f32 + 0.6f32 * ctx.ad,
        3 => 90f32 + 0.6f32 * ctx.ad,
        4 => 110f32 + 0.6f32 * ctx.ad,
        5 => 130f32 + 0.6f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sett_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.2f32 * ctx.bonus_ad + 0.4f32 * ctx.enemy_bonus_health,
        2 => 300f32 + 1.2f32 * ctx.bonus_ad + 0.5f32 * ctx.enemy_bonus_health,
        3 => 400f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.enemy_bonus_health,
        _ => 0.0,
    }
}
pub const fn sett_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 50f32 + 0.3f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_bonus_health,
        2 => 75f32 + 0.3f32 * ctx.bonus_ad + 0.125f32 * ctx.enemy_bonus_health,
        3 => 100f32 + 0.3f32 * ctx.bonus_ad + 0.15f32 * ctx.enemy_bonus_health,
        _ => 0.0,
    }
}
pub static SHACO: CachedChampion = CachedChampion {
    name: "Shaco",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 297f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.694f32,
            per_level: 3f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694000005722045f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        shaco_q_1, shaco_w_1, shaco_w_2, shaco_w_3, shaco_w_4, shaco_w_5,
        shaco_e_1, shaco_e_2, shaco_r_1, shaco_r_2, shaco_r_3,
    ],
};
pub const fn shaco_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 25f32 + 0.65f32 * ctx.bonus_ad,
        2 => 35f32 + 0.65f32 * ctx.bonus_ad,
        3 => 45f32 + 0.65f32 * ctx.bonus_ad,
        4 => 55f32 + 0.65f32 * ctx.bonus_ad,
        5 => 65f32 + 0.65f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn shaco_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32,
        2 => 35f32,
        3 => 50f32,
        4 => 65f32,
        5 => 80f32,
        _ => 0.0,
    }
}
pub const fn shaco_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 25f32 + 0.18f32 * ctx.ap,
        2 => 40f32 + 0.18f32 * ctx.ap,
        3 => 55f32 + 0.18f32 * ctx.ap,
        4 => 70f32 + 0.18f32 * ctx.ap,
        5 => 85f32 + 0.18f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 45f32 + 0.18f32 * ctx.ap,
        2 => 75f32 + 0.18f32 * ctx.ap,
        3 => 105f32 + 0.18f32 * ctx.ap,
        4 => 135f32 + 0.18f32 * ctx.ap,
        5 => 165f32 + 0.18f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.12f32 * ctx.ap,
        2 => 15f32 + 0.12f32 * ctx.ap,
        3 => 20f32 + 0.12f32 * ctx.ap,
        4 => 25f32 + 0.12f32 * ctx.ap,
        5 => 30f32 + 0.12f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_w_5(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.12f32 * ctx.ap,
        2 => 50f32 + 0.12f32 * ctx.ap,
        3 => 70f32 + 0.12f32 * ctx.ap,
        4 => 90f32 + 0.12f32 * ctx.ap,
        5 => 110f32 + 0.12f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 105f32 + 1.2f32 * ctx.bonus_ad + 0.9f32 * ctx.ap,
        2 => 142.5f32 + 1.2f32 * ctx.bonus_ad + 0.9f32 * ctx.ap,
        3 => 180f32 + 1.2f32 * ctx.bonus_ad + 0.9f32 * ctx.ap,
        4 => 217.5f32 + 1.2f32 * ctx.bonus_ad + 0.9f32 * ctx.ap,
        5 => 255f32 + 1.2f32 * ctx.bonus_ad + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        2 => 95f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        3 => 120f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        4 => 145f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        5 => 170f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 25f32 + 0.15f32 * ctx.ap,
        2 => 50f32 + 0.15f32 * ctx.ap,
        3 => 75f32 + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 10f32 + 0.1f32 * ctx.ap,
        2 => 20f32 + 0.1f32 * ctx.ap,
        3 => 30f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shaco_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.7f32 * ctx.ap,
        2 => 225f32 + 0.7f32 * ctx.ap,
        3 => 300f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SHEN: CachedChampion = CachedChampion {
    name: "Shen",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 64f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.751f32,
            per_level: 3f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.651000022888183f32,
        attack_range: 125f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[shen_q_1, shen_q_2, shen_q_3, shen_q_4, shen_q_5, shen_e_1],
};
pub const fn shen_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 120f32,
        2 => 140f32,
        3 => 160f32,
        4 => 180f32,
        5 => 200f32,
        _ => 0.0,
    }
}
pub const fn shen_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            10f32 + 0.05f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap
        }
        2 => {
            11.764705882352942f32
                + 0.055f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.ap
        }
        3 => {
            13.529411764705882f32
                + 0.06f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.ap
        }
        4 => {
            15.294117647058822f32
                + 0.065f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.ap
        }
        5 => {
            17.058823529411764f32
                + 0.07f32 * ctx.enemy_max_health
                + 0.02f32 * 0.01f32 * ctx.ap
        }
        6 => 18.823529411764707f32,
        7 => 20.588235294117645f32,
        8 => 22.352941176470587f32,
        9 => 24.11764705882353f32,
        10 => 25.88235294117647f32,
        11 => 27.647058823529413f32,
        12 => 29.41176470588235f32,
        13 => 31.176470588235293f32,
        14 => 32.94117647058823f32,
        15 => 34.705882352941174f32,
        16 => 36.47058823529411f32,
        17 => 38.23529411764706f32,
        18 => 40f32,
        _ => 0.0,
    }
}
pub const fn shen_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            30f32 + 0.15f32 * ctx.enemy_max_health + 0.06f32 * 0.01f32 * ctx.ap
        }
        2 => {
            35.294117647058826f32
                + 0.165f32 * ctx.enemy_max_health
                + 0.06f32 * 0.01f32 * ctx.ap
        }
        3 => {
            40.588235294117645f32
                + 0.18f32 * ctx.enemy_max_health
                + 0.06f32 * 0.01f32 * ctx.ap
        }
        4 => {
            45.88235294117647f32
                + 0.195f32 * ctx.enemy_max_health
                + 0.06f32 * 0.01f32 * ctx.ap
        }
        5 => {
            51.17647058823529f32
                + 0.21f32 * ctx.enemy_max_health
                + 0.06f32 * 0.01f32 * ctx.ap
        }
        6 => 56.47058823529411f32,
        7 => 61.76470588235294f32,
        8 => 67.05882352941177f32,
        9 => 72.35294117647058f32,
        10 => 77.64705882352942f32,
        11 => 82.94117647058823f32,
        12 => 88.23529411764706f32,
        13 => 93.52941176470588f32,
        14 => 98.82352941176472f32,
        15 => 104.11764705882352f32,
        16 => 109.41176470588236f32,
        17 => 114.70588235294116f32,
        18 => 120f32,
        _ => 0.0,
    }
}
pub const fn shen_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            10f32 + 0.02f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap
        }
        2 => {
            11.764705882352942f32
                + 0.025f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        3 => {
            13.529411764705882f32
                + 0.03f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        4 => {
            15.294117647058822f32
                + 0.035f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        5 => {
            17.058823529411764f32
                + 0.04f32 * ctx.enemy_max_health
                + 0.015f32 * 0.01f32 * ctx.ap
        }
        6 => 18.823529411764707f32,
        7 => 20.588235294117645f32,
        8 => 22.352941176470587f32,
        9 => 24.11764705882353f32,
        10 => 25.88235294117647f32,
        11 => 27.647058823529413f32,
        12 => 29.41176470588235f32,
        13 => 31.176470588235293f32,
        14 => 32.94117647058823f32,
        15 => 34.705882352941174f32,
        16 => 36.47058823529411f32,
        17 => 38.23529411764706f32,
        18 => 40f32,
        _ => 0.0,
    }
}
pub const fn shen_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => {
            30f32 + 0.06f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap
        }
        2 => {
            35.294117647058826f32
                + 0.075f32 * ctx.enemy_max_health
                + 0.045f32 * 0.01f32 * ctx.ap
        }
        3 => {
            40.588235294117645f32
                + 0.09f32 * ctx.enemy_max_health
                + 0.045f32 * 0.01f32 * ctx.ap
        }
        4 => {
            45.88235294117647f32
                + 0.105f32 * ctx.enemy_max_health
                + 0.045f32 * 0.01f32 * ctx.ap
        }
        5 => {
            51.17647058823529f32
                + 0.12f32 * ctx.enemy_max_health
                + 0.045f32 * 0.01f32 * ctx.ap
        }
        6 => 56.47058823529411f32,
        7 => 61.76470588235294f32,
        8 => 67.05882352941177f32,
        9 => 72.35294117647058f32,
        10 => 77.64705882352942f32,
        11 => 82.94117647058823f32,
        12 => 88.23529411764706f32,
        13 => 93.52941176470588f32,
        14 => 98.82352941176472f32,
        15 => 104.11764705882352f32,
        16 => 109.41176470588236f32,
        17 => 114.70588235294116f32,
        18 => 120f32,
        _ => 0.0,
    }
}
pub const fn shen_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.11f32 * ctx.bonus_health,
        2 => 85f32 + 0.11f32 * ctx.bonus_health,
        3 => 110f32 + 0.11f32 * ctx.bonus_health,
        4 => 135f32 + 0.11f32 * ctx.bonus_health,
        5 => 160f32 + 0.11f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub static SHYVANA: CachedChampion = CachedChampion {
    name: "Shyvana",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 665f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 38f32,
            per_level: 4.55f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.5f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.5f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[
        shyvana_q_1,
        shyvana_w_1,
        shyvana_w_2,
        shyvana_e_1,
        shyvana_e_2,
        shyvana_r_1,
    ],
};
pub const fn shyvana_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.2f32 * ctx.ad + 0.25f32 * ctx.ap,
        2 => 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        3 => 0.6f32 * ctx.ad + 0.25f32 * ctx.ap,
        4 => 0.8f32 * ctx.ad + 0.25f32 * ctx.ap,
        5 => ctx.ad + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shyvana_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.1f32 * ctx.bonus_ad,
        2 => 15f32 + 0.1f32 * ctx.bonus_ad,
        3 => 20f32 + 0.1f32 * ctx.bonus_ad,
        4 => 25f32 + 0.1f32 * ctx.bonus_ad,
        5 => 30f32 + 0.1f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn shyvana_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.05f32 * ctx.bonus_ad,
        2 => 7f32 + 0.05f32 * ctx.bonus_ad,
        3 => 9f32 + 0.05f32 * ctx.bonus_ad,
        4 => 11f32 + 0.05f32 * ctx.bonus_ad,
        5 => 13f32 + 0.05f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn shyvana_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 85f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        2 => 125f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        3 => 165f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        4 => 205f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        5 => 245f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn shyvana_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32 + 85f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        2 => 78.52941176470588f32 + 125f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        3 => 82.05882352941177f32 + 165f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        4 => 85.58823529411765f32 + 205f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        5 => 89.11764705882354f32 + 245f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,
        6 => 92.64705882352942f32,
        7 => 96.17647058823528f32,
        8 => 99.70588235294116f32,
        9 => 103.23529411764706f32,
        10 => 106.76470588235294f32,
        11 => 110.29411764705884f32,
        12 => 113.8235294117647f32,
        13 => 117.35294117647058f32,
        14 => 120.88235294117646f32,
        15 => 124.41176470588236f32,
        16 => 127.94117647058825f32,
        17 => 131.47058823529412f32,
        18 => 135f32,
        _ => 0.0,
    }
}
pub const fn shyvana_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + ctx.ap,
        2 => 250f32 + ctx.ap,
        3 => 350f32 + ctx.ap,
        _ => 0.0,
    }
}
pub static SINGED: CachedChampion = CachedChampion {
    name: "Singed",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 96f32,
        },
        mana: CachedChampionStatsMap {
            flat: 330f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3.4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.7f32,
            per_level: 1.9f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 1.08f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[singed_q_1, singed_q_2, singed_e_1],
};
pub const fn singed_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.10625f32 * ctx.ap,
        2 => 7.5f32 + 0.10625f32 * ctx.ap,
        3 => 10f32 + 0.10625f32 * ctx.ap,
        4 => 12.5f32 + 0.10625f32 * ctx.ap,
        5 => 15f32 + 0.10625f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn singed_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.85f32 * ctx.ap,
        2 => 60f32 + 0.85f32 * ctx.ap,
        3 => 80f32 + 0.85f32 * ctx.ap,
        4 => 100f32 + 0.85f32 * ctx.ap,
        5 => 120f32 + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn singed_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.06f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,
        2 => 60f32 + 0.065f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,
        3 => 70f32 + 0.07f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,
        4 => 80f32 + 0.075f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,
        5 => 90f32 + 0.08f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SION: CachedChampion = CachedChampion {
    name: "Sion",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_7),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 87f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 52f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.679f32,
            per_level: 1.3f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.67900002002716f32,
        attack_range: 175f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 0.92f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        sion_q_1, sion_q_2, sion_q_3, sion_q_4, sion_q_5, sion_q_6, sion_q_7,
        sion_w_1, sion_e_1, sion_r_1, sion_r_2,
    ],
};
pub const fn sion_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 1.25f32,
        2 => 1.5833000000000002f32,
        3 => 1.75f32,
        4 => 1.85f32,
        5 => 1.9166999999999998f32,
        _ => 0.0,
    }
}
pub const fn sion_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.2f32 * ctx.ad,
        2 => 155f32 + 1.5f32 * ctx.ad,
        3 => 220f32 + 1.8f32 * ctx.ad,
        4 => 285f32 + 2.1f32 * ctx.ad,
        5 => 350f32 + 2.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sion_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.4f32 * ctx.ad,
        2 => 60f32 + 0.5f32 * ctx.ad,
        3 => 80f32 + 0.6f32 * ctx.ad,
        4 => 100f32 + 0.7f32 * ctx.ad,
        5 => 120f32 + 0.8f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sion_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 54f32 + 0.72f32 * ctx.ad,
        2 => 93f32 + 0.9f32 * ctx.ad,
        3 => 132f32 + 1.08f32 * ctx.ad,
        4 => 171f32 + 1.26f32 * ctx.ad,
        5 => 210f32 + 1.44f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sion_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 135f32 + 1.8f32 * ctx.ad,
        2 => 232.5f32 + 2.25f32 * ctx.ad,
        3 => 330f32 + 2.7f32 * ctx.ad,
        4 => 427.5f32 + 3.15f32 * ctx.ad,
        5 => 525f32 + 3.6f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sion_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 24f32 + 0.24f32 * ctx.ad,
        2 => 36f32 + 0.3f32 * ctx.ad,
        3 => 48f32 + 0.36f32 * ctx.ad,
        4 => 60f32 + 0.42f32 * ctx.ad,
        5 => 72f32 + 0.48f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sion_q_7(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.6f32 * ctx.ad,
        2 => 90f32 + 0.75f32 * ctx.ad,
        3 => 120f32 + 0.9f32 * ctx.ad,
        4 => 150f32 + 1.05f32 * ctx.ad,
        5 => 180f32 + 1.2f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sion_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + 0.4f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,
        2 => 65f32 + 0.4f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,
        3 => 90f32 + 0.4f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,
        4 => 115f32 + 0.4f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,
        5 => 140f32 + 0.4f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn sion_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 65f32 + 0.55f32 * ctx.ap,
        2 => 100f32 + 0.55f32 * ctx.ap,
        3 => 135f32 + 0.55f32 * ctx.ap,
        4 => 170f32 + 0.55f32 * ctx.ap,
        5 => 205f32 + 0.55f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sion_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 400f32 + 0.8f32 * ctx.bonus_ad,
        2 => 800f32 + 0.8f32 * ctx.bonus_ad,
        3 => 1200f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn sion_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.4f32 * ctx.bonus_ad,
        2 => 300f32 + 0.4f32 * ctx.bonus_ad,
        3 => 450f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static SIVIR: CachedChampion = CachedChampion {
    name: "Sivir",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 340f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4.45f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 500f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.93f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[
        sivir_q_1, sivir_q_2, sivir_q_3, sivir_w_1, sivir_w_2, sivir_w_3,
        sivir_w_4,
    ],
};
pub const fn sivir_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + ctx.bonus_ad + 0.6f32 * ctx.ap,
        2 => 85f32 + ctx.bonus_ad + 0.6f32 * ctx.ap,
        3 => 110f32 + ctx.bonus_ad + 0.6f32 * ctx.ap,
        4 => 135f32 + ctx.bonus_ad + 0.6f32 * ctx.ap,
        5 => 160f32 + ctx.bonus_ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sivir_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 24f32 + 0.4f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,
        2 => 34f32 + 0.4f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,
        3 => 44f32 + 0.4f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,
        4 => 54f32 + 0.4f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,
        5 => 64f32 + 0.4f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sivir_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 120f32 + 2f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        2 => 170f32 + 2f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        3 => 220f32 + 2f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        4 => 270f32 + 2f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        5 => 320f32 + 2f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sivir_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.7f32 * ctx.ad,
        2 => 0.74375f32 * ctx.ad,
        3 => 0.7875f32 * ctx.ad,
        4 => 0.83125f32 * ctx.ad,
        5 => 0.875f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sivir_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.4f32 * ctx.ad,
        2 => 0.425f32 * ctx.ad,
        3 => 0.45f32 * ctx.ad,
        4 => 0.475f32 * ctx.ad,
        5 => 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sivir_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.455f32 * ctx.ad,
        2 => 0.4834375f32 * ctx.ad,
        3 => 0.511875f32 * ctx.ad,
        4 => 0.5403125f32 * ctx.ad,
        5 => 0.56875f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn sivir_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.26f32 * ctx.ad,
        2 => 0.27625f32 * ctx.ad,
        3 => 0.2925f32 * ctx.ad,
        4 => 0.30875f32 * ctx.ad,
        5 => 0.325f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static SKARNER: CachedChampion = CachedChampion {
    name: "Skarner",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 320f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 150f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        skarner_q_2,
        skarner_q_3,
        skarner_q_4,
        skarner_q_5,
        skarner_w_1,
        skarner_e_1,
        skarner_r_1,
    ],
};
pub const fn skarner_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,
        2 => 60f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,
        3 => 90f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,
        4 => 120f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,
        5 => 150f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn skarner_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 150f32,
        2 => 200f32,
        3 => 250f32,
        4 => 300f32,
        5 => 350f32,
        _ => 0.0,
    }
}
pub const fn skarner_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.2f32,
        2 => 0.25f32,
        3 => 0.3f32,
        4 => 0.35f32,
        5 => 0.4f32,
        _ => 0.0,
    }
}
pub const fn skarner_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,
        2 => 20f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,
        3 => 30f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,
        4 => 40f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,
        5 => 50f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn skarner_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.8f32 * ctx.ap,
        2 => 70f32 + 0.8f32 * ctx.ap,
        3 => 90f32 + 0.8f32 * ctx.ap,
        4 => 110f32 + 0.8f32 * ctx.ap,
        5 => 130f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn skarner_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 30f32 + 0.06f32 * ctx.max_health,
        2 => 60f32 + 0.06f32 * ctx.max_health,
        3 => 90f32 + 0.06f32 * ctx.max_health,
        4 => 120f32 + 0.06f32 * ctx.max_health,
        5 => 150f32 + 0.06f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn skarner_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + ctx.ap,
        2 => 250f32 + ctx.ap,
        3 => 350f32 + ctx.ap,
        _ => 0.0,
    }
}
pub static SMOLDER: CachedChampion = CachedChampion {
    name: "Smolder",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 575f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 4f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638f32,
        attack_range: 550f32,
        aram_damage_taken: 1.02f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        smolder_q_1,
        smolder_q_2,
        smolder_q_3,
        smolder_q_4,
        smolder_q_5,
        smolder_q_6,
        smolder_w_1,
        smolder_w_2,
        smolder_w_3,
        smolder_e_1,
        smolder_e_2,
        smolder_r_1,
        smolder_r_2,
    ],
};
pub const fn smolder_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 139.75f32 + 2.795f32 * ctx.bonus_ad,
        2 => 172f32 + 2.795f32 * ctx.bonus_ad,
        3 => 204.25f32 + 2.795f32 * ctx.bonus_ad,
        4 => 236.5f32 + 2.795f32 * ctx.bonus_ad,
        5 => 268.75f32 + 2.795f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 113.75f32 + 2.275f32 * ctx.bonus_ad,
        2 => 140f32 + 2.275f32 * ctx.bonus_ad,
        3 => 166.25f32 + 2.275f32 * ctx.bonus_ad,
        4 => 192.5f32 + 2.275f32 * ctx.bonus_ad,
        5 => 218.75f32 + 2.275f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + 1.3f32 * ctx.bonus_ad,
        2 => 80f32 + 1.3f32 * ctx.bonus_ad,
        3 => 95f32 + 1.3f32 * ctx.bonus_ad,
        4 => 110f32 + 1.3f32 * ctx.bonus_ad,
        5 => 125f32 + 1.3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 56.875f32 + 1.1375f32 * ctx.bonus_ad,
        2 => 70f32 + 1.1375f32 * ctx.bonus_ad,
        3 => 83.125f32 + 1.1375f32 * ctx.bonus_ad,
        4 => 96.25f32 + 1.1375f32 * ctx.bonus_ad,
        5 => 109.375f32 + 1.1375f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 69.875f32 + 1.3975f32 * ctx.bonus_ad,
        2 => 86f32 + 1.3975f32 * ctx.bonus_ad,
        3 => 102.125f32 + 1.3975f32 * ctx.bonus_ad,
        4 => 118.25f32 + 1.3975f32 * ctx.bonus_ad,
        5 => 134.375f32 + 1.3975f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 32.5f32 + 0.65f32 * ctx.bonus_ad,
        2 => 40f32 + 0.65f32 * ctx.bonus_ad,
        3 => 47.5f32 + 0.65f32 * ctx.bonus_ad,
        4 => 55f32 + 0.65f32 * ctx.bonus_ad,
        5 => 62.5f32 + 0.65f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        2 => 35f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        3 => 60f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        4 => 85f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        5 => 110f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn smolder_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.6f32 * ctx.bonus_ad,
        2 => 70f32 + 0.6f32 * ctx.bonus_ad,
        3 => 80f32 + 0.6f32 * ctx.bonus_ad,
        4 => 90f32 + 0.6f32 * ctx.bonus_ad,
        5 => 100f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn smolder_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        2 => 105f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        3 => 140f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        4 => 175f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        5 => 210f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn smolder_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 1.5f32 * ctx.ad,
        2 => 75f32 + 1.5f32 * ctx.ad,
        3 => 100f32 + 1.5f32 * ctx.ad,
        4 => 125f32 + 1.5f32 * ctx.ad,
        5 => 150f32 + 1.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn smolder_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 10f32 + 0.3f32 * ctx.ad,
        2 => 15f32 + 0.3f32 * ctx.ad,
        3 => 20f32 + 0.3f32 * ctx.ad,
        4 => 25f32 + 0.3f32 * ctx.ad,
        5 => 30f32 + 0.3f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn smolder_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 1.65f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        2 => 450f32 + 1.65f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        3 => 600f32 + 1.65f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn smolder_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.1f32 * ctx.bonus_ad + ctx.ap,
        2 => 300f32 + 1.1f32 * ctx.bonus_ad + ctx.ap,
        3 => 400f32 + 1.1f32 * ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub static SONA: CachedChampion = CachedChampion {
    name: "Sona",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 550f32,
            per_level: 91f32,
        },
        mana: CachedChampionStatsMap {
            flat: 340f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 26f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 49f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 2.3f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[sona_q_1, sona_q_2, sona_w_1, sona_r_1],
};
pub const fn sona_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 50f32 + 0.4f32 * ctx.ap,
        2 => 85f32,
        3 => 120f32,
        4 => 155f32,
        5 => 190f32,
        _ => 0.0,
    }
}
pub const fn sona_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.1f32 * ctx.ap,
        2 => 15f32,
        3 => 20f32,
        4 => 25f32,
        5 => 30f32,
        _ => 0.0,
    }
}
pub const fn sona_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 25f32,
        2 => 45f32,
        3 => 65f32,
        4 => 85f32,
        5 => 105f32,
        _ => 0.0,
    }
}
pub const fn sona_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.ap,
        2 => 250f32 + 0.5f32 * ctx.ap,
        3 => 350f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SORAKA: CachedChampion = CachedChampion {
    name: "Soraka",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 605f32,
            per_level: 88f32,
        },
        mana: CachedChampionStatsMap {
            flat: 425f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 50f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.14f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.92f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[soraka_q_1, soraka_e_1, soraka_e_2],
};
pub const fn soraka_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 85f32 + 0.35f32 * ctx.ap,
        2 => 120f32 + 0.35f32 * ctx.ap,
        3 => 155f32 + 0.35f32 * ctx.ap,
        4 => 190f32 + 0.35f32 * ctx.ap,
        5 => 225f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn soraka_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.4f32 * ctx.ap,
        2 => 95f32 + 0.4f32 * ctx.ap,
        3 => 120f32 + 0.4f32 * ctx.ap,
        4 => 145f32 + 0.4f32 * ctx.ap,
        5 => 170f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn soraka_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 140f32 + 0.8f32 * ctx.ap,
        2 => 190f32 + 0.8f32 * ctx.ap,
        3 => 240f32 + 0.8f32 * ctx.ap,
        4 => 290f32 + 0.8f32 * ctx.ap,
        5 => 340f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SWAIN: CachedChampion = CachedChampion {
    name: "Swain",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 595f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 29f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 1.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 2.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.11f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1.15f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        swain_q_1, swain_q_2, swain_q_3, swain_w_1, swain_w_2, swain_e_1,
        swain_r_1,
    ],
};
pub const fn swain_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15f32 + 0.1125f32 * ctx.ap,
        2 => 22.5f32 + 0.1125f32 * ctx.ap,
        3 => 30f32 + 0.1125f32 * ctx.ap,
        4 => 37.5f32 + 0.1125f32 * ctx.ap,
        5 => 45f32 + 0.1125f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn swain_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.45f32 * ctx.ap,
        2 => 90f32 + 0.45f32 * ctx.ap,
        3 => 120f32 + 0.45f32 * ctx.ap,
        4 => 150f32 + 0.45f32 * ctx.ap,
        5 => 180f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn swain_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 120f32 + 0.9f32 * ctx.ap,
        2 => 180f32 + 0.9f32 * ctx.ap,
        3 => 240f32 + 0.9f32 * ctx.ap,
        4 => 300f32 + 0.9f32 * ctx.ap,
        5 => 360f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn swain_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.6f32 * ctx.ap,
        2 => 105f32 + 0.6f32 * ctx.ap,
        3 => 140f32 + 0.6f32 * ctx.ap,
        4 => 175f32 + 0.6f32 * ctx.ap,
        5 => 210f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn swain_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 35f32 + 0.3f32 * ctx.ap,
        2 => 52.5f32 + 0.3f32 * ctx.ap,
        3 => 70f32 + 0.3f32 * ctx.ap,
        4 => 87.5f32 + 0.3f32 * ctx.ap,
        5 => 105f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn swain_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 120f32 + 0.7f32 * ctx.ap,
        3 => 160f32 + 0.7f32 * ctx.ap,
        4 => 200f32 + 0.7f32 * ctx.ap,
        5 => 240f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn swain_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 7.5f32 + 0.025f32 * ctx.ap,
        2 => 12.5f32 + 0.025f32 * ctx.ap,
        3 => 17.5f32 + 0.025f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SYLAS: CachedChampion = CachedChampion {
    name: "Sylas",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 122f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 70f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 61f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.645f32,
            per_level: 3.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.644999980926513f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        sylas_q_1, sylas_q_2, sylas_q_3, sylas_q_4, sylas_q_5, sylas_w_1,
    ],
};
pub const fn sylas_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.4f32 * ctx.ap,
        2 => 60f32 + 0.4f32 * ctx.ap,
        3 => 80f32 + 0.4f32 * ctx.ap,
        4 => 100f32 + 0.4f32 * ctx.ap,
        5 => 120f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sylas_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.8f32 * ctx.ap,
        2 => 115f32 + 0.8f32 * ctx.ap,
        3 => 170f32 + 0.8f32 * ctx.ap,
        4 => 225f32 + 0.8f32 * ctx.ap,
        5 => 280f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sylas_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 24f32 + 0.32f32 * ctx.ap,
        2 => 46f32 + 0.32f32 * ctx.ap,
        3 => 68f32 + 0.32f32 * ctx.ap,
        4 => 90f32 + 0.32f32 * ctx.ap,
        5 => 112f32 + 0.32f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sylas_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 100f32 + 1.2f32 * ctx.ap,
        2 => 175f32 + 1.2f32 * ctx.ap,
        3 => 250f32 + 1.2f32 * ctx.ap,
        4 => 325f32 + 1.2f32 * ctx.ap,
        5 => 400f32 + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sylas_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 64f32 + 0.72f32 * ctx.ap,
        2 => 106f32 + 0.72f32 * ctx.ap,
        3 => 148f32 + 0.72f32 * ctx.ap,
        4 => 190f32 + 0.72f32 * ctx.ap,
        5 => 232f32 + 0.72f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn sylas_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 75f32 + 0.6f32 * ctx.ap,
        2 => 110f32 + 0.6f32 * ctx.ap,
        3 => 145f32 + 0.6f32 * ctx.ap,
        4 => 180f32 + 0.6f32 * ctx.ap,
        5 => 215f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static SYNDRA: CachedChampion = CachedChampion {
    name: "Syndra",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 563f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 480f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 2.9f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        syndra_q_1, syndra_w_1, syndra_w_2, syndra_w_3, syndra_e_1, syndra_r_1,
        syndra_r_2, syndra_r_3,
    ],
};
pub const fn syndra_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.6f32 * ctx.ap,
        2 => 110f32 + 0.6f32 * ctx.ap,
        3 => 145f32 + 0.6f32 * ctx.ap,
        4 => 180f32 + 0.6f32 * ctx.ap,
        5 => 215f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + 0.65f32 * ctx.ap,
        2 => 105f32 + 0.65f32 * ctx.ap,
        3 => 140f32 + 0.65f32 * ctx.ap,
        4 => 175f32 + 0.65f32 * ctx.ap,
        5 => 210f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 8.4f32 + 0.092f32 * ctx.ap,
        2 => 12.6f32 + 0.099f32 * ctx.ap,
        3 => 16.8f32 + 0.106f32 * ctx.ap,
        4 => 21f32 + 0.113f32 * ctx.ap,
        5 => 25.2f32 + 0.12f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 78.4f32 + 0.742f32 * ctx.ap,
        2 => 117.6f32 + 0.7490000000000001f32 * ctx.ap,
        3 => 156.8f32 + 0.7559999999999999f32 * ctx.ap,
        4 => 196f32 + 0.763f32 * ctx.ap,
        5 => 235.2f32 + 0.77f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 95f32 + 0.6f32 * ctx.ap,
        3 => 130f32 + 0.6f32 * ctx.ap,
        4 => 165f32 + 0.6f32 * ctx.ap,
        5 => 200f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 90f32 + 0.2f32 * ctx.ap,
        2 => 130f32 + 0.2f32 * ctx.ap,
        3 => 170f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 630f32 + 1.4f32 * ctx.ap,
        2 => 910f32 + 1.4f32 * ctx.ap,
        3 => 1190f32 + 1.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn syndra_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 270f32 + 0.6f32 * ctx.ap,
        2 => 390f32 + 0.6f32 * ctx.ap,
        3 => 510f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static TAHMKENCH: CachedChampion = CachedChampion {
    name: "Tahm Kench",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 325f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 39f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.5f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 175f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1.2f32,
    },
    merge_data: &[],
    closures: &[tahmkench_q_1, tahmkench_w_1, tahmkench_e_1, tahmkench_e_2],
};
pub const fn tahmkench_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn tahmkench_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 100f32 + 1.5f32 * ctx.ap,
        2 => 135f32 + 1.5f32 * ctx.ap,
        3 => 170f32 + 1.5f32 * ctx.ap,
        4 => 205f32 + 1.5f32 * ctx.ap,
        5 => 240f32 + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn tahmkench_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.15f32,
        2 => 0.23f32,
        3 => 0.31f32,
        4 => 0.39f32,
        5 => 0.47f32,
        _ => 0.0,
    }
}
pub const fn tahmkench_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.42f32,
        2 => 0.44f32,
        3 => 0.46f32,
        4 => 0.48f32,
        5 => 0.5f32,
        _ => 0.0,
    }
}
pub static TALIYAH: CachedChampion = CachedChampion {
    name: "Taliyah",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle, Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 550f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 470f32,
            per_level: 30f32,
        },
        armor: CachedChampionStatsMap {
            flat: 18f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 1.36f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        taliyah_q_1,
        taliyah_q_2,
        taliyah_q_3,
        taliyah_q_4,
        taliyah_e_1,
        taliyah_e_2,
        taliyah_e_3,
    ],
};
pub const fn taliyah_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 56f32 + 0.5f32 * ctx.ap,
        2 => 74.5f32 + 0.5f32 * ctx.ap,
        3 => 93f32 + 0.5f32 * ctx.ap,
        4 => 111.5f32 + 0.5f32 * ctx.ap,
        5 => 130f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn taliyah_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 22.4f32 + 0.2f32 * ctx.ap,
        2 => 29.8f32 + 0.2f32 * ctx.ap,
        3 => 37.2f32 + 0.2f32 * ctx.ap,
        4 => 44.6f32 + 0.2f32 * ctx.ap,
        5 => 52f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn taliyah_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 145.6f32 + 1.3f32 * ctx.ap,
        2 => 193.7f32 + 1.3f32 * ctx.ap,
        3 => 241.8f32 + 1.3f32 * ctx.ap,
        4 => 289.9f32 + 1.3f32 * ctx.ap,
        5 => 338f32 + 1.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn taliyah_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 100.8f32 + 0.9f32 * ctx.ap,
        2 => 134.1f32 + 0.9f32 * ctx.ap,
        3 => 167.4f32 + 0.9f32 * ctx.ap,
        4 => 200.7f32 + 0.9f32 * ctx.ap,
        5 => 234f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn taliyah_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 105f32 + 0.6f32 * ctx.ap,
        3 => 150f32 + 0.6f32 * ctx.ap,
        4 => 195f32 + 0.6f32 * ctx.ap,
        5 => 240f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn taliyah_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 62.5f32 + 0.75f32 * ctx.ap,
        2 => 112.5f32 + 0.75f32 * ctx.ap,
        3 => 162.5f32 + 0.75f32 * ctx.ap,
        4 => 212.5f32 + 0.75f32 * ctx.ap,
        5 => 262.5f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn taliyah_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 25f32 + 0.3f32 * ctx.ap,
        2 => 45f32 + 0.3f32 * ctx.ap,
        3 => 65f32 + 0.3f32 * ctx.ap,
        4 => 85f32 + 0.3f32 * ctx.ap,
        5 => 105f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static TALON: CachedChampion = CachedChampion {
    name: "Talon",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 658f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 37f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.9f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        talon_q_1, talon_q_2, talon_w_1, talon_w_2, talon_w_3, talon_r_1,
        talon_r_2,
    ],
};
pub const fn talon_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 97.5f32 + 1.5f32 * ctx.bonus_ad,
        2 => 127.5f32 + 1.5f32 * ctx.bonus_ad,
        3 => 157.5f32 + 1.5f32 * ctx.bonus_ad,
        4 => 187.5f32 + 1.5f32 * ctx.bonus_ad,
        5 => 217.5f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn talon_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 65f32 + ctx.bonus_ad,
        2 => 85f32 + ctx.bonus_ad,
        3 => 105f32 + ctx.bonus_ad,
        4 => 125f32 + ctx.bonus_ad,
        5 => 145f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn talon_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.4f32 * ctx.bonus_ad,
        2 => 60f32 + 0.4f32 * ctx.bonus_ad,
        3 => 70f32 + 0.4f32 * ctx.bonus_ad,
        4 => 80f32 + 0.4f32 * ctx.bonus_ad,
        5 => 90f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn talon_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 60f32 + 0.9f32 * ctx.bonus_ad,
        2 => 90f32 + 0.9f32 * ctx.bonus_ad,
        3 => 120f32 + 0.9f32 * ctx.bonus_ad,
        4 => 150f32 + 0.9f32 * ctx.bonus_ad,
        5 => 180f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn talon_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 110f32 + 1.3f32 * ctx.bonus_ad,
        2 => 150f32 + 1.3f32 * ctx.bonus_ad,
        3 => 190f32 + 1.3f32 * ctx.bonus_ad,
        4 => 230f32 + 1.3f32 * ctx.bonus_ad,
        5 => 270f32 + 1.3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn talon_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 90f32 + ctx.bonus_ad,
        2 => 135f32 + ctx.bonus_ad,
        3 => 180f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn talon_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 180f32 + 2f32 * ctx.bonus_ad,
        2 => 270f32 + 2f32 * ctx.bonus_ad,
        3 => 360f32 + 2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static TARIC: CachedChampion = CachedChampion {
    name: "Taric",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Support],
    metadata: &[TypeMetadata {
        kind: AbilityId::E(AbilityName::_1),
        damage_type: DamageType::Magic,
        attributes: Attrs::Undefined,
    }],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 645f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 40f32,
            per_level: 4.3f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 150f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[taric_e_1],
};
pub const fn taric_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub static TEEMO: CachedChampion = CachedChampion {
    name: "Teemo",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Jungle, Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 615f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 334f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.95f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.69f32,
            per_level: 3.38f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.689999997615814f32,
        attack_range: 500f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        teemo_q_1, teemo_e_1, teemo_e_2, teemo_e_3, teemo_e_4, teemo_e_5,
        teemo_e_6, teemo_r_1, teemo_r_2,
    ],
};
pub const fn teemo_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.7f32 * ctx.ap,
        2 => 125f32 + 0.7f32 * ctx.ap,
        3 => 170f32 + 0.7f32 * ctx.ap,
        4 => 215f32 + 0.7f32 * ctx.ap,
        5 => 260f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 9f32 + 0.3f32 * ctx.ap,
        2 => 23f32 + 0.3f32 * ctx.ap,
        3 => 37f32 + 0.3f32 * ctx.ap,
        4 => 51f32 + 0.3f32 * ctx.ap,
        5 => 65f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 6f32 + 0.1f32 * ctx.ap,
        2 => 12f32 + 0.1f32 * ctx.ap,
        3 => 18f32 + 0.1f32 * ctx.ap,
        4 => 24f32 + 0.1f32 * ctx.ap,
        5 => 30f32 + 0.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 24f32 + 0.4f32 * ctx.ap,
        2 => 48f32 + 0.4f32 * ctx.ap,
        3 => 72f32 + 0.4f32 * ctx.ap,
        4 => 96f32 + 0.4f32 * ctx.ap,
        5 => 120f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 13.05f32 + 0.435f32 * ctx.ap,
        2 => 33.35f32 + 0.435f32 * ctx.ap,
        3 => 53.65f32 + 0.435f32 * ctx.ap,
        4 => 73.95f32 + 0.435f32 * ctx.ap,
        5 => 94.25f32 + 0.435f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_e_5(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 8.7f32 + 0.145f32 * ctx.ap,
        2 => 17.4f32 + 0.145f32 * ctx.ap,
        3 => 26.1f32 + 0.145f32 * ctx.ap,
        4 => 34.8f32 + 0.145f32 * ctx.ap,
        5 => 43.5f32 + 0.145f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_e_6(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 34.8f32 + 0.58f32 * ctx.ap,
        2 => 69.6f32 + 0.58f32 * ctx.ap,
        3 => 104.4f32 + 0.58f32 * ctx.ap,
        4 => 139.2f32 + 0.58f32 * ctx.ap,
        5 => 174f32 + 0.58f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 50f32 + 0.125f32 * ctx.ap,
        2 => 81.25f32 + 0.125f32 * ctx.ap,
        3 => 112.5f32 + 0.125f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn teemo_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.5f32 * ctx.ap,
        2 => 325f32 + 0.5f32 * ctx.ap,
        3 => 450f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static THRESH: CachedChampion = CachedChampion {
    name: "Thresh",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 120f32,
        },
        mana: CachedChampionStatsMap {
            flat: 274f32,
            per_level: 44f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 0f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.55f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 2.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 450f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.87f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[thresh_q_1, thresh_e_1, thresh_e_2, thresh_e_3, thresh_r_1],
};
pub const fn thresh_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 100f32 + 0.9f32 * ctx.ap,
        2 => 150f32 + 0.9f32 * ctx.ap,
        3 => 200f32 + 0.9f32 * ctx.ap,
        4 => 250f32 + 0.9f32 * ctx.ap,
        5 => 300f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn thresh_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32 + 0.7f32 * ctx.ap,
        2 => 120f32 + 0.7f32 * ctx.ap,
        3 => 165f32 + 0.7f32 * ctx.ap,
        4 => 210f32 + 0.7f32 * ctx.ap,
        5 => 255f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn thresh_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 1.7f32 * ctx.thresh_stacks + 0.9f32 * ctx.ad,
        2 => 1.7f32 * ctx.thresh_stacks + 1.2f32 * ctx.ad,
        3 => 1.7f32 * ctx.thresh_stacks + 1.5f32 * ctx.ad,
        4 => 1.7f32 * ctx.thresh_stacks + 1.8f32 * ctx.ad,
        5 => 1.7f32 * ctx.thresh_stacks + 2.1f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn thresh_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,
        2 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,
        3 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,
        4 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,
        5 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn thresh_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 250f32 + ctx.ap,
        2 => 400f32 + ctx.ap,
        3 => 550f32 + ctx.ap,
        _ => 0.0,
    }
}
pub static TRISTANA: CachedChampion = CachedChampion {
    name: "Tristana",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 102f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 32f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.656f32,
            per_level: 1.5f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[
        tristana_w_1,
        tristana_e_1,
        tristana_e_2,
        tristana_e_3,
        tristana_e_4,
        tristana_e_5,
        tristana_r_1,
    ],
};
pub const fn tristana_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 70f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 105f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 140f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 175f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 210f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn tristana_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 70f32 + 1.1f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 80f32 + 1.2f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 90f32 + 1.3f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 100f32 + 1.4f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn tristana_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 45f32 + 0.25f32 * ctx.ap,
        2 => 60f32 + 0.25f32 * ctx.ap,
        3 => 75f32 + 0.25f32 * ctx.ap,
        4 => 90f32 + 0.25f32 * ctx.ap,
        5 => 105f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn tristana_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 15f32 + 0.25f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,
        2 => 17.5f32 + 0.275f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,
        3 => 20f32 + 0.3f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,
        4 => 22.5f32 + 0.325f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,
        5 => 25f32 + 0.35f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn tristana_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,
        2 => 70f32 + 1.1f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        3 => 80f32 + 1.2f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        4 => 90f32 + 1.3f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        5 => 100f32 + 1.4f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn tristana_e_5(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 120f32 + 2f32 * ctx.bonus_ad + ctx.ap,
        2 => 140f32 + 2.2f32 * ctx.bonus_ad + ctx.ap,
        3 => 160f32 + 2.4f32 * ctx.bonus_ad + ctx.ap,
        4 => 180f32 + 2.6f32 * ctx.bonus_ad + ctx.ap,
        5 => 200f32 + 2.8f32 * ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn tristana_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 225f32 + 0.7f32 * ctx.bonus_ad + ctx.ap,
        2 => 275f32 + 0.7f32 * ctx.bonus_ad + ctx.ap,
        3 => 325f32 + 0.7f32 * ctx.bonus_ad + ctx.ap,
        _ => 0.0,
    }
}
pub static TRUNDLE: CachedChampion = CachedChampion {
    name: "Trundle",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 340f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 37f32,
            per_level: 3.9f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 68f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.67f32,
            per_level: 2.9f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.6700000166893f32,
        attack_range: 175f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.95f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        trundle_q_1,
        trundle_q_2,
        trundle_q_3,
        trundle_r_1,
        trundle_r_2,
        trundle_r_3,
    ],
};
pub const fn trundle_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.15f32 * ctx.ad,
        2 => 30f32 + 0.25f32 * ctx.ad,
        3 => 50f32 + 0.35f32 * ctx.ad,
        4 => 70f32 + 0.45f32 * ctx.ad,
        5 => 90f32 + 0.55f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn trundle_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32,
        2 => 12.5f32,
        3 => 15f32,
        4 => 17.5f32,
        5 => 20f32,
        _ => 0.0,
    }
}
pub const fn trundle_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32,
        2 => 25f32,
        3 => 30f32,
        4 => 35f32,
        5 => 40f32,
        _ => 0.0,
    }
}
pub const fn trundle_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.2f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,
        2 => 0.25f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,
        3 => 0.3f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn trundle_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.1f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        2 => 0.125f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        3 => 0.15f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn trundle_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.025f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,
        2 => 0.03125f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,
        3 => 0.0375f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static TRYNDAMERE: CachedChampion = CachedChampion {
    name: "Tryndamere",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 696f32,
            per_level: 108f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.8f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 66f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.67f32,
            per_level: 3.4f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.694f32,
        attack_range: 175f32,
        aram_damage_taken: 0.9f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[tryndamere_q_1, tryndamere_w_1, tryndamere_e_1],
};
pub const fn tryndamere_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32,
        2 => 10f32,
        3 => 15f32,
        4 => 20f32,
        5 => 25f32,
        _ => 0.0,
    }
}
pub const fn tryndamere_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32,
        2 => 35f32,
        3 => 50f32,
        4 => 65f32,
        5 => 80f32,
        _ => 0.0,
    }
}
pub const fn tryndamere_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        2 => 105f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        3 => 135f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        4 => 165f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        5 => 195f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static TWISTEDFATE: CachedChampion = CachedChampion {
    name: "Twisted Fate",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 604f32,
            per_level: 108f32,
        },
        mana: CachedChampionStatsMap {
            flat: 333f32,
            per_level: 39f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.35f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 52f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.651000022888183f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        twistedfate_q_1,
        twistedfate_w_1,
        twistedfate_w_2,
        twistedfate_w_3,
        twistedfate_e_1,
    ],
};
pub const fn twistedfate_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,
        2 => 105f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,
        3 => 150f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,
        4 => 195f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,
        5 => 240f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn twistedfate_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 40f32 + ctx.ad + ctx.ap,
        2 => 60f32 + ctx.ad + ctx.ap,
        3 => 80f32 + ctx.ad + ctx.ap,
        4 => 100f32 + ctx.ad + ctx.ap,
        5 => 120f32 + ctx.ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn twistedfate_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 15f32 + ctx.ad + 0.5f32 * ctx.ap,
        2 => 22.5f32 + ctx.ad + 0.5f32 * ctx.ap,
        3 => 30f32 + ctx.ad + 0.5f32 * ctx.ap,
        4 => 37.5f32 + ctx.ad + 0.5f32 * ctx.ap,
        5 => 45f32 + ctx.ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn twistedfate_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + ctx.ad + 0.7f32 * ctx.ap,
        2 => 45f32 + ctx.ad + 0.7f32 * ctx.ap,
        3 => 60f32 + ctx.ad + 0.7f32 * ctx.ap,
        4 => 75f32 + ctx.ad + 0.7f32 * ctx.ap,
        5 => 90f32 + ctx.ad + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn twistedfate_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 65f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,
        2 => 90f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,
        3 => 115f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,
        4 => 140f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,
        5 => 165f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static TWITCH: CachedChampion = CachedChampion {
    name: "Twitch",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.679f32,
            per_level: 3.38f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.67900002002716f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[twitch_e_1, twitch_e_2, twitch_e_3, twitch_e_4, twitch_r_1],
};
pub const fn twitch_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32,
        2 => 30f32,
        3 => 40f32,
        4 => 50f32,
        5 => 60f32,
        _ => 0.0,
    }
}
pub const fn twitch_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 110f32 + 2.1f32 * ctx.bonus_ad + 2.1f32 * ctx.ap,
        2 => 150f32 + 2.1f32 * ctx.bonus_ad + 2.1f32 * ctx.ap,
        3 => 190f32 + 2.1f32 * ctx.bonus_ad + 2.1f32 * ctx.ap,
        4 => 230f32 + 2.1f32 * ctx.bonus_ad + 2.1f32 * ctx.ap,
        5 => 270f32 + 2.1f32 * ctx.bonus_ad + 2.1f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn twitch_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 35f32 + 0.35f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        2 => 50f32 + 0.35f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        3 => 65f32 + 0.35f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        4 => 80f32 + 0.35f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        5 => 95f32 + 0.35f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn twitch_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 15f32 + 0.35f32 * ctx.bonus_ad,
        2 => 20f32 + 0.35f32 * ctx.bonus_ad,
        3 => 25f32 + 0.35f32 * ctx.bonus_ad,
        4 => 30f32 + 0.35f32 * ctx.bonus_ad,
        5 => 35f32 + 0.35f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn twitch_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 30f32,
        2 => 45f32,
        3 => 60f32,
        _ => 0.0,
    }
}
pub static UDYR: CachedChampion = CachedChampion {
    name: "Udyr",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 664f32,
            per_level: 92f32,
        },
        mana: CachedChampionStatsMap {
            flat: 271f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.65f32,
            per_level: 3f32,
        },
        movespeed: 350f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.6499999761581421f32,
        attack_range: 125f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.92f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[udyr_q_1, udyr_q_2, udyr_q_3, udyr_r_1, udyr_r_2],
};
pub const fn udyr_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.03f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,
        2 => 0.04f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,
        3 => 0.05f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,
        4 => 0.06f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,
        5 => 0.07f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,
        6 => 0.08f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn udyr_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.06f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,
        2 => 0.08f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,
        3 => 0.1f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,
        4 => 0.12f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,
        5 => 0.14f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,
        6 => 0.16f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn udyr_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.25f32 * ctx.bonus_ad,
        2 => 11f32 + 0.25f32 * ctx.bonus_ad,
        3 => 17f32 + 0.25f32 * ctx.bonus_ad,
        4 => 23f32 + 0.25f32 * ctx.bonus_ad,
        5 => 29f32 + 0.25f32 * ctx.bonus_ad,
        6 => 35f32 + 0.25f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn udyr_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 10f32 + 0.175f32 * ctx.ap,
        2 => 18f32 + 0.175f32 * ctx.ap,
        3 => 26f32 + 0.175f32 * ctx.ap,
        4 => 34f32 + 0.175f32 * ctx.ap,
        5 => 42f32 + 0.175f32 * ctx.ap,
        6 => 50f32 + 0.175f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn udyr_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 80f32 + 1.4f32 * ctx.ap,
        2 => 144f32 + 1.4f32 * ctx.ap,
        3 => 208f32 + 1.4f32 * ctx.ap,
        4 => 272f32 + 1.4f32 * ctx.ap,
        5 => 336f32 + 1.4f32 * ctx.ap,
        6 => 400f32 + 1.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static URGOT: CachedChampion = CachedChampion {
    name: "Urgot",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 102f32,
        },
        mana: CachedChampionStatsMap {
            flat: 340f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.75f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 350f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.85f32,
        urf_damage_dealt: 1.15f32,
    },
    merge_data: &[],
    closures: &[urgot_q_1, urgot_w_1, urgot_e_1, urgot_r_1],
};
pub const fn urgot_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 25f32 + 0.7f32 * ctx.ad,
        2 => 70f32 + 0.7f32 * ctx.ad,
        3 => 115f32 + 0.7f32 * ctx.ad,
        4 => 160f32 + 0.7f32 * ctx.ad,
        5 => 205f32 + 0.7f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn urgot_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 12f32 + 0.2f32 * ctx.ad,
        2 => 12f32 + 0.235f32 * ctx.ad,
        3 => 12f32 + 0.27f32 * ctx.ad,
        4 => 12f32 + 0.305f32 * ctx.ad,
        5 => 12f32 + 0.34f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn urgot_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 90f32 + ctx.bonus_ad,
        2 => 120f32 + ctx.bonus_ad,
        3 => 150f32 + ctx.bonus_ad,
        4 => 180f32 + ctx.bonus_ad,
        5 => 210f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn urgot_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.5f32 * ctx.bonus_ad,
        2 => 225f32 + 0.5f32 * ctx.bonus_ad,
        3 => 350f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static VARUS: CachedChampion = CachedChampion {
    name: "Varus",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_5),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_6),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_7),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 320f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 59f32,
            per_level: 3.4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.5f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 575f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        varus_q_1, varus_q_2, varus_q_3, varus_q_4, varus_w_1, varus_w_2,
        varus_w_3, varus_w_4, varus_w_5, varus_w_6, varus_w_7, varus_e_1,
        varus_r_1,
    ],
};
pub const fn varus_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + 1.3f32 * ctx.bonus_ad,
        2 => 160f32 + 1.4f32 * ctx.bonus_ad,
        3 => 230f32 + 1.5f32 * ctx.bonus_ad,
        4 => 300f32 + 1.6f32 * ctx.bonus_ad,
        5 => 370f32 + 1.7f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn varus_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 29.7f32 + 0.429f32 * ctx.bonus_ad,
        2 => 52.8f32 + 0.462f32 * ctx.bonus_ad,
        3 => 75.9f32 + 0.495f32 * ctx.bonus_ad,
        4 => 99f32 + 0.528f32 * ctx.bonus_ad,
        5 => 122.1f32 + 0.561f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn varus_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.8667f32 * ctx.bonus_ad,
        2 => 106.67f32 + 0.9333f32 * ctx.bonus_ad,
        3 => 153.33f32 + ctx.bonus_ad,
        4 => 200f32 + 1.0667f32 * ctx.bonus_ad,
        5 => 246.67f32 + 1.1333f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn varus_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 19.8f32 + 0.28600000000000003f32 * ctx.bonus_ad,
        2 => 35.2f32 + 0.308f32 * ctx.bonus_ad,
        3 => 50.6f32 + 0.33f32 * ctx.bonus_ad,
        4 => 66f32 + 0.35200000000000004f32 * ctx.bonus_ad,
        5 => 81.4f32 + 0.374f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn varus_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.09f32 * ctx.missing_health,
        2 => 0.12f32 * ctx.missing_health,
        3 => 0.15f32 * ctx.missing_health,
        4 => 0.18f32 * ctx.missing_health,
        5 => 0.21f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn varus_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.06f32 * ctx.missing_health,
        2 => 0.08f32 * ctx.missing_health,
        3 => 0.1f32 * ctx.missing_health,
        4 => 0.12f32 * ctx.missing_health,
        5 => 0.14f32 * ctx.missing_health,
        _ => 0.0,
    }
}
pub const fn varus_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.09f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,
        2 => 0.105f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,
        3 => 0.12f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,
        4 => 0.135f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,
        5 => 0.15f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn varus_w_4(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,
        2 => 0.035f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,
        3 => 0.04f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,
        4 => 0.045f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,
        5 => 0.05f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn varus_w_5(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.135f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,
        2 => 0.1575f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,
        3 => 0.18f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,
        4 => 0.2025f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,
        5 => 0.225f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn varus_w_6(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.045f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,
        2 => 0.0525f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,
        3 => 0.06f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,
        4 => 0.0675f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,
        5 => 0.075f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn varus_w_7(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 6f32 + 0.35f32 * ctx.ap,
        2 => 12f32 + 0.35f32 * ctx.ap,
        3 => 18f32 + 0.35f32 * ctx.ap,
        4 => 24f32 + 0.35f32 * ctx.ap,
        5 => 30f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn varus_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + ctx.bonus_ad,
        2 => 100f32 + ctx.bonus_ad,
        3 => 140f32 + ctx.bonus_ad,
        4 => 180f32 + ctx.bonus_ad,
        5 => 220f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn varus_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + ctx.ap,
        2 => 250f32 + ctx.ap,
        3 => 350f32 + ctx.ap,
        _ => 0.0,
    }
}
pub static VAYNE: CachedChampion = CachedChampion {
    name: "Vayne",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 550f32,
            per_level: 103f32,
        },
        mana: CachedChampionStatsMap {
            flat: 232f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 23f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.35f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.3f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        vayne_q_1, vayne_w_1, vayne_w_2, vayne_e_1, vayne_e_2, vayne_e_3,
        vayne_r_1,
    ],
};
pub const fn vayne_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.75f32 * ctx.ad + 0.5f32 * ctx.ap,
        2 => 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,
        3 => 0.95f32 * ctx.ad + 0.5f32 * ctx.ap,
        4 => 1.05f32 * ctx.ad + 0.5f32 * ctx.ap,
        5 => 1.15f32 * ctx.ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vayne_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.06f32 * ctx.enemy_max_health,
        2 => 0.07f32 * ctx.enemy_max_health,
        3 => 0.08f32 * ctx.enemy_max_health,
        4 => 0.09f32 * ctx.enemy_max_health,
        5 => 0.1f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn vayne_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32,
        2 => 65f32,
        3 => 80f32,
        4 => 95f32,
        5 => 110f32,
        _ => 0.0,
    }
}
pub const fn vayne_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.5f32 * ctx.bonus_ad,
        2 => 85f32 + 0.5f32 * ctx.bonus_ad,
        3 => 120f32 + 0.5f32 * ctx.bonus_ad,
        4 => 155f32 + 0.5f32 * ctx.bonus_ad,
        5 => 190f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn vayne_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 75f32 + 0.75f32 * ctx.bonus_ad,
        2 => 127.5f32 + 0.75f32 * ctx.bonus_ad,
        3 => 180f32 + 0.75f32 * ctx.bonus_ad,
        4 => 232.5f32 + 0.75f32 * ctx.bonus_ad,
        5 => 285f32 + 0.75f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn vayne_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 125f32 + 1.25f32 * ctx.bonus_ad,
        2 => 212.5f32 + 1.25f32 * ctx.bonus_ad,
        3 => 300f32 + 1.25f32 * ctx.bonus_ad,
        4 => 387.5f32 + 1.25f32 * ctx.bonus_ad,
        5 => 475f32 + 1.25f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn vayne_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 35f32,
        2 => 50f32,
        3 => 65f32,
        _ => 0.0,
    }
}
pub static VEIGAR: CachedChampion = CachedChampion {
    name: "Veigar",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 580f32,
            per_level: 108f32,
        },
        mana: CachedChampionStatsMap {
            flat: 490f32,
            per_level: 26f32,
        },
        armor: CachedChampionStatsMap {
            flat: 18f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 52f32,
            per_level: 2.7f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.24f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1.1f32,
        aram_damage_dealt: 0.93f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[veigar_q_1, veigar_w_1, veigar_r_1, veigar_r_2],
};
pub const fn veigar_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.5f32 * ctx.ap,
        2 => 120f32 + 0.55f32 * ctx.ap,
        3 => 160f32 + 0.6f32 * ctx.ap,
        4 => 200f32 + 0.65f32 * ctx.ap,
        5 => 240f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn veigar_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 85f32 + 0.6f32 * ctx.ap,
        2 => 140f32 + 0.7f32 * ctx.ap,
        3 => 195f32 + 0.8f32 * ctx.ap,
        4 => 250f32 + 0.9f32 * ctx.ap,
        5 => 305f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn veigar_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 350f32 + 1.3f32 * ctx.ap,
        2 => 500f32 + 1.4f32 * ctx.ap,
        3 => 650f32 + 1.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn veigar_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 0.65f32 * ctx.ap,
        2 => 250f32 + 0.7f32 * ctx.ap,
        3 => 325f32 + 0.75f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static VELKOZ: CachedChampion = CachedChampion {
    name: "Vel'Koz",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 102f32,
        },
        mana: CachedChampionStatsMap {
            flat: 469f32,
            per_level: 21f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3.1416f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.643f32,
            per_level: 1.59f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        velkoz_q_1, velkoz_w_1, velkoz_w_2, velkoz_w_3, velkoz_e_1, velkoz_r_1,
        velkoz_r_2,
    ],
};
pub const fn velkoz_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.9f32 * ctx.ap,
        2 => 120f32 + 0.9f32 * ctx.ap,
        3 => 160f32 + 0.9f32 * ctx.ap,
        4 => 200f32 + 0.9f32 * ctx.ap,
        5 => 240f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn velkoz_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.2f32 * ctx.ap,
        2 => 50f32 + 0.2f32 * ctx.ap,
        3 => 70f32 + 0.2f32 * ctx.ap,
        4 => 90f32 + 0.2f32 * ctx.ap,
        5 => 110f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn velkoz_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 45f32 + 0.25f32 * ctx.ap,
        2 => 75f32 + 0.25f32 * ctx.ap,
        3 => 105f32 + 0.25f32 * ctx.ap,
        4 => 135f32 + 0.25f32 * ctx.ap,
        5 => 165f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn velkoz_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 75f32 + 0.45f32 * ctx.ap,
        2 => 125f32 + 0.45f32 * ctx.ap,
        3 => 175f32 + 0.45f32 * ctx.ap,
        4 => 225f32 + 0.45f32 * ctx.ap,
        5 => 275f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn velkoz_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.3f32 * ctx.ap,
        2 => 100f32 + 0.3f32 * ctx.ap,
        3 => 130f32 + 0.3f32 * ctx.ap,
        4 => 160f32 + 0.3f32 * ctx.ap,
        5 => 190f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn velkoz_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 34.62f32 + 0.0962f32 * ctx.ap,
        2 => 48.08f32 + 0.0962f32 * ctx.ap,
        3 => 61.54f32 + 0.0962f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn velkoz_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 450f32 + 1.25f32 * ctx.ap,
        2 => 625f32 + 1.25f32 * ctx.ap,
        3 => 800f32 + 1.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static VEX: CachedChampion = CachedChampion {
    name: "Vex",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 490f32,
            per_level: 32f32,
        },
        armor: CachedChampionStatsMap {
            flat: 23f32,
            per_level: 4.45f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 28f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 54f32,
            per_level: 2.75f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.669f32,
            per_level: 1f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.9f32,
    },
    merge_data: &[],
    closures: &[vex_q_1, vex_w_1, vex_e_1, vex_r_1, vex_r_2, vex_r_3],
};
pub const fn vex_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 70f32 + 0.7f32 * ctx.ap,
        2 => 115f32 + 0.7f32 * ctx.ap,
        3 => 160f32 + 0.7f32 * ctx.ap,
        4 => 205f32 + 0.7f32 * ctx.ap,
        5 => 250f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vex_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 0.3f32 * ctx.ap,
        2 => 120f32 + 0.3f32 * ctx.ap,
        3 => 160f32 + 0.3f32 * ctx.ap,
        4 => 200f32 + 0.3f32 * ctx.ap,
        5 => 240f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vex_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.4f32 * ctx.ap,
        2 => 70f32 + 0.45f32 * ctx.ap,
        3 => 90f32 + 0.5f32 * ctx.ap,
        4 => 110f32 + 0.55f32 * ctx.ap,
        5 => 130f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vex_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 75f32 + 0.2f32 * ctx.ap,
        2 => 125f32 + 0.2f32 * ctx.ap,
        3 => 175f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vex_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.ap,
        2 => 250f32 + 0.5f32 * ctx.ap,
        3 => 350f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vex_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 225f32 + 0.7f32 * ctx.ap,
        2 => 375f32 + 0.7f32 * ctx.ap,
        3 => 525f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static VI: CachedChampion = CachedChampion {
    name: "Vi",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 655f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 295f32,
            per_level: 65f32,
        },
        armor: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.644f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.643999993801116f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[vi_q_1, vi_q_2, vi_w_1, vi_e_1, vi_r_1],
};
pub const fn vi_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 100f32 + 1.5f32 * ctx.bonus_ad,
        2 => 150f32 + 1.5f32 * ctx.bonus_ad,
        3 => 200f32 + 1.5f32 * ctx.bonus_ad,
        4 => 250f32 + 1.5f32 * ctx.bonus_ad,
        5 => 300f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn vi_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.6f32 * ctx.bonus_ad,
        2 => 60f32 + 0.6f32 * ctx.bonus_ad,
        3 => 80f32 + 0.6f32 * ctx.bonus_ad,
        4 => 100f32 + 0.6f32 * ctx.bonus_ad,
        5 => 120f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn vi_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 0.04f32 * ctx.enemy_max_health + 0.035f32 * 0.01f32 * ctx.bonus_ad,
        2 => 0.05f32 * ctx.enemy_max_health + 0.035f32 * 0.01f32 * ctx.bonus_ad,
        3 => 0.06f32 * ctx.enemy_max_health + 0.035f32 * 0.01f32 * ctx.bonus_ad,
        4 => 0.07f32 * ctx.enemy_max_health + 0.035f32 * 0.01f32 * ctx.bonus_ad,
        5 => 0.08f32 * ctx.enemy_max_health + 0.035f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn vi_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 10f32 + 1.1f32 * ctx.ad + ctx.ap,
        2 => 30f32 + 1.1f32 * ctx.ad + ctx.ap,
        3 => 50f32 + 1.1f32 * ctx.ad + ctx.ap,
        4 => 70f32 + 1.1f32 * ctx.ad + ctx.ap,
        5 => 90f32 + 1.1f32 * ctx.ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn vi_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.9f32 * ctx.bonus_ad,
        2 => 250f32 + 0.9f32 * ctx.bonus_ad,
        3 => 350f32 + 0.9f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static VIEGO: CachedChampion = CachedChampion {
    name: "Viego",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_6),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 34f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 57f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 200f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[
        viego_q_1, viego_q_2, viego_q_3, viego_q_4, viego_q_5, viego_q_6,
        viego_w_1, viego_r_1,
    ],
};
pub const fn viego_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 1.4f32 * ctx.ad,
        2 => 90f32 + 1.4f32 * ctx.ad,
        3 => 120f32 + 1.4f32 * ctx.ad,
        4 => 150f32 + 1.4f32 * ctx.ad,
        5 => 180f32 + 1.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn viego_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 1.4f32 * ctx.ad,
        2 => 60f32 + 1.4f32 * ctx.ad,
        3 => 90f32 + 1.4f32 * ctx.ad,
        4 => 120f32 + 1.4f32 * ctx.ad,
        5 => 150f32 + 1.4f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn viego_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.7f32 * ctx.ad,
        2 => 45f32 + 0.7f32 * ctx.ad,
        3 => 60f32 + 0.7f32 * ctx.ad,
        4 => 75f32 + 0.7f32 * ctx.ad,
        5 => 90f32 + 0.7f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn viego_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15f32 + 0.7f32 * ctx.ad,
        2 => 30f32 + 0.7f32 * ctx.ad,
        3 => 45f32 + 0.7f32 * ctx.ad,
        4 => 60f32 + 0.7f32 * ctx.ad,
        5 => 75f32 + 0.7f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn viego_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.02f32 * ctx.current_health,
        2 => 0.03f32 * ctx.current_health,
        3 => 0.04f32 * ctx.current_health,
        4 => 0.05f32 * ctx.current_health,
        5 => 0.06f32 * ctx.current_health,
        _ => 0.0,
    }
}
pub const fn viego_q_6(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32,
        2 => 15f32,
        3 => 20f32,
        4 => 25f32,
        5 => 30f32,
        _ => 0.0,
    }
}
pub const fn viego_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + ctx.ap,
        2 => 135f32 + ctx.ap,
        3 => 190f32 + ctx.ap,
        4 => 245f32 + ctx.ap,
        5 => 300f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn viego_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.12f32 * ctx.missing_health + 0.05f32 * 0.01f32 * ctx.bonus_ad,
        2 => 0.16f32 * ctx.missing_health + 0.05f32 * 0.01f32 * ctx.bonus_ad,
        3 => 0.2f32 * ctx.missing_health + 0.05f32 * 0.01f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static VIKTOR: CachedChampion = CachedChampion {
    name: "Viktor",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 100f32,
        },
        mana: CachedChampionStatsMap {
            flat: 405f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 23f32,
            per_level: 4.4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 53f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.11f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 525f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[
        viktor_q_1, viktor_q_2, viktor_q_3, viktor_e_1, viktor_e_2, viktor_e_3,
        viktor_r_1, viktor_r_2, viktor_r_3,
    ],
};
pub const fn viktor_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.4f32 * ctx.ap,
        2 => 75f32 + 0.4f32 * ctx.ap,
        3 => 90f32 + 0.4f32 * ctx.ap,
        4 => 105f32 + 0.4f32 * ctx.ap,
        5 => 120f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + ctx.ad + 0.5f32 * ctx.ap,
        2 => 45f32 + ctx.ad + 0.5f32 * ctx.ap,
        3 => 70f32 + ctx.ad + 0.5f32 * ctx.ap,
        4 => 95f32 + ctx.ad + 0.5f32 * ctx.ap,
        5 => 120f32 + ctx.ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + ctx.ad + 0.9f32 * ctx.ap,
        2 => 120f32 + ctx.ad + 0.9f32 * ctx.ap,
        3 => 160f32 + ctx.ad + 0.9f32 * ctx.ap,
        4 => 200f32 + ctx.ad + 0.9f32 * ctx.ap,
        5 => 240f32 + ctx.ad + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.5f32 * ctx.ap,
        2 => 110f32 + 0.5f32 * ctx.ap,
        3 => 150f32 + 0.5f32 * ctx.ap,
        4 => 190f32 + 0.5f32 * ctx.ap,
        5 => 230f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 20f32 + 0.8f32 * ctx.ap,
        2 => 50f32 + 0.8f32 * ctx.ap,
        3 => 80f32 + 0.8f32 * ctx.ap,
        4 => 110f32 + 0.8f32 * ctx.ap,
        5 => 140f32 + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 90f32 + 1.3f32 * ctx.ap,
        2 => 160f32 + 1.3f32 * ctx.ap,
        3 => 230f32 + 1.3f32 * ctx.ap,
        4 => 300f32 + 1.3f32 * ctx.ap,
        5 => 370f32 + 1.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.5f32 * ctx.ap,
        2 => 175f32 + 0.5f32 * ctx.ap,
        3 => 250f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 65f32 + 0.35f32 * ctx.ap,
        2 => 105f32 + 0.35f32 * ctx.ap,
        3 => 145f32 + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn viktor_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 490f32 + 2.6f32 * ctx.ap,
        2 => 805f32 + 2.6f32 * ctx.ap,
        3 => 1120f32 + 2.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static VLADIMIR: CachedChampion = CachedChampion {
    name: "Vladimir",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 607f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 2f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 27f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.657999992370605f32,
        attack_range: 450f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.05f32,
        urf_damage_dealt: 0.92f32,
    },
    merge_data: &[],
    closures: &[
        vladimir_q_1,
        vladimir_q_2,
        vladimir_w_1,
        vladimir_w_2,
        vladimir_e_1,
        vladimir_e_2,
        vladimir_r_1,
    ],
};
pub const fn vladimir_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.6f32 * ctx.ap,
        2 => 100f32 + 0.6f32 * ctx.ap,
        3 => 120f32 + 0.6f32 * ctx.ap,
        4 => 140f32 + 0.6f32 * ctx.ap,
        5 => 160f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vladimir_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 148f32 + 1.11f32 * ctx.ap,
        2 => 185f32 + 1.11f32 * ctx.ap,
        3 => 222f32 + 1.11f32 * ctx.ap,
        4 => 259f32 + 1.11f32 * ctx.ap,
        5 => 296f32 + 1.11f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vladimir_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 20f32 + 0.0375f32 * ctx.bonus_health,
        2 => 33.75f32 + 0.0375f32 * ctx.bonus_health,
        3 => 47.5f32 + 0.0375f32 * ctx.bonus_health,
        4 => 61.25f32 + 0.0375f32 * ctx.bonus_health,
        5 => 75f32 + 0.0375f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn vladimir_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 0.15f32 * ctx.bonus_health,
        2 => 135f32 + 0.15f32 * ctx.bonus_health,
        3 => 190f32 + 0.15f32 * ctx.bonus_health,
        4 => 245f32 + 0.15f32 * ctx.bonus_health,
        5 => 300f32 + 0.15f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn vladimir_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 60f32 + 0.06f32 * ctx.max_health + 0.8f32 * ctx.ap,
        2 => 90f32 + 0.06f32 * ctx.max_health + 0.8f32 * ctx.ap,
        3 => 120f32 + 0.06f32 * ctx.max_health + 0.8f32 * ctx.ap,
        4 => 150f32 + 0.06f32 * ctx.max_health + 0.8f32 * ctx.ap,
        5 => 180f32 + 0.06f32 * ctx.max_health + 0.8f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vladimir_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 30f32 + 0.015f32 * ctx.max_health + 0.35f32 * ctx.ap,
        2 => 45f32 + 0.015f32 * ctx.max_health + 0.35f32 * ctx.ap,
        3 => 60f32 + 0.015f32 * ctx.max_health + 0.35f32 * ctx.ap,
        4 => 75f32 + 0.015f32 * ctx.max_health + 0.35f32 * ctx.ap,
        5 => 90f32 + 0.015f32 * ctx.max_health + 0.35f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn vladimir_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.7f32 * ctx.ap,
        2 => 250f32 + 0.7f32 * ctx.ap,
        3 => 350f32 + 0.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static VOLIBEAR: CachedChampion = CachedChampion {
    name: "Volibear",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 104f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 70f32,
        },
        armor: CachedChampionStatsMap {
            flat: 31f32,
            per_level: 5.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.7f32,
        attack_range: 150f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.95f32,
    },
    merge_data: &[],
    closures: &[volibear_q_1, volibear_w_1, volibear_e_1, volibear_r_1],
};
pub const fn volibear_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 1.2f32 * ctx.bonus_ad,
        2 => 30f32 + 1.2f32 * ctx.bonus_ad,
        3 => 50f32 + 1.2f32 * ctx.bonus_ad,
        4 => 70f32 + 1.2f32 * ctx.bonus_ad,
        5 => 90f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn volibear_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + ctx.ad + 0.06f32 * ctx.bonus_health,
        2 => 30f32 + ctx.ad + 0.06f32 * ctx.bonus_health,
        3 => 55f32 + ctx.ad + 0.06f32 * ctx.bonus_health,
        4 => 80f32 + ctx.ad + 0.06f32 * ctx.bonus_health,
        5 => 105f32 + ctx.ad + 0.06f32 * ctx.bonus_health,
        _ => 0.0,
    }
}
pub const fn volibear_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 80f32 + 0.8f32 * ctx.ap + 0.11f32 * ctx.enemy_max_health,
        2 => 110f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.enemy_max_health,
        3 => 140f32 + 0.8f32 * ctx.ap + 0.13f32 * ctx.enemy_max_health,
        4 => 170f32 + 0.8f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,
        5 => 200f32 + 0.8f32 * ctx.ap + 0.15f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn volibear_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 300f32 + 2.5f32 * ctx.bonus_ad + 1.25f32 * ctx.ap,
        2 => 500f32 + 2.5f32 * ctx.bonus_ad + 1.25f32 * ctx.ap,
        3 => 700f32 + 2.5f32 * ctx.bonus_ad + 1.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static WARWICK: CachedChampion = CachedChampion {
    name: "Warwick",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 280f32,
            per_level: 35f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.75f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.638f32,
            per_level: 2.3f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[warwick_q_1, warwick_q_2, warwick_e_1, warwick_r_1],
};
pub const fn warwick_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 0.06f32 * ctx.enemy_max_health + 1.2f32 * ctx.ad + ctx.ap,
        2 => 0.07f32 * ctx.enemy_max_health + 1.2f32 * ctx.ad + ctx.ap,
        3 => 0.08f32 * ctx.enemy_max_health + 1.2f32 * ctx.ad + ctx.ap,
        4 => 0.09f32 * ctx.enemy_max_health + 1.2f32 * ctx.ad + ctx.ap,
        5 => 0.1f32 * ctx.enemy_max_health + 1.2f32 * ctx.ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn warwick_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 150f32 + 1.2f32 * ctx.ad + ctx.ap,
        2 => 165f32 + 1.2f32 * ctx.ad + ctx.ap,
        3 => 180f32 + 1.2f32 * ctx.ad + ctx.ap,
        4 => 195f32 + 1.2f32 * ctx.ad + ctx.ap,
        5 => 210f32 + 1.2f32 * ctx.ad + ctx.ap,
        _ => 0.0,
    }
}
pub const fn warwick_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.35f32,
        2 => 0.4f32,
        3 => 0.45f32,
        4 => 0.5f32,
        5 => 0.55f32,
        _ => 0.0,
    }
}
pub const fn warwick_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 175f32 + 1.67f32 * ctx.bonus_ad,
        2 => 350f32 + 1.67f32 * ctx.bonus_ad,
        3 => 525f32 + 1.67f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static XAYAH: CachedChampion = CachedChampion {
    name: "Xayah",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 107f32,
        },
        mana: CachedChampionStatsMap {
            flat: 340f32,
            per_level: 40f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 3.9f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.658f32,
        attack_range: 525f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[
        xayah_q_1, xayah_q_2, xayah_q_3, xayah_q_4, xayah_e_1, xayah_e_2,
        xayah_r_1,
    ],
};
pub const fn xayah_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.5f32 * ctx.bonus_ad,
        2 => 60f32 + 0.5f32 * ctx.bonus_ad,
        3 => 75f32 + 0.5f32 * ctx.bonus_ad,
        4 => 90f32 + 0.5f32 * ctx.bonus_ad,
        5 => 105f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xayah_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 90f32 + ctx.bonus_ad,
        2 => 120f32 + ctx.bonus_ad,
        3 => 150f32 + ctx.bonus_ad,
        4 => 180f32 + ctx.bonus_ad,
        5 => 210f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xayah_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 22.5f32 + 0.25f32 * ctx.bonus_ad,
        2 => 30f32 + 0.25f32 * ctx.bonus_ad,
        3 => 37.5f32 + 0.25f32 * ctx.bonus_ad,
        4 => 45f32 + 0.25f32 * ctx.bonus_ad,
        5 => 52.5f32 + 0.25f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xayah_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 45f32 + 0.5f32 * ctx.bonus_ad,
        2 => 60f32 + 0.5f32 * ctx.bonus_ad,
        3 => 75f32 + 0.5f32 * ctx.bonus_ad,
        4 => 90f32 + 0.5f32 * ctx.bonus_ad,
        5 => 105f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xayah_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 27.5f32 + 0.3f32 * ctx.bonus_ad,
        2 => 32.5f32 + 0.3f32 * ctx.bonus_ad,
        3 => 37.5f32 + 0.3f32 * ctx.bonus_ad,
        4 => 42.5f32 + 0.3f32 * ctx.bonus_ad,
        5 => 47.5f32 + 0.3f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xayah_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 55f32 + 0.6f32 * ctx.bonus_ad,
        2 => 65f32 + 0.6f32 * ctx.bonus_ad,
        3 => 75f32 + 0.6f32 * ctx.bonus_ad,
        4 => 85f32 + 0.6f32 * ctx.bonus_ad,
        5 => 95f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xayah_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + ctx.bonus_ad,
        2 => 300f32 + ctx.bonus_ad,
        3 => 400f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static XERATH: CachedChampion = CachedChampion {
    name: "Xerath",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 596f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 400f32,
            per_level: 22f32,
        },
        armor: CachedChampionStatsMap {
            flat: 22f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 1.36f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 525f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.93f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        xerath_q_1, xerath_w_1, xerath_w_2, xerath_e_1, xerath_r_1, xerath_r_2,
        xerath_r_3,
    ],
};
pub const fn xerath_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.9f32 * ctx.ap,
        2 => 115f32 + 0.9f32 * ctx.ap,
        3 => 155f32 + 0.9f32 * ctx.ap,
        4 => 195f32 + 0.9f32 * ctx.ap,
        5 => 235f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xerath_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.65f32 * ctx.ap,
        2 => 85f32 + 0.65f32 * ctx.ap,
        3 => 120f32 + 0.65f32 * ctx.ap,
        4 => 155f32 + 0.65f32 * ctx.ap,
        5 => 190f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xerath_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 83.35f32 + 1.08355f32 * ctx.ap,
        2 => 141.695f32 + 1.08355f32 * ctx.ap,
        3 => 200.04f32 + 1.08355f32 * ctx.ap,
        4 => 258.385f32 + 1.08355f32 * ctx.ap,
        5 => 316.73f32 + 1.08355f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xerath_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.45f32 * ctx.ap,
        2 => 100f32 + 0.45f32 * ctx.ap,
        3 => 130f32 + 0.45f32 * ctx.ap,
        4 => 160f32 + 0.45f32 * ctx.ap,
        5 => 190f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xerath_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 20f32 + 0.05f32 * ctx.ap,
        2 => 25f32 + 0.05f32 * ctx.ap,
        3 => 30f32 + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xerath_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 170f32 + 0.45f32 * ctx.ap,
        2 => 220f32 + 0.45f32 * ctx.ap,
        3 => 270f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xerath_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 680f32 + 1.8f32 * ctx.ap,
        2 => 1100f32 + 2.25f32 * ctx.ap,
        3 => 1620f32 + 2.7f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static XINZHAO: CachedChampion = CachedChampion {
    name: "Xin Zhao",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 274f32,
            per_level: 55f32,
        },
        armor: CachedChampionStatsMap {
            flat: 35f32,
            per_level: 4.4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.645f32,
            per_level: 3.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.644999980926513f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 0.95f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        xinzhao_q_1,
        xinzhao_q_2,
        xinzhao_w_1,
        xinzhao_w_2,
        xinzhao_w_3,
        xinzhao_e_1,
        xinzhao_r_1,
    ],
};
pub const fn xinzhao_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 0.4f32 * ctx.bonus_ad,
        2 => 35f32 + 0.4f32 * ctx.bonus_ad,
        3 => 50f32 + 0.4f32 * ctx.bonus_ad,
        4 => 65f32 + 0.4f32 * ctx.bonus_ad,
        5 => 80f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xinzhao_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 1.2f32 * ctx.bonus_ad,
        2 => 105f32 + 1.2f32 * ctx.bonus_ad,
        3 => 150f32 + 1.2f32 * ctx.bonus_ad,
        4 => 195f32 + 1.2f32 * ctx.bonus_ad,
        5 => 240f32 + 1.2f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn xinzhao_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 30f32 + 0.3f32 * ctx.ad,
        2 => 40f32 + 0.3f32 * ctx.ad,
        3 => 50f32 + 0.3f32 * ctx.ad,
        4 => 60f32 + 0.3f32 * ctx.ad,
        5 => 70f32 + 0.3f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn xinzhao_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 50f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,
        2 => 85f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,
        3 => 120f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,
        4 => 155f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,
        5 => 190f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xinzhao_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 80f32 + 1.2f32 * ctx.ad + 0.65f32 * ctx.ap,
        2 => 125f32 + 1.2f32 * ctx.ad + 0.65f32 * ctx.ap,
        3 => 170f32 + 1.2f32 * ctx.ad + 0.65f32 * ctx.ap,
        4 => 215f32 + 1.2f32 * ctx.ad + 0.65f32 * ctx.ap,
        5 => 260f32 + 1.2f32 * ctx.ad + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xinzhao_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + 0.6f32 * ctx.ap,
        2 => 75f32 + 0.6f32 * ctx.ap,
        3 => 100f32 + 0.6f32 * ctx.ap,
        4 => 125f32 + 0.6f32 * ctx.ap,
        5 => 150f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn xinzhao_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => {
            75f32
                + ctx.bonus_ad
                + 1.1f32 * ctx.ap
                + 0.15f32 * ctx.current_health
        }
        2 => {
            175f32
                + ctx.bonus_ad
                + 1.1f32 * ctx.ap
                + 0.15f32 * ctx.current_health
        }
        3 => {
            275f32
                + ctx.bonus_ad
                + 1.1f32 * ctx.ap
                + 0.15f32 * ctx.current_health
        }
        _ => 0.0,
    }
}
pub static YASUO: CachedChampion = CachedChampion {
    name: "Yasuo",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Bottom, Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_4),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 590f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 100f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.697f32,
            per_level: 3.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.6700000166893f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[
        yasuo_q_1, yasuo_e_1, yasuo_e_2, yasuo_e_3, yasuo_e_4, yasuo_r_1,
    ],
};
pub const fn yasuo_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 1.05f32 * ctx.ad,
        2 => 45f32 + 1.05f32 * ctx.ad,
        3 => 70f32 + 1.05f32 * ctx.ad,
        4 => 95f32 + 1.05f32 * ctx.ad,
        5 => 120f32 + 1.05f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn yasuo_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        2 => 85f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        3 => 100f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        4 => 115f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        5 => 130f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yasuo_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 17.5f32 + 0.05f32 * ctx.bonus_ad + 0.15f32 * ctx.ap,
        2 => 21.25f32 + 0.05f32 * ctx.bonus_ad + 0.15f32 * ctx.ap,
        3 => 25f32 + 0.05f32 * ctx.bonus_ad + 0.15f32 * ctx.ap,
        4 => 28.75f32 + 0.05f32 * ctx.bonus_ad + 0.15f32 * ctx.ap,
        5 => 32.5f32 + 0.05f32 * ctx.bonus_ad + 0.15f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yasuo_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        2 => 85f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        3 => 100f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        4 => 115f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        5 => 130f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yasuo_e_4(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 140f32 + 0.4f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        2 => 170f32 + 0.4f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        3 => 200f32 + 0.4f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        4 => 230f32 + 0.4f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        5 => 260f32 + 0.4f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yasuo_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 1.5f32 * ctx.bonus_ad,
        2 => 350f32 + 1.5f32 * ctx.bonus_ad,
        3 => 500f32 + 1.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static YONE: CachedChampion = CachedChampion {
    name: "Yone",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Middle, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::True,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 620f32,
            per_level: 105f32,
        },
        mana: CachedChampionStatsMap {
            flat: 500f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.6f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 3.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 0.97f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.1f32,
    },
    merge_data: &[],
    closures: &[
        yone_q_1, yone_w_1, yone_w_2, yone_w_3, yone_e_1, yone_r_1, yone_r_2,
        yone_r_3,
    ],
};
pub const fn yone_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 20f32 + 1.05f32 * ctx.ad,
        2 => 45f32 + 1.05f32 * ctx.ad,
        3 => 70f32 + 1.05f32 * ctx.ad,
        4 => 95f32 + 1.05f32 * ctx.ad,
        5 => 120f32 + 1.05f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn yone_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.04f32 * ctx.enemy_max_health,
        2 => 10f32 + 0.045f32 * ctx.enemy_max_health,
        3 => 15f32 + 0.05f32 * ctx.enemy_max_health,
        4 => 20f32 + 0.055f32 * ctx.enemy_max_health,
        5 => 25f32 + 0.06f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn yone_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.04f32 * ctx.enemy_max_health,
        2 => 10f32 + 0.045f32 * ctx.enemy_max_health,
        3 => 15f32 + 0.05f32 * ctx.enemy_max_health,
        4 => 20f32 + 0.055f32 * ctx.enemy_max_health,
        5 => 25f32 + 0.06f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn yone_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 10f32 + 0.08f32 * ctx.enemy_max_health,
        2 => 20f32 + 0.09f32 * ctx.enemy_max_health,
        3 => 30f32 + 0.1f32 * ctx.enemy_max_health,
        4 => 40f32 + 0.11f32 * ctx.enemy_max_health,
        5 => 50f32 + 0.12f32 * ctx.enemy_max_health,
        _ => 0.0,
    }
}
pub const fn yone_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.25f32 * 100.0f32,
        2 => 0.275f32 * 100.0f32,
        3 => 0.3f32 * 100.0f32,
        4 => 0.325f32 * 100.0f32,
        5 => 0.35f32 * 100.0f32,
        _ => 0.0,
    }
}
pub const fn yone_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.4f32 * ctx.bonus_ad,
        2 => 200f32 + 0.4f32 * ctx.bonus_ad,
        3 => 300f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn yone_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 100f32 + 0.4f32 * ctx.bonus_ad,
        2 => 200f32 + 0.4f32 * ctx.bonus_ad,
        3 => 300f32 + 0.4f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn yone_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 200f32 + 0.8f32 * ctx.bonus_ad,
        2 => 400f32 + 0.8f32 * ctx.bonus_ad,
        3 => 600f32 + 0.8f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub static YORICK: CachedChampion = CachedChampion {
    name: "Yorick",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 650f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 300f32,
            per_level: 60f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 4.5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 62f32,
            per_level: 5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[yorick_q_1, yorick_e_1, yorick_e_2, yorick_e_3],
};
pub const fn yorick_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 30f32 + 0.5f32 * ctx.ad,
        2 => 50f32 + 0.5f32 * ctx.ad,
        3 => 70f32 + 0.5f32 * ctx.ad,
        4 => 90f32 + 0.5f32 * ctx.ad,
        5 => 110f32 + 0.5f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn yorick_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 50f32 + ctx.ap,
        2 => 75f32 + ctx.ap,
        3 => 100f32 + ctx.ap,
        4 => 125f32 + ctx.ap,
        5 => 150f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn yorick_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        2 => 0.065f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        3 => 0.07f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        4 => 0.075f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        5 => 0.08f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yorick_e_3(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 70f32 + ctx.ap,
        2 => 105f32 + ctx.ap,
        3 => 140f32 + ctx.ap,
        4 => 175f32 + ctx.ap,
        5 => 210f32 + ctx.ap,
        _ => 0.0,
    }
}
pub static YUNARA: CachedChampion = CachedChampion {
    name: "Yunara",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_4),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_5),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 275f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4.4f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 2.5f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.65f32,
            per_level: 2f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.65f32,
        attack_range: 575f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 0f32,
        urf_damage_taken: 0f32,
        urf_damage_dealt: 0f32,
    },
    merge_data: &[],
    closures: &[
        yunara_q_1, yunara_q_2, yunara_q_3, yunara_q_4, yunara_q_5, yunara_w_1,
        yunara_w_2, yunara_w_3, yunara_r_1,
    ],
};
pub const fn yunara_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.2f32 * ctx.ap,
        2 => 10f32 + 0.2f32 * ctx.ap,
        3 => 15f32 + 0.2f32 * ctx.ap,
        4 => 20f32 + 0.2f32 * ctx.ap,
        5 => 25f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 12.5f32 + 0.5f32 * ctx.ap,
        2 => 25f32 + 0.5f32 * ctx.ap,
        3 => 37.5f32 + 0.5f32 * ctx.ap,
        4 => 50f32 + 0.5f32 * ctx.ap,
        5 => 62.5f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.4f32 * ctx.ap,
        2 => 20f32 + 0.4f32 * ctx.ap,
        3 => 30f32 + 0.4f32 * ctx.ap,
        4 => 40f32 + 0.4f32 * ctx.ap,
        5 => 50f32 + 0.4f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_q_4(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 25f32 + ctx.ap,
        2 => 50f32 + ctx.ap,
        3 => 75f32 + ctx.ap,
        4 => 100f32 + ctx.ap,
        5 => 125f32 + ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_q_5(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 5f32 + 0.2f32 * ctx.ap,
        2 => 10f32 + 0.2f32 * ctx.ap,
        3 => 15f32 + 0.2f32 * ctx.ap,
        4 => 20f32 + 0.2f32 * ctx.ap,
        5 => 25f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,
        2 => 30f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,
        3 => 55f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,
        4 => 80f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,
        5 => 105f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 1.25f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,
        2 => 5f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,
        3 => 8.75f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,
        4 => 12.5f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,
        5 => 16.25f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_w_3(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 5f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        2 => 20f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        3 => 35f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        4 => 50f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        5 => 65f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yunara_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 50f32,
        2 => 200f32,
        3 => 350f32,
        _ => 0.0,
    }
}
pub static YUUMI: CachedChampion = CachedChampion {
    name: "Yuumi",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 500f32,
            per_level: 69f32,
        },
        mana: CachedChampionStatsMap {
            flat: 440f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 25f32,
            per_level: 1.1f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 49f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 1f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 425f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 0.75f32,
    },
    merge_data: &[],
    closures: &[
        yuumi_q_1, yuumi_q_2, yuumi_q_3, yuumi_r_1, yuumi_r_2, yuumi_r_3,
    ],
};
pub const fn yuumi_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.2f32 * ctx.ap,
        2 => 95f32 + 0.2f32 * ctx.ap,
        3 => 130f32 + 0.2f32 * ctx.ap,
        4 => 165f32 + 0.2f32 * ctx.ap,
        5 => 200f32 + 0.2f32 * ctx.ap,
        6 => 235f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yuumi_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.3f32 * ctx.ap,
        2 => 140f32 + 0.3f32 * ctx.ap,
        3 => 200f32 + 0.3f32 * ctx.ap,
        4 => 260f32 + 0.3f32 * ctx.ap,
        5 => 320f32 + 0.3f32 * ctx.ap,
        6 => 380f32 + 0.3f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yuumi_q_3(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 10f32 + 0.05f32 * ctx.ap,
        2 => 12f32 + 0.05f32 * ctx.ap,
        3 => 14f32 + 0.05f32 * ctx.ap,
        4 => 16f32 + 0.05f32 * ctx.ap,
        5 => 18f32 + 0.05f32 * ctx.ap,
        6 => 20f32 + 0.05f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yuumi_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 75f32 + 0.25f32 * ctx.ap,
        2 => 125f32 + 0.25f32 * ctx.ap,
        3 => 175f32 + 0.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yuumi_r_2(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 18.75f32 + 0.0625f32 * ctx.ap,
        2 => 31.25f32 + 0.0625f32 * ctx.ap,
        3 => 43.75f32 + 0.0625f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn yuumi_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 150f32 + 0.5f32 * ctx.ap,
        2 => 250f32 + 0.5f32 * ctx.ap,
        3 => 350f32 + 0.5f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ZAAHEN: CachedChampion = CachedChampion {
    name: "Zaahen",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 640f32,
            per_level: 114f32,
        },
        mana: CachedChampionStatsMap {
            flat: 350f32,
            per_level: 55f32,
        },
        armor: CachedChampionStatsMap {
            flat: 36f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.625f32,
            per_level: 2.5f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 175f32,
        aram_damage_taken: 1f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[zaahen_q_1, zaahen_e_1, zaahen_r_1],
};
pub const fn zaahen_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15f32 + ctx.ad + 0.2f32 * ctx.bonus_ad,
        2 => 30f32 + ctx.ad + 0.3f32 * ctx.bonus_ad,
        3 => 45f32 + ctx.ad + 0.4f32 * ctx.bonus_ad,
        4 => 60f32 + ctx.ad + 0.5f32 * ctx.bonus_ad,
        5 => 75f32 + ctx.ad + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn zaahen_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 40f32 + 0.5f32 * ctx.bonus_ad,
        2 => 60f32 + 0.5f32 * ctx.bonus_ad,
        3 => 80f32 + 0.5f32 * ctx.bonus_ad,
        4 => 100f32 + 0.5f32 * ctx.bonus_ad,
        5 => 120f32 + 0.5f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn zaahen_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 0.1f32,
        2 => 0.2f32,
        3 => 0.3f32,
        _ => 0.0,
    }
}
pub static ZAC: CachedChampion = CachedChampion {
    name: "Zac",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Support, Position::Top],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::W(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 685f32,
            per_level: 109f32,
        },
        mana: CachedChampionStatsMap {
            flat: 0f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 33f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 60f32,
            per_level: 3.4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.736f32,
            per_level: 1.6f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.638000011444091f32,
        attack_range: 175f32,
        aram_damage_taken: 0.96f32,
        aram_damage_dealt: 1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[zac_q_1, zac_q_2, zac_w_1, zac_w_2, zac_e_1, zac_r_1],
};
pub const fn zac_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 40f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,
        2 => 55f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,
        3 => 70f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,
        4 => 85f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,
        5 => 100f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn zac_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,
        2 => 110f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,
        3 => 140f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,
        4 => 170f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,
        5 => 200f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,
        _ => 0.0,
    }
}
pub const fn zac_w_1(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => 240f32,
        2 => 250f32,
        3 => 260f32,
        4 => 270f32,
        5 => 280f32,
        _ => 0.0,
    }
}
pub const fn zac_w_2(ctx: &EvalContext) -> f32 {
    match ctx.w_level {
        1 => {
            40f32 + 0.04f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap
        }
        2 => {
            50f32 + 0.05f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap
        }
        3 => {
            60f32 + 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap
        }
        4 => {
            70f32 + 0.07f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap
        }
        5 => {
            80f32 + 0.08f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap
        }
        _ => 0.0,
    }
}
pub const fn zac_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.0f32,
        2 => 0.0f32,
        3 => 0.0f32,
        4 => 0.0f32,
        5 => 0.0f32,
        _ => 0.0,
    }
}
pub const fn zac_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 70f32 + 0.2f32 * ctx.ap,
        2 => 105f32 + 0.2f32 * ctx.ap,
        3 => 140f32 + 0.2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ZED: CachedChampion = CachedChampion {
    name: "Zed",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Melee,
    positions: &[Position::Jungle, Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_3),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 654f32,
            per_level: 99f32,
        },
        mana: CachedChampionStatsMap {
            flat: 200f32,
            per_level: 0f32,
        },
        armor: CachedChampionStatsMap {
            flat: 32f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 2.05f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 63f32,
            per_level: 3.4f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.651f32,
            per_level: 3.3f32,
        },
        movespeed: 345f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.651000022888183f32,
        attack_range: 125f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 1.1f32,
        urf_damage_dealt: 0.85f32,
    },
    merge_data: &[],
    closures: &[zed_q_1, zed_q_2, zed_r_3],
};
pub const fn zed_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 80f32 + ctx.bonus_ad,
        2 => 120f32 + ctx.bonus_ad,
        3 => 160f32 + ctx.bonus_ad,
        4 => 200f32 + ctx.bonus_ad,
        5 => 240f32 + ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn zed_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 48f32 + 0.6f32 * ctx.bonus_ad,
        2 => 72f32 + 0.6f32 * ctx.bonus_ad,
        3 => 96f32 + 0.6f32 * ctx.bonus_ad,
        4 => 120f32 + 0.6f32 * ctx.bonus_ad,
        5 => 144f32 + 0.6f32 * ctx.bonus_ad,
        _ => 0.0,
    }
}
pub const fn zed_r_3(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => ctx.ad + 0.25f32 * 100.0f32,
        2 => ctx.ad + 0.4f32 * 100.0f32,
        3 => ctx.ad + 0.55f32 * 100.0f32,
        _ => 0.0,
    }
}
pub static ZERI: CachedChampion = CachedChampion {
    name: "Zeri",
    adaptative_type: AdaptativeType::Physical,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_2),
            damage_type: DamageType::Physical,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 600f32,
            per_level: 110f32,
        },
        mana: CachedChampionStatsMap {
            flat: 250f32,
            per_level: 45f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 56f32,
            per_level: 2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2f32,
        },
        movespeed: 330f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 500f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[zeri_q_1, zeri_q_2],
};
pub const fn zeri_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 2.14f32 + 0.14859999999999998f32 * ctx.ad,
        2 => 2.43f32 + 0.1543f32 * ctx.ad,
        3 => 2.71f32 + 0.16f32 * ctx.ad,
        4 => 3f32 + 0.16570000000000001f32 * ctx.ad,
        5 => 3.29f32 + 0.1714f32 * ctx.ad,
        _ => 0.0,
    }
}
pub const fn zeri_q_2(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 15f32 + 1.04f32 * ctx.ad,
        2 => 17f32 + 1.08f32 * ctx.ad,
        3 => 19f32 + 1.12f32 * ctx.ad,
        4 => 21f32 + 1.16f32 * ctx.ad,
        5 => 23f32 + 1.2f32 * ctx.ad,
        _ => 0.0,
    }
}
pub static ZIGGS: CachedChampion = CachedChampion {
    name: "Ziggs",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Bottom, Position::Middle],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_2),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 606f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 480f32,
            per_level: 23.5f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 55f32,
            per_level: 3.1f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.656f32,
            per_level: 2f32,
        },
        movespeed: 325f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.656000018119812f32,
        attack_range: 550f32,
        aram_damage_taken: 1.2f32,
        aram_damage_dealt: 0.87f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[ziggs_e_1, ziggs_e_2],
};
pub const fn ziggs_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 30f32 + 0.25f32 * ctx.ap,
        2 => 70f32 + 0.3f32 * ctx.ap,
        3 => 110f32 + 0.35f32 * ctx.ap,
        4 => 150f32 + 0.4f32 * ctx.ap,
        5 => 190f32 + 0.45f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn ziggs_e_2(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 150f32 + 1.25f32 * ctx.ap,
        2 => 350f32 + 1.5f32 * ctx.ap,
        3 => 550f32 + 1.75f32 * ctx.ap,
        4 => 750f32 + 2f32 * ctx.ap,
        5 => 950f32 + 2.25f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ZILEAN: CachedChampion = CachedChampion {
    name: "Zilean",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::E(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
        TypeMetadata {
            kind: AbilityId::R(AbilityName::_1),
            damage_type: DamageType::Unknown,
            attributes: Attrs::Undefined,
        },
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 574f32,
            per_level: 96f32,
        },
        mana: CachedChampionStatsMap {
            flat: 452f32,
            per_level: 50f32,
        },
        armor: CachedChampionStatsMap {
            flat: 24f32,
            per_level: 5f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 52f32,
            per_level: 3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.13f32,
        },
        movespeed: 335f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.05f32,
        urf_damage_taken: 0.9f32,
        urf_damage_dealt: 1.05f32,
    },
    merge_data: &[],
    closures: &[zilean_q_1, zilean_e_1, zilean_r_1],
};
pub const fn zilean_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 75f32 + 0.9f32 * ctx.ap,
        2 => 115f32 + 0.9f32 * ctx.ap,
        3 => 165f32 + 0.9f32 * ctx.ap,
        4 => 230f32 + 0.9f32 * ctx.ap,
        5 => 300f32 + 0.9f32 * ctx.ap,
        _ => 0.0,
    }
}
pub const fn zilean_e_1(ctx: &EvalContext) -> f32 {
    match ctx.e_level {
        1 => 0.4f32,
        2 => 0.55f32,
        3 => 0.7f32,
        4 => 0.85f32,
        5 => 0.99f32,
        _ => 0.0,
    }
}
pub const fn zilean_r_1(ctx: &EvalContext) -> f32 {
    match ctx.r_level {
        1 => 600f32 + 2f32 * ctx.ap,
        2 => 850f32 + 2f32 * ctx.ap,
        3 => 1100f32 + 2f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static ZOE: CachedChampion = CachedChampion {
    name: "Zoe",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle],
    metadata: &[],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 630f32,
            per_level: 106f32,
        },
        mana: CachedChampionStatsMap {
            flat: 425f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 21f32,
            per_level: 4.7f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 58f32,
            per_level: 3.3f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.658f32,
            per_level: 2.5f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 550f32,
        aram_damage_taken: 0.95f32,
        aram_damage_dealt: 1.1f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[],
};
pub static ZYRA: CachedChampion = CachedChampion {
    name: "Zyra",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Support],
    metadata: &[TypeMetadata {
        kind: AbilityId::Q(AbilityName::_1),
        damage_type: DamageType::Magic,
        attributes: Attrs::Undefined,
    }],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 574f32,
            per_level: 93f32,
        },
        mana: CachedChampionStatsMap {
            flat: 418f32,
            per_level: 25f32,
        },
        armor: CachedChampionStatsMap {
            flat: 29f32,
            per_level: 4.2f32,
        },
        magic_resist: CachedChampionStatsMap {
            flat: 30f32,
            per_level: 1.3f32,
        },
        attack_damage: CachedChampionStatsMap {
            flat: 53f32,
            per_level: 3.2f32,
        },
        attack_speed: CachedChampionStatsMap {
            flat: 0.681f32,
            per_level: 2.11f32,
        },
        movespeed: 340f32,
        critical_strike_damage: 175f32,
        critical_strike_damage_modifier: 1f32,
        attack_speed_ratio: 0.625f32,
        attack_range: 575f32,
        aram_damage_taken: 1.05f32,
        aram_damage_dealt: 0.9f32,
        urf_damage_taken: 1f32,
        urf_damage_dealt: 1f32,
    },
    merge_data: &[],
    closures: &[zyra_q_1],
};
pub const fn zyra_q_1(ctx: &EvalContext) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.65f32 * ctx.ap,
        2 => 100f32 + 0.65f32 * ctx.ap,
        3 => 140f32 + 0.65f32 * ctx.ap,
        4 => 180f32 + 0.65f32 * ctx.ap,
        5 => 220f32 + 0.65f32 * ctx.ap,
        _ => 0.0,
    }
}
pub static CHAMPION_NAME_TO_ID: phf::Map<&str, ChampionId> = phf::phf_map! {
    "آتروكس" => ChampionId::Aatrox,"Aatrox" => ChampionId::Aatrox,"Άατροξ" => ChampionId::Aatrox,"エイトロックス" => ChampionId::Aatrox,"아트록스" => ChampionId::Aatrox,"Атрокс" => ChampionId::Aatrox,"暗裔剑魔" => ChampionId::Aatrox,"厄萨斯" => ChampionId::Aatrox,"厄薩斯" => ChampionId::Aatrox,"آري" => ChampionId::Ahri,"Ahri" => ChampionId::Ahri,"Άρι" => ChampionId::Ahri,"アーリ" => ChampionId::Ahri,"아리" => ChampionId::Ahri,"Ари" => ChampionId::Ahri,"九尾妖狐" => ChampionId::Ahri,"阿璃" => ChampionId::Ahri,"أكالي" => ChampionId::Akali,"Akali" => ChampionId::Akali,"Ακάλι" => ChampionId::Akali,"アカリ" => ChampionId::Akali,"아칼리" => ChampionId::Akali,"Акали" => ChampionId::Akali,"离群之刺" => ChampionId::Akali,"阿卡莉" => ChampionId::Akali,"أكشان" => ChampionId::Akshan,"Akshan" => ChampionId::Akshan,"Ακσάν" => ChampionId::Akshan,"アクシャン" => ChampionId::Akshan,"아크샨" => ChampionId::Akshan,"Акшан" => ChampionId::Akshan,"影哨" => ChampionId::Akshan,"埃可尚" => ChampionId::Akshan,"أليستار" => ChampionId::Alistar,"Alistar" => ChampionId::Alistar,"Άλισταρ" => ChampionId::Alistar,"アリスター" => ChampionId::Alistar,"알리스타" => ChampionId::Alistar,"Алистар" => ChampionId::Alistar,"牛头酋长" => ChampionId::Alistar,"亚历斯塔" => ChampionId::Alistar,"亞歷斯塔" => ChampionId::Alistar,"أمبيسا" => ChampionId::Ambessa,"Ambessa" => ChampionId::Ambessa,"Αμπέσα" => ChampionId::Ambessa,"アンベッサ" => ChampionId::Ambessa,"암베사" => ChampionId::Ambessa,"Амбесса" => ChampionId::Ambessa,"铁血狼母" => ChampionId::Ambessa,"安蓓萨" => ChampionId::Ambessa,"安比薩" => ChampionId::Ambessa,"أمومو" => ChampionId::Amumu,"Amumu" => ChampionId::Amumu,"Αμούμου" => ChampionId::Amumu,"アムム" => ChampionId::Amumu,"아무무" => ChampionId::Amumu,"Амуму" => ChampionId::Amumu,"殇之木乃伊" => ChampionId::Amumu,"阿姆姆" => ChampionId::Amumu,"أنيفيا" => ChampionId::Anivia,"Anivia" => ChampionId::Anivia,"Ανίβια" => ChampionId::Anivia,"アニビア" => ChampionId::Anivia,"애니비아" => ChampionId::Anivia,"Анивия" => ChampionId::Anivia,"冰晶凤凰" => ChampionId::Anivia,"艾妮维亚" => ChampionId::Anivia,"艾妮維亞" => ChampionId::Anivia,"آني" => ChampionId::Annie,"Annie" => ChampionId::Annie,"Άνι" => ChampionId::Annie,"アニー" => ChampionId::Annie,"애니" => ChampionId::Annie,"Энни" => ChampionId::Annie,"黑暗之女" => ChampionId::Annie,"安妮" => ChampionId::Annie,"أفيليوس" => ChampionId::Aphelios,"Aphelios" => ChampionId::Aphelios,"Αφέλιος" => ChampionId::Aphelios,"アフェリオス" => ChampionId::Aphelios,"아펠리오스" => ChampionId::Aphelios,"Афелий" => ChampionId::Aphelios,"残月之肃" => ChampionId::Aphelios,"亚菲利欧" => ChampionId::Aphelios,"亞菲利歐" => ChampionId::Aphelios,"آش" => ChampionId::Ashe,"Ashe" => ChampionId::Ashe,"Ας" => ChampionId::Ashe,"アッシュ" => ChampionId::Ashe,"애쉬" => ChampionId::Ashe,"Эш" => ChampionId::Ashe,"寒冰射手" => ChampionId::Ashe,"艾希" => ChampionId::Ashe,"أوريليون سول" => ChampionId::AurelionSol,"Aurelion Sol" => ChampionId::AurelionSol,"Ωρέλιον Σολ" => ChampionId::AurelionSol,"オレリオン・ソル" => ChampionId::AurelionSol,"아우렐리온 솔" => ChampionId::AurelionSol,"Аурелион Сол" => ChampionId::AurelionSol,"铸星龙王" => ChampionId::AurelionSol,"翱锐龙兽" => ChampionId::AurelionSol,"翱銳龍獸" => ChampionId::AurelionSol,"أورورا" => ChampionId::Aurora,"Aurora" => ChampionId::Aurora,"Ωρόρα" => ChampionId::Aurora,"オーロラ" => ChampionId::Aurora,"오로라" => ChampionId::Aurora,"Аврора" => ChampionId::Aurora,"双界灵兔" => ChampionId::Aurora,"极光" => ChampionId::Aurora,"歐羅拉" => ChampionId::Aurora,"أزير" => ChampionId::Azir,"Azir" => ChampionId::Azir,"Αζίρ" => ChampionId::Azir,"アジール" => ChampionId::Azir,"아지르" => ChampionId::Azir,"Азир" => ChampionId::Azir,"沙漠皇帝" => ChampionId::Azir,"阿祈尔" => ChampionId::Azir,"阿祈爾" => ChampionId::Azir,"بارد" => ChampionId::Bard,"Bard" => ChampionId::Bard,"Βάρδος" => ChampionId::Bard,"Bardo" => ChampionId::Bard,"バード" => ChampionId::Bard,"바드" => ChampionId::Bard,"Бард" => ChampionId::Bard,"星界游神" => ChampionId::Bard,"巴德" => ChampionId::Bard,"بيلفيث" => ChampionId::Belveth,"Bel'Veth" => ChampionId::Belveth,"Μπελ'Βεθ" => ChampionId::Belveth,"ベル＝ヴェス" => ChampionId::Belveth,"벨베스" => ChampionId::Belveth,"Бел'Вет" => ChampionId::Belveth,"虚空女皇" => ChampionId::Belveth,"贝尔薇斯" => ChampionId::Belveth,"貝爾薇斯" => ChampionId::Belveth,"بليتزكرانك" => ChampionId::Blitzcrank,"Blitzcrank" => ChampionId::Blitzcrank,"Μπλίτζκρανκ" => ChampionId::Blitzcrank,"ブリッツクランク" => ChampionId::Blitzcrank,"블리츠크랭크" => ChampionId::Blitzcrank,"Блицкранк" => ChampionId::Blitzcrank,"蒸汽机器人" => ChampionId::Blitzcrank,"布里茨" => ChampionId::Blitzcrank,"براند" => ChampionId::Brand,"Brand" => ChampionId::Brand,"Μπραντ" => ChampionId::Brand,"ブランド" => ChampionId::Brand,"브랜드" => ChampionId::Brand,"Брэнд" => ChampionId::Brand,"复仇焰魂" => ChampionId::Brand,"布兰德" => ChampionId::Brand,"布蘭德" => ChampionId::Brand,"بروم" => ChampionId::Braum,"Braum" => ChampionId::Braum,"Μπράουμ" => ChampionId::Braum,"ブラウム" => ChampionId::Braum,"브라움" => ChampionId::Braum,"Браум" => ChampionId::Braum,"弗雷尔卓德之心" => ChampionId::Braum,"布郎姆" => ChampionId::Braum,"براير" => ChampionId::Briar,"Briar" => ChampionId::Briar,"Μπράιαρ" => ChampionId::Briar,"ブライアー" => ChampionId::Briar,"브라이어" => ChampionId::Briar,"Брайер" => ChampionId::Briar,"狂厄蔷薇" => ChampionId::Briar,"布蕾尔" => ChampionId::Briar,"布蕾爾" => ChampionId::Briar,"كايتلين" => ChampionId::Caitlyn,"Caitlyn" => ChampionId::Caitlyn,"Κέιτλιν" => ChampionId::Caitlyn,"ケイトリン" => ChampionId::Caitlyn,"케이틀린" => ChampionId::Caitlyn,"Кейтлин" => ChampionId::Caitlyn,"皮城女警" => ChampionId::Caitlyn,"凯特琳" => ChampionId::Caitlyn,"凱特琳" => ChampionId::Caitlyn,"كاميل" => ChampionId::Camille,"Camille" => ChampionId::Camille,"Καμίλ" => ChampionId::Camille,"カミール" => ChampionId::Camille,"카밀" => ChampionId::Camille,"Камилла" => ChampionId::Camille,"青钢影" => ChampionId::Camille,"卡蜜儿" => ChampionId::Camille,"卡蜜兒" => ChampionId::Camille,"كاسيوبيا" => ChampionId::Cassiopeia,"Cassiopeia" => ChampionId::Cassiopeia,"Κασσιόπη" => ChampionId::Cassiopeia,"カシオペア" => ChampionId::Cassiopeia,"카시오페아" => ChampionId::Cassiopeia,"Кассиопея" => ChampionId::Cassiopeia,"魔蛇之拥" => ChampionId::Cassiopeia,"卡莎碧雅" => ChampionId::Cassiopeia,"تشوغاث" => ChampionId::Chogath,"Cho'Gath" => ChampionId::Chogath,"Τσο'Γκαθ" => ChampionId::Chogath,"チョ＝ガス" => ChampionId::Chogath,"초가스" => ChampionId::Chogath,"Чо'Гат" => ChampionId::Chogath,"虚空恐惧" => ChampionId::Chogath,"科加斯" => ChampionId::Chogath,"كوركي" => ChampionId::Corki,"Corki" => ChampionId::Corki,"Κόρκι" => ChampionId::Corki,"コーキ" => ChampionId::Corki,"코르키" => ChampionId::Corki,"Корки" => ChampionId::Corki,"英勇投弹手" => ChampionId::Corki,"库奇" => ChampionId::Corki,"庫奇" => ChampionId::Corki,"داريوس" => ChampionId::Darius,"Darius" => ChampionId::Darius,"Ντάριους" => ChampionId::Darius,"ダリウス" => ChampionId::Darius,"다리우스" => ChampionId::Darius,"Дариус" => ChampionId::Darius,"诺克萨斯之手" => ChampionId::Darius,"达瑞斯" => ChampionId::Darius,"達瑞斯" => ChampionId::Darius,"ديانا" => ChampionId::Diana,"Diana" => ChampionId::Diana,"Ντιάνα" => ChampionId::Diana,"ダイアナ" => ChampionId::Diana,"다이애나" => ChampionId::Diana,"Диана" => ChampionId::Diana,"皎月女神" => ChampionId::Diana,"黛安娜" => ChampionId::Diana,"د. موندو" => ChampionId::DrMundo,"Dr. Mundo" => ChampionId::DrMundo,"Δρ. Μούντο" => ChampionId::DrMundo,"ドクター・ムンド" => ChampionId::DrMundo,"문도 박사" => ChampionId::DrMundo,"Dr Mundo" => ChampionId::DrMundo,"Доктор Мундо" => ChampionId::DrMundo,"祖安狂人" => ChampionId::DrMundo,"蒙多医生" => ChampionId::DrMundo,"蒙多醫生" => ChampionId::DrMundo,"درايفن" => ChampionId::Draven,"Draven" => ChampionId::Draven,"Ντρέιβεν" => ChampionId::Draven,"ドレイヴン" => ChampionId::Draven,"드레이븐" => ChampionId::Draven,"Дрейвен" => ChampionId::Draven,"荣耀行刑官" => ChampionId::Draven,"达瑞文" => ChampionId::Draven,"達瑞文" => ChampionId::Draven,"إيكو" => ChampionId::Ekko,"Ekko" => ChampionId::Ekko,"Έκκο" => ChampionId::Ekko,"エコー" => ChampionId::Ekko,"에코" => ChampionId::Ekko,"Экко" => ChampionId::Ekko,"时间刺客" => ChampionId::Ekko,"艾克" => ChampionId::Ekko,"إليز" => ChampionId::Elise,"Elise" => ChampionId::Elise,"Ελίζ" => ChampionId::Elise,"エリス" => ChampionId::Elise,"엘리스" => ChampionId::Elise,"Элиза" => ChampionId::Elise,"蜘蛛女皇" => ChampionId::Elise,"伊莉丝" => ChampionId::Elise,"伊莉絲" => ChampionId::Elise,"إيفلين" => ChampionId::Evelynn,"Evelynn" => ChampionId::Evelynn,"Έβελιν" => ChampionId::Evelynn,"イブリン" => ChampionId::Evelynn,"이블린" => ChampionId::Evelynn,"Эвелинн" => ChampionId::Evelynn,"痛苦之拥" => ChampionId::Evelynn,"伊芙琳" => ChampionId::Evelynn,"إزريال" => ChampionId::Ezreal,"Ezreal" => ChampionId::Ezreal,"Έζρεαλ" => ChampionId::Ezreal,"エズリアル" => ChampionId::Ezreal,"이즈리얼" => ChampionId::Ezreal,"Эзреаль" => ChampionId::Ezreal,"探险家" => ChampionId::Ezreal,"伊泽瑞尔" => ChampionId::Ezreal,"伊澤瑞爾" => ChampionId::Ezreal,"فيدل ستيكس" => ChampionId::Fiddlesticks,"Fiddlesticks" => ChampionId::Fiddlesticks,"Φίντλστιξ" => ChampionId::Fiddlesticks,"フィドルスティックス" => ChampionId::Fiddlesticks,"피들스틱" => ChampionId::Fiddlesticks,"Фиддлстикс" => ChampionId::Fiddlesticks,"远古恐惧" => ChampionId::Fiddlesticks,"费德提克" => ChampionId::Fiddlesticks,"費德提克" => ChampionId::Fiddlesticks,"فيورا" => ChampionId::Fiora,"Fiora" => ChampionId::Fiora,"Φιόρα" => ChampionId::Fiora,"フィオラ" => ChampionId::Fiora,"피오라" => ChampionId::Fiora,"Фиора" => ChampionId::Fiora,"无双剑姬" => ChampionId::Fiora,"菲欧拉" => ChampionId::Fiora,"菲歐拉" => ChampionId::Fiora,"فيز" => ChampionId::Fizz,"Fizz" => ChampionId::Fizz,"Φιζ" => ChampionId::Fizz,"フィズ" => ChampionId::Fizz,"피즈" => ChampionId::Fizz,"Физз" => ChampionId::Fizz,"潮汐海灵" => ChampionId::Fizz,"飞斯" => ChampionId::Fizz,"飛斯" => ChampionId::Fizz,"غاليو" => ChampionId::Galio,"Galio" => ChampionId::Galio,"Γκάλιο" => ChampionId::Galio,"ガリオ" => ChampionId::Galio,"갈리오" => ChampionId::Galio,"Галио" => ChampionId::Galio,"正义巨像" => ChampionId::Galio,"加里欧" => ChampionId::Galio,"加里歐" => ChampionId::Galio,"غانغ بلانك" => ChampionId::Gangplank,"Gangplank" => ChampionId::Gangplank,"Γκάνγκπλανκ" => ChampionId::Gangplank,"ガングプランク" => ChampionId::Gangplank,"갱플랭크" => ChampionId::Gangplank,"Гангпланк" => ChampionId::Gangplank,"海洋之灾" => ChampionId::Gangplank,"刚普朗克" => ChampionId::Gangplank,"剛普朗克" => ChampionId::Gangplank,"غارين" => ChampionId::Garen,"Garen" => ChampionId::Garen,"Γκάρεν" => ChampionId::Garen,"ガレン" => ChampionId::Garen,"가렌" => ChampionId::Garen,"Гарен" => ChampionId::Garen,"德玛西亚之力" => ChampionId::Garen,"盖伦" => ChampionId::Garen,"蓋倫" => ChampionId::Garen,"غنار" => ChampionId::Gnar,"Gnar" => ChampionId::Gnar,"Γκναρ" => ChampionId::Gnar,"ナー" => ChampionId::Gnar,"나르" => ChampionId::Gnar,"Гнар" => ChampionId::Gnar,"迷失之牙" => ChampionId::Gnar,"呐儿" => ChampionId::Gnar,"吶兒" => ChampionId::Gnar,"غراغاس" => ChampionId::Gragas,"Gragas" => ChampionId::Gragas,"Γκράγκας" => ChampionId::Gragas,"グラガス" => ChampionId::Gragas,"그라가스" => ChampionId::Gragas,"Грагас" => ChampionId::Gragas,"酒桶" => ChampionId::Gragas,"古拉格斯" => ChampionId::Gragas,"غرايفز" => ChampionId::Graves,"Graves" => ChampionId::Graves,"Γκρέιβς" => ChampionId::Graves,"グレイブス" => ChampionId::Graves,"그레이브즈" => ChampionId::Graves,"Грейвз" => ChampionId::Graves,"法外狂徒" => ChampionId::Graves,"葛雷夫" => ChampionId::Graves,"غوين" => ChampionId::Gwen,"Gwen" => ChampionId::Gwen,"Γκουέν" => ChampionId::Gwen,"グウェン" => ChampionId::Gwen,"그웬" => ChampionId::Gwen,"Гвен" => ChampionId::Gwen,"灵罗娃娃" => ChampionId::Gwen,"关" => ChampionId::Gwen,"關" => ChampionId::Gwen,"هيكاريم" => ChampionId::Hecarim,"Hecarim" => ChampionId::Hecarim,"Χέκαριμ" => ChampionId::Hecarim,"ヘカリム" => ChampionId::Hecarim,"헤카림" => ChampionId::Hecarim,"Гекарим" => ChampionId::Hecarim,"战争之影" => ChampionId::Hecarim,"赫克林" => ChampionId::Hecarim,"هايمردينغر" => ChampionId::Heimerdinger,"Heimerdinger" => ChampionId::Heimerdinger,"Χάιμερντιγκερ" => ChampionId::Heimerdinger,"ハイマーディンガー" => ChampionId::Heimerdinger,"하이머딩거" => ChampionId::Heimerdinger,"Хеймердингер" => ChampionId::Heimerdinger,"大发明家" => ChampionId::Heimerdinger,"汉默丁格" => ChampionId::Heimerdinger,"漢默丁格" => ChampionId::Heimerdinger,"هواي" => ChampionId::Hwei,"Hwei" => ChampionId::Hwei,"Χουέι" => ChampionId::Hwei,"フェイ" => ChampionId::Hwei,"흐웨이" => ChampionId::Hwei,"Хвэй" => ChampionId::Hwei,"异画师" => ChampionId::Hwei,"慧" => ChampionId::Hwei,"赫威" => ChampionId::Hwei,"إيلاوي" => ChampionId::Illaoi,"Illaoi" => ChampionId::Illaoi,"Ιλλαόη" => ChampionId::Illaoi,"イラオイ" => ChampionId::Illaoi,"일라오이" => ChampionId::Illaoi,"Иллаой" => ChampionId::Illaoi,"海兽祭司" => ChampionId::Illaoi,"伊罗旖" => ChampionId::Illaoi,"伊羅旖" => ChampionId::Illaoi,"إيريليا" => ChampionId::Irelia,"Irelia" => ChampionId::Irelia,"Ιρέλια" => ChampionId::Irelia,"イレリア" => ChampionId::Irelia,"이렐리아" => ChampionId::Irelia,"Ирелия" => ChampionId::Irelia,"刀锋舞者" => ChampionId::Irelia,"伊瑞莉雅" => ChampionId::Irelia,"آيفرن" => ChampionId::Ivern,"Ivern" => ChampionId::Ivern,"Άιβερν" => ChampionId::Ivern,"アイバーン" => ChampionId::Ivern,"아이번" => ChampionId::Ivern,"Иверн" => ChampionId::Ivern,"翠神" => ChampionId::Ivern,"埃尔文" => ChampionId::Ivern,"埃爾文" => ChampionId::Ivern,"جانا" => ChampionId::Janna,"Janna" => ChampionId::Janna,"Τζάνα" => ChampionId::Janna,"ジャンナ" => ChampionId::Janna,"잔나" => ChampionId::Janna,"Жанна" => ChampionId::Janna,"风暴之怒" => ChampionId::Janna,"珍娜" => ChampionId::Janna,"جارفان الرابع" => ChampionId::JarvanIV,"Jarvan IV" => ChampionId::JarvanIV,"Jarvan IV." => ChampionId::JarvanIV,"Τζάρβαν ο Δ'" => ChampionId::JarvanIV,"IV. Jarvan" => ChampionId::JarvanIV,"ジャーヴァンⅣ" => ChampionId::JarvanIV,"자르반 4세" => ChampionId::JarvanIV,"Джарван IV" => ChampionId::JarvanIV,"德玛西亚皇子" => ChampionId::JarvanIV,"嘉文四世" => ChampionId::JarvanIV,"جاكس" => ChampionId::Jax,"Jax" => ChampionId::Jax,"Τζαξ" => ChampionId::Jax,"ジャックス" => ChampionId::Jax,"잭스" => ChampionId::Jax,"Джакс" => ChampionId::Jax,"武器大师" => ChampionId::Jax,"贾克斯" => ChampionId::Jax,"賈克斯" => ChampionId::Jax,"جايس" => ChampionId::Jayce,"Jayce" => ChampionId::Jayce,"Τζέις" => ChampionId::Jayce,"ジェイス" => ChampionId::Jayce,"제이스" => ChampionId::Jayce,"Джейс" => ChampionId::Jayce,"未来守护者" => ChampionId::Jayce,"杰西" => ChampionId::Jayce,"جين" => ChampionId::Jhin,"Jhin" => ChampionId::Jhin,"Τζιν" => ChampionId::Jhin,"ジン" => ChampionId::Jhin,"진" => ChampionId::Jhin,"Джин" => ChampionId::Jhin,"戏命师" => ChampionId::Jhin,"烬" => ChampionId::Jhin,"燼" => ChampionId::Jhin,"جينكس" => ChampionId::Jinx,"Jinx" => ChampionId::Jinx,"Τζινξ" => ChampionId::Jinx,"ジンクス" => ChampionId::Jinx,"징크스" => ChampionId::Jinx,"Джинкс" => ChampionId::Jinx,"暴走萝莉" => ChampionId::Jinx,"吉茵珂丝" => ChampionId::Jinx,"吉茵珂絲" => ChampionId::Jinx,"كاسانتي" => ChampionId::KSante,"K'Sante" => ChampionId::KSante,"Κα'Σάντι" => ChampionId::KSante,"K'Santé" => ChampionId::KSante,"カ・サンテ" => ChampionId::KSante,"크산테" => ChampionId::KSante,"К'Санте" => ChampionId::KSante,"纳祖芒荣耀" => ChampionId::KSante,"卡桑帝" => ChampionId::KSante,"كايسا" => ChampionId::Kaisa,"Kai'Sa" => ChampionId::Kaisa,"Κάι'Σα" => ChampionId::Kaisa,"カイ＝サ" => ChampionId::Kaisa,"카이사" => ChampionId::Kaisa,"Кай'Са" => ChampionId::Kaisa,"虚空之女" => ChampionId::Kaisa,"凯莎" => ChampionId::Kaisa,"凱莎" => ChampionId::Kaisa,"كاليستا" => ChampionId::Kalista,"Kalista" => ChampionId::Kalista,"Καλίστα" => ChampionId::Kalista,"カリスタ" => ChampionId::Kalista,"칼리스타" => ChampionId::Kalista,"Калиста" => ChampionId::Kalista,"复仇之矛" => ChampionId::Kalista,"克黎思妲" => ChampionId::Kalista,"كارما" => ChampionId::Karma,"Karma" => ChampionId::Karma,"Κάρμα" => ChampionId::Karma,"カルマ" => ChampionId::Karma,"카르마" => ChampionId::Karma,"Карма" => ChampionId::Karma,"天启者" => ChampionId::Karma,"卡玛" => ChampionId::Karma,"卡瑪" => ChampionId::Karma,"كارثوس" => ChampionId::Karthus,"Karthus" => ChampionId::Karthus,"Κάρθους" => ChampionId::Karthus,"カーサス" => ChampionId::Karthus,"카서스" => ChampionId::Karthus,"Картус" => ChampionId::Karthus,"死亡颂唱者" => ChampionId::Karthus,"卡尔瑟斯" => ChampionId::Karthus,"卡爾瑟斯" => ChampionId::Karthus,"كاسادين" => ChampionId::Kassadin,"Kassadin" => ChampionId::Kassadin,"Κάσαντιν" => ChampionId::Kassadin,"カサディン" => ChampionId::Kassadin,"카사딘" => ChampionId::Kassadin,"Кассадин" => ChampionId::Kassadin,"虚空行者" => ChampionId::Kassadin,"卡萨丁" => ChampionId::Kassadin,"卡薩丁" => ChampionId::Kassadin,"كاتارينا" => ChampionId::Katarina,"Katarina" => ChampionId::Katarina,"Καταρίνα" => ChampionId::Katarina,"カタリナ" => ChampionId::Katarina,"카타리나" => ChampionId::Katarina,"Катарина" => ChampionId::Katarina,"不祥之刃" => ChampionId::Katarina,"卡特莲娜" => ChampionId::Katarina,"卡特蓮娜" => ChampionId::Katarina,"كايل" => ChampionId::Kayle,"Kayle" => ChampionId::Kayle,"Κέιλ" => ChampionId::Kayle,"ケイル" => ChampionId::Kayle,"케일" => ChampionId::Kayle,"Кейл" => ChampionId::Kayle,"正义天使" => ChampionId::Kayle,"凯尔" => ChampionId::Kayle,"凱爾" => ChampionId::Kayle,"كاين" => ChampionId::Kayn,"Kayn" => ChampionId::Kayn,"Κέιν" => ChampionId::Kayn,"ケイン" => ChampionId::Kayn,"케인" => ChampionId::Kayn,"Каин" => ChampionId::Kayn,"影流之镰" => ChampionId::Kayn,"慨影" => ChampionId::Kayn,"كينين" => ChampionId::Kennen,"Kennen" => ChampionId::Kennen,"Κένεν" => ChampionId::Kennen,"ケネン" => ChampionId::Kennen,"케넨" => ChampionId::Kennen,"Кеннен" => ChampionId::Kennen,"狂暴之心" => ChampionId::Kennen,"凯能" => ChampionId::Kennen,"凱能" => ChampionId::Kennen,"كازيكس" => ChampionId::Khazix,"Kha'Zix" => ChampionId::Khazix,"Κα'Ζιξ" => ChampionId::Khazix,"カ＝ジックス" => ChampionId::Khazix,"카직스" => ChampionId::Khazix,"Ка'Зикс" => ChampionId::Khazix,"虚空掠夺者" => ChampionId::Khazix,"卡力斯" => ChampionId::Khazix,"كيندريد" => ChampionId::Kindred,"Kindred" => ChampionId::Kindred,"Κίντρεντ" => ChampionId::Kindred,"キンドレッド" => ChampionId::Kindred,"킨드레드" => ChampionId::Kindred,"Киндред" => ChampionId::Kindred,"永猎双子" => ChampionId::Kindred,"镜爪" => ChampionId::Kindred,"鏡爪" => ChampionId::Kindred,"كليد" => ChampionId::Kled,"Kled" => ChampionId::Kled,"Κλεντ" => ChampionId::Kled,"クレッド" => ChampionId::Kled,"클레드" => ChampionId::Kled,"Клед" => ChampionId::Kled,"暴怒骑士" => ChampionId::Kled,"克雷德" => ChampionId::Kled,"كوغ ماو" => ChampionId::KogMaw,"Kog'Maw" => ChampionId::KogMaw,"Κογκ'Μο" => ChampionId::KogMaw,"コグ＝マウ" => ChampionId::KogMaw,"코그모" => ChampionId::KogMaw,"Ког'Мао" => ChampionId::KogMaw,"深渊巨口" => ChampionId::KogMaw,"寇格魔" => ChampionId::KogMaw,"لوبلانك" => ChampionId::Leblanc,"LeBlanc" => ChampionId::Leblanc,"ΛεΜπλάν" => ChampionId::Leblanc,"ルブラン" => ChampionId::Leblanc,"르블랑" => ChampionId::Leblanc,"Ле Блан" => ChampionId::Leblanc,"诡术妖姬" => ChampionId::Leblanc,"勒布朗" => ChampionId::Leblanc,"لي سين" => ChampionId::LeeSin,"Lee Sin" => ChampionId::LeeSin,"Λι Σιν" => ChampionId::LeeSin,"リー・シン" => ChampionId::LeeSin,"리 신" => ChampionId::LeeSin,"Ли Син" => ChampionId::LeeSin,"盲僧" => ChampionId::LeeSin,"李星" => ChampionId::LeeSin,"ليونا" => ChampionId::Leona,"Leona" => ChampionId::Leona,"Λεόνα" => ChampionId::Leona,"レオナ" => ChampionId::Leona,"레오나" => ChampionId::Leona,"Леона" => ChampionId::Leona,"曙光女神" => ChampionId::Leona,"雷欧娜" => ChampionId::Leona,"雷歐娜" => ChampionId::Leona,"ليليا" => ChampionId::Lillia,"Lillia" => ChampionId::Lillia,"Λίλια" => ChampionId::Lillia,"リリア" => ChampionId::Lillia,"릴리아" => ChampionId::Lillia,"Лиллия" => ChampionId::Lillia,"含羞蓓蕾" => ChampionId::Lillia,"莉莉亚" => ChampionId::Lillia,"莉莉亞" => ChampionId::Lillia,"ليساندرا" => ChampionId::Lissandra,"Lissandra" => ChampionId::Lissandra,"Λισάντρα" => ChampionId::Lissandra,"リサンドラ" => ChampionId::Lissandra,"리산드라" => ChampionId::Lissandra,"Лиссандра" => ChampionId::Lissandra,"冰霜女巫" => ChampionId::Lissandra,"丽珊卓" => ChampionId::Lissandra,"麗珊卓" => ChampionId::Lissandra,"لوشيان" => ChampionId::Lucian,"Lucian" => ChampionId::Lucian,"Λούσιαν" => ChampionId::Lucian,"ルシアン" => ChampionId::Lucian,"루시안" => ChampionId::Lucian,"Люциан" => ChampionId::Lucian,"圣枪游侠" => ChampionId::Lucian,"路西恩" => ChampionId::Lucian,"لولو" => ChampionId::Lulu,"Lulu" => ChampionId::Lulu,"Λούλου" => ChampionId::Lulu,"ルル" => ChampionId::Lulu,"룰루" => ChampionId::Lulu,"Лулу" => ChampionId::Lulu,"仙灵女巫" => ChampionId::Lulu,"露璐" => ChampionId::Lulu,"لكس" => ChampionId::Lux,"Lux" => ChampionId::Lux,"Λουξ" => ChampionId::Lux,"ラックス" => ChampionId::Lux,"럭스" => ChampionId::Lux,"Люкс" => ChampionId::Lux,"光辉女郎" => ChampionId::Lux,"拉克丝" => ChampionId::Lux,"拉克絲" => ChampionId::Lux,"مالفايت" => ChampionId::Malphite,"Malphite" => ChampionId::Malphite,"Μάλφαϊτ" => ChampionId::Malphite,"マルファイト" => ChampionId::Malphite,"말파이트" => ChampionId::Malphite,"Мальфит" => ChampionId::Malphite,"熔岩巨兽" => ChampionId::Malphite,"墨菲特" => ChampionId::Malphite,"مالزاهار" => ChampionId::Malzahar,"Malzahar" => ChampionId::Malzahar,"Μάλζαχαρ" => ChampionId::Malzahar,"マルザハール" => ChampionId::Malzahar,"말자하" => ChampionId::Malzahar,"Мальзахар" => ChampionId::Malzahar,"虚空先知" => ChampionId::Malzahar,"马尔札哈" => ChampionId::Malzahar,"馬爾札哈" => ChampionId::Malzahar,"ماوكاي" => ChampionId::Maokai,"Maokai" => ChampionId::Maokai,"Μαοκάι" => ChampionId::Maokai,"マオカイ" => ChampionId::Maokai,"마오카이" => ChampionId::Maokai,"Маокай" => ChampionId::Maokai,"扭曲树精" => ChampionId::Maokai,"茂凯" => ChampionId::Maokai,"茂凱" => ChampionId::Maokai,"ماستر يي" => ChampionId::MasterYi,"Master Yi" => ChampionId::MasterYi,"Mistr Yi" => ChampionId::MasterYi,"Μάστερ Γι" => ChampionId::MasterYi,"Maestro Yi" => ChampionId::MasterYi,"Maître Yi" => ChampionId::MasterYi,"マスター・イー" => ChampionId::MasterYi,"마스터 이" => ChampionId::MasterYi,"Мастер Йи" => ChampionId::MasterYi,"无极剑圣" => ChampionId::MasterYi,"易大师" => ChampionId::MasterYi,"易大師" => ChampionId::MasterYi,"ميل" => ChampionId::Mel,"Mel" => ChampionId::Mel,"Μελ" => ChampionId::Mel,"メル" => ChampionId::Mel,"멜" => ChampionId::Mel,"Мэл" => ChampionId::Mel,"流光镜影" => ChampionId::Mel,"梅尔" => ChampionId::Mel,"梅爾" => ChampionId::Mel,"ميليو" => ChampionId::Milio,"Milio" => ChampionId::Milio,"Μίλιο" => ChampionId::Milio,"ミリオ" => ChampionId::Milio,"밀리오" => ChampionId::Milio,"Милио" => ChampionId::Milio,"明烛" => ChampionId::Milio,"米里欧" => ChampionId::Milio,"米里歐" => ChampionId::Milio,"ميس فورتشن" => ChampionId::MissFortune,"Miss Fortune" => ChampionId::MissFortune,"Μις Φόρτσουν" => ChampionId::MissFortune,"ミス・フォーチュン" => ChampionId::MissFortune,"미스 포츈" => ChampionId::MissFortune,"Мисс Фортуна" => ChampionId::MissFortune,"赏金猎人" => ChampionId::MissFortune,"好运姐" => ChampionId::MissFortune,"好運姐" => ChampionId::MissFortune,"ووكونغ" => ChampionId::MonkeyKing,"Wukong" => ChampionId::MonkeyKing,"Γουκόνγκ" => ChampionId::MonkeyKing,"ウーコン" => ChampionId::MonkeyKing,"오공" => ChampionId::MonkeyKing,"Вуконг" => ChampionId::MonkeyKing,"Ngộ Không" => ChampionId::MonkeyKing,"齐天大圣" => ChampionId::MonkeyKing,"悟空" => ChampionId::MonkeyKing,"مورديكايزر" => ChampionId::Mordekaiser,"Mordekaiser" => ChampionId::Mordekaiser,"Μορντεκάιζερ" => ChampionId::Mordekaiser,"モルデカイザー" => ChampionId::Mordekaiser,"모데카이저" => ChampionId::Mordekaiser,"Мордекайзер" => ChampionId::Mordekaiser,"铁铠冥魂" => ChampionId::Mordekaiser,"魔斗凯萨" => ChampionId::Mordekaiser,"魔鬥凱薩" => ChampionId::Mordekaiser,"مورغانا" => ChampionId::Morgana,"Morgana" => ChampionId::Morgana,"Μοργκάνα" => ChampionId::Morgana,"モルガナ" => ChampionId::Morgana,"모르가나" => ChampionId::Morgana,"Моргана" => ChampionId::Morgana,"堕落天使" => ChampionId::Morgana,"魔甘娜" => ChampionId::Morgana,"نافيري" => ChampionId::Naafiri,"Naafiri" => ChampionId::Naafiri,"Νααφίρι" => ChampionId::Naafiri,"ナフィーリ" => ChampionId::Naafiri,"나피리" => ChampionId::Naafiri,"Наафири" => ChampionId::Naafiri,"百裂冥犬" => ChampionId::Naafiri,"纳菲利" => ChampionId::Naafiri,"娜菲芮" => ChampionId::Naafiri,"نامي" => ChampionId::Nami,"Nami" => ChampionId::Nami,"Νάμι" => ChampionId::Nami,"ナミ" => ChampionId::Nami,"나미" => ChampionId::Nami,"Нами" => ChampionId::Nami,"唤潮鲛姬" => ChampionId::Nami,"娜米" => ChampionId::Nami,"ناسوس" => ChampionId::Nasus,"Nasus" => ChampionId::Nasus,"Νάσους" => ChampionId::Nasus,"ナサス" => ChampionId::Nasus,"나서스" => ChampionId::Nasus,"Насус" => ChampionId::Nasus,"沙漠死神" => ChampionId::Nasus,"纳瑟斯" => ChampionId::Nasus,"納瑟斯" => ChampionId::Nasus,"نوتيلوس" => ChampionId::Nautilus,"Nautilus" => ChampionId::Nautilus,"Νότιλους" => ChampionId::Nautilus,"ノーチラス" => ChampionId::Nautilus,"노틸러스" => ChampionId::Nautilus,"Наутилус" => ChampionId::Nautilus,"深海泰坦" => ChampionId::Nautilus,"纳帝鲁斯" => ChampionId::Nautilus,"納帝魯斯" => ChampionId::Nautilus,"ن\u{650}كو" => ChampionId::Neeko,"Neeko" => ChampionId::Neeko,"Νίκκο" => ChampionId::Neeko,"ニーコ" => ChampionId::Neeko,"니코" => ChampionId::Neeko,"Нико" => ChampionId::Neeko,"万花通灵" => ChampionId::Neeko,"妮可" => ChampionId::Neeko,"نيدالي" => ChampionId::Nidalee,"Nidalee" => ChampionId::Nidalee,"Νίνταλι" => ChampionId::Nidalee,"ニダリー" => ChampionId::Nidalee,"니달리" => ChampionId::Nidalee,"Нидали" => ChampionId::Nidalee,"狂野女猎手" => ChampionId::Nidalee,"奈德丽" => ChampionId::Nidalee,"奈德麗" => ChampionId::Nidalee,"نيلا" => ChampionId::Nilah,"Nilah" => ChampionId::Nilah,"Νάιλα" => ChampionId::Nilah,"ニーラ" => ChampionId::Nilah,"닐라" => ChampionId::Nilah,"Нила" => ChampionId::Nilah,"不羁之悦" => ChampionId::Nilah,"淣菈" => ChampionId::Nilah,"نوكتورن" => ChampionId::Nocturne,"Nocturne" => ChampionId::Nocturne,"Νόκτουρν" => ChampionId::Nocturne,"ノクターン" => ChampionId::Nocturne,"녹턴" => ChampionId::Nocturne,"Ноктюрн" => ChampionId::Nocturne,"永恒梦魇" => ChampionId::Nocturne,"夜曲" => ChampionId::Nocturne,"نونو وويلامب" => ChampionId::Nunu,"Nunu & Willump" => ChampionId::Nunu,"Nunu a Willump" => ChampionId::Nunu,"Νούνου και Γουίλαμπ" => ChampionId::Nunu,"Nunu y Willump" => ChampionId::Nunu,"Nunu et Willump" => ChampionId::Nunu,"Nunu és Willump" => ChampionId::Nunu,"Nunu e Willump" => ChampionId::Nunu,"ヌヌ＆ウィルンプ" => ChampionId::Nunu,"누누와 윌럼프" => ChampionId::Nunu,"Nunu i Willump" => ChampionId::Nunu,"Nunu și Willump" => ChampionId::Nunu,"Нуну и Виллумп" => ChampionId::Nunu,"Nunu ve Willump" => ChampionId::Nunu,"雪原双子" => ChampionId::Nunu,"努努和威朗普" => ChampionId::Nunu,"أولاف" => ChampionId::Olaf,"Olaf" => ChampionId::Olaf,"Όλαφ" => ChampionId::Olaf,"オラフ" => ChampionId::Olaf,"올라프" => ChampionId::Olaf,"Олаф" => ChampionId::Olaf,"狂战士" => ChampionId::Olaf,"欧拉夫" => ChampionId::Olaf,"歐拉夫" => ChampionId::Olaf,"أوريانا" => ChampionId::Orianna,"Orianna" => ChampionId::Orianna,"Οριάνα" => ChampionId::Orianna,"オリアナ" => ChampionId::Orianna,"오리아나" => ChampionId::Orianna,"Орианна" => ChampionId::Orianna,"发条魔灵" => ChampionId::Orianna,"奥莉安娜" => ChampionId::Orianna,"奧莉安娜" => ChampionId::Orianna,"أورن" => ChampionId::Ornn,"Ornn" => ChampionId::Ornn,"Ορν" => ChampionId::Ornn,"オーン" => ChampionId::Ornn,"오른" => ChampionId::Ornn,"Орн" => ChampionId::Ornn,"山隐之焰" => ChampionId::Ornn,"鄂尔" => ChampionId::Ornn,"鄂爾" => ChampionId::Ornn,"بانثيون" => ChampionId::Pantheon,"Pantheon" => ChampionId::Pantheon,"Πάνθεον" => ChampionId::Pantheon,"パンテオン" => ChampionId::Pantheon,"판테온" => ChampionId::Pantheon,"Пантеон" => ChampionId::Pantheon,"不屈之枪" => ChampionId::Pantheon,"潘森" => ChampionId::Pantheon,"بوبي" => ChampionId::Poppy,"Poppy" => ChampionId::Poppy,"Πόπι" => ChampionId::Poppy,"ポッピー" => ChampionId::Poppy,"뽀삐" => ChampionId::Poppy,"Поппи" => ChampionId::Poppy,"圣锤之毅" => ChampionId::Poppy,"波比" => ChampionId::Poppy,"بايك" => ChampionId::Pyke,"Pyke" => ChampionId::Pyke,"Πάικ" => ChampionId::Pyke,"パイク" => ChampionId::Pyke,"파이크" => ChampionId::Pyke,"Пайк" => ChampionId::Pyke,"血港鬼影" => ChampionId::Pyke,"派克" => ChampionId::Pyke,"كيانا" => ChampionId::Qiyana,"Qiyana" => ChampionId::Qiyana,"Κιάνα" => ChampionId::Qiyana,"キヤナ" => ChampionId::Qiyana,"키아나" => ChampionId::Qiyana,"Киана" => ChampionId::Qiyana,"元素女皇" => ChampionId::Qiyana,"姬亚娜" => ChampionId::Qiyana,"姬亞娜" => ChampionId::Qiyana,"كوين" => ChampionId::Quinn,"Quinn" => ChampionId::Quinn,"Κουίν" => ChampionId::Quinn,"クイン" => ChampionId::Quinn,"퀸" => ChampionId::Quinn,"Квинн" => ChampionId::Quinn,"德玛西亚之翼" => ChampionId::Quinn,"葵恩" => ChampionId::Quinn,"راكان" => ChampionId::Rakan,"Rakan" => ChampionId::Rakan,"Ρακάν" => ChampionId::Rakan,"ラカン" => ChampionId::Rakan,"라칸" => ChampionId::Rakan,"Рэйкан" => ChampionId::Rakan,"幻翎" => ChampionId::Rakan,"锐空" => ChampionId::Rakan,"銳空" => ChampionId::Rakan,"راموس" => ChampionId::Rammus,"Rammus" => ChampionId::Rammus,"Ράμους" => ChampionId::Rammus,"ラムス" => ChampionId::Rammus,"람머스" => ChampionId::Rammus,"Раммус" => ChampionId::Rammus,"披甲龙龟" => ChampionId::Rammus,"拉姆斯" => ChampionId::Rammus,"ريكساي" => ChampionId::RekSai,"Rek'Sai" => ChampionId::RekSai,"Ρεκ'Σάι" => ChampionId::RekSai,"レク＝サイ" => ChampionId::RekSai,"렉사이" => ChampionId::RekSai,"Рек'Сай" => ChampionId::RekSai,"虚空遁地兽" => ChampionId::RekSai,"雷珂煞" => ChampionId::RekSai,"ريل" => ChampionId::Rell,"Rell" => ChampionId::Rell,"Ρελ" => ChampionId::Rell,"レル" => ChampionId::Rell,"렐" => ChampionId::Rell,"Релл" => ChampionId::Rell,"镕铁少女" => ChampionId::Rell,"锐儿" => ChampionId::Rell,"銳兒" => ChampionId::Rell,"ريناتا غلاسك" => ChampionId::Renata,"Renata Glasc" => ChampionId::Renata,"Ρενάτα Γκλασκ" => ChampionId::Renata,"レナータ・グラスク" => ChampionId::Renata,"레나타 글라스크" => ChampionId::Renata,"Рената Гласк" => ChampionId::Renata,"炼金男爵" => ChampionId::Renata,"睿娜妲‧格莱斯克" => ChampionId::Renata,"睿娜妲‧格萊斯克" => ChampionId::Renata,"رينيكتون" => ChampionId::Renekton,"Renekton" => ChampionId::Renekton,"Ρένεκτον" => ChampionId::Renekton,"レネクトン" => ChampionId::Renekton,"레넥톤" => ChampionId::Renekton,"Ренектон" => ChampionId::Renekton,"荒漠屠夫" => ChampionId::Renekton,"雷尼克顿" => ChampionId::Renekton,"雷尼克頓" => ChampionId::Renekton,"رينغار" => ChampionId::Rengar,"Rengar" => ChampionId::Rengar,"Ρένγκαρ" => ChampionId::Rengar,"レンガー" => ChampionId::Rengar,"렝가" => ChampionId::Rengar,"Ренгар" => ChampionId::Rengar,"傲之追猎者" => ChampionId::Rengar,"雷葛尔" => ChampionId::Rengar,"雷葛爾" => ChampionId::Rengar,"ريفين" => ChampionId::Riven,"Riven" => ChampionId::Riven,"Ρίβεν" => ChampionId::Riven,"リヴェン" => ChampionId::Riven,"리븐" => ChampionId::Riven,"Ривен" => ChampionId::Riven,"放逐之刃" => ChampionId::Riven,"雷玟" => ChampionId::Riven,"رامبل" => ChampionId::Rumble,"Rumble" => ChampionId::Rumble,"Ραμπλ" => ChampionId::Rumble,"ランブル" => ChampionId::Rumble,"럼블" => ChampionId::Rumble,"Рамбл" => ChampionId::Rumble,"机械公敌" => ChampionId::Rumble,"蓝宝" => ChampionId::Rumble,"藍寶" => ChampionId::Rumble,"رايز" => ChampionId::Ryze,"Ryze" => ChampionId::Ryze,"Ράιζ" => ChampionId::Ryze,"ライズ" => ChampionId::Ryze,"라이즈" => ChampionId::Ryze,"Райз" => ChampionId::Ryze,"符文法师" => ChampionId::Ryze,"雷兹" => ChampionId::Ryze,"雷茲" => ChampionId::Ryze,"سميرة" => ChampionId::Samira,"Samira" => ChampionId::Samira,"Σαμίρα" => ChampionId::Samira,"サミーラ" => ChampionId::Samira,"사미라" => ChampionId::Samira,"Самира" => ChampionId::Samira,"沙漠玫瑰" => ChampionId::Samira,"煞蜜拉" => ChampionId::Samira,"سيجواني" => ChampionId::Sejuani,"Sejuani" => ChampionId::Sejuani,"Σεζουάνι" => ChampionId::Sejuani,"セジュアニ" => ChampionId::Sejuani,"세주아니" => ChampionId::Sejuani,"Седжуани" => ChampionId::Sejuani,"北地之怒" => ChampionId::Sejuani,"史瓦妮" => ChampionId::Sejuani,"سينا" => ChampionId::Senna,"Senna" => ChampionId::Senna,"Σέννα" => ChampionId::Senna,"セナ" => ChampionId::Senna,"세나" => ChampionId::Senna,"Сенна" => ChampionId::Senna,"涤魂圣枪" => ChampionId::Senna,"姗娜" => ChampionId::Senna,"姍娜" => ChampionId::Senna,"سيرافين" => ChampionId::Seraphine,"Seraphine" => ChampionId::Seraphine,"Σεραφίν" => ChampionId::Seraphine,"Séraphine" => ChampionId::Seraphine,"セラフィーン" => ChampionId::Seraphine,"세라핀" => ChampionId::Seraphine,"Серафина" => ChampionId::Seraphine,"星籁歌姬" => ChampionId::Seraphine,"瑟菈纷" => ChampionId::Seraphine,"瑟菈紛" => ChampionId::Seraphine,"سيت" => ChampionId::Sett,"Sett" => ChampionId::Sett,"Σεττ" => ChampionId::Sett,"セト" => ChampionId::Sett,"세트" => ChampionId::Sett,"Сетт" => ChampionId::Sett,"腕豪" => ChampionId::Sett,"赛特" => ChampionId::Sett,"賽特" => ChampionId::Sett,"شاكو" => ChampionId::Shaco,"Shaco" => ChampionId::Shaco,"Σάκο" => ChampionId::Shaco,"シャコ" => ChampionId::Shaco,"샤코" => ChampionId::Shaco,"Шако" => ChampionId::Shaco,"恶魔小丑" => ChampionId::Shaco,"萨科" => ChampionId::Shaco,"薩科" => ChampionId::Shaco,"شين" => ChampionId::Shen,"Shen" => ChampionId::Shen,"Σεν" => ChampionId::Shen,"シェン" => ChampionId::Shen,"쉔" => ChampionId::Shen,"Шен" => ChampionId::Shen,"暮光之眼" => ChampionId::Shen,"慎" => ChampionId::Shen,"شيفانا" => ChampionId::Shyvana,"Shyvana" => ChampionId::Shyvana,"Σιβάνα" => ChampionId::Shyvana,"シヴァーナ" => ChampionId::Shyvana,"쉬바나" => ChampionId::Shyvana,"Шивана" => ChampionId::Shyvana,"龙血武姬" => ChampionId::Shyvana,"希瓦娜" => ChampionId::Shyvana,"سينجد" => ChampionId::Singed,"Singed" => ChampionId::Singed,"Σιντζντ" => ChampionId::Singed,"シンジド" => ChampionId::Singed,"신지드" => ChampionId::Singed,"Синджед" => ChampionId::Singed,"炼金术士" => ChampionId::Singed,"辛吉德" => ChampionId::Singed,"سايون" => ChampionId::Sion,"Sion" => ChampionId::Sion,"Σάιον" => ChampionId::Sion,"サイオン" => ChampionId::Sion,"사이온" => ChampionId::Sion,"Сион" => ChampionId::Sion,"亡灵战神" => ChampionId::Sion,"赛恩" => ChampionId::Sion,"賽恩" => ChampionId::Sion,"سيفير" => ChampionId::Sivir,"Sivir" => ChampionId::Sivir,"Σίβιρ" => ChampionId::Sivir,"シヴィア" => ChampionId::Sivir,"시비르" => ChampionId::Sivir,"Сивир" => ChampionId::Sivir,"战争女神" => ChampionId::Sivir,"希维尔" => ChampionId::Sivir,"希維爾" => ChampionId::Sivir,"سكارنر" => ChampionId::Skarner,"Skarner" => ChampionId::Skarner,"Σκάρνερ" => ChampionId::Skarner,"スカーナー" => ChampionId::Skarner,"스카너" => ChampionId::Skarner,"Скарнер" => ChampionId::Skarner,"上古领主" => ChampionId::Skarner,"史加纳" => ChampionId::Skarner,"史加納" => ChampionId::Skarner,"سمولدر" => ChampionId::Smolder,"Smolder" => ChampionId::Smolder,"Σμόλντερ" => ChampionId::Smolder,"スモルダー" => ChampionId::Smolder,"스몰더" => ChampionId::Smolder,"Смолдер" => ChampionId::Smolder,"炽炎雏龙" => ChampionId::Smolder,"烟炎" => ChampionId::Smolder,"史矛德" => ChampionId::Smolder,"سونا" => ChampionId::Sona,"Sona" => ChampionId::Sona,"Σόνα" => ChampionId::Sona,"ソナ" => ChampionId::Sona,"소나" => ChampionId::Sona,"Сона" => ChampionId::Sona,"琴瑟仙女" => ChampionId::Sona,"索娜" => ChampionId::Sona,"سوراكا" => ChampionId::Soraka,"Soraka" => ChampionId::Soraka,"Σοράκα" => ChampionId::Soraka,"ソラカ" => ChampionId::Soraka,"소라카" => ChampionId::Soraka,"Сорака" => ChampionId::Soraka,"众星之子" => ChampionId::Soraka,"索拉卡" => ChampionId::Soraka,"سواين" => ChampionId::Swain,"Swain" => ChampionId::Swain,"Σουέιν" => ChampionId::Swain,"スウェイン" => ChampionId::Swain,"스웨인" => ChampionId::Swain,"Свейн" => ChampionId::Swain,"诺克萨斯统领" => ChampionId::Swain,"斯温" => ChampionId::Swain,"斯溫" => ChampionId::Swain,"سايلاس" => ChampionId::Sylas,"Sylas" => ChampionId::Sylas,"Σάιλας" => ChampionId::Sylas,"サイラス" => ChampionId::Sylas,"사일러스" => ChampionId::Sylas,"Сайлас" => ChampionId::Sylas,"解脱者" => ChampionId::Sylas,"赛勒斯" => ChampionId::Sylas,"賽勒斯" => ChampionId::Sylas,"سيندرا" => ChampionId::Syndra,"Syndra" => ChampionId::Syndra,"Σίντρα" => ChampionId::Syndra,"シンドラ" => ChampionId::Syndra,"신드라" => ChampionId::Syndra,"Синдра" => ChampionId::Syndra,"暗黑元首" => ChampionId::Syndra,"星朵拉" => ChampionId::Syndra,"تام كينش" => ChampionId::TahmKench,"Tahm Kench" => ChampionId::TahmKench,"Ταμ Κεντς" => ChampionId::TahmKench,"タム・ケンチ" => ChampionId::TahmKench,"탐 켄치" => ChampionId::TahmKench,"Таам Кенч" => ChampionId::TahmKench,"河流之王" => ChampionId::TahmKench,"贪啃奇" => ChampionId::TahmKench,"貪啃奇" => ChampionId::TahmKench,"تاليا" => ChampionId::Taliyah,"Taliyah" => ChampionId::Taliyah,"Τάλια" => ChampionId::Taliyah,"タリヤ" => ChampionId::Taliyah,"탈리야" => ChampionId::Taliyah,"Талия" => ChampionId::Taliyah,"岩雀" => ChampionId::Taliyah,"塔莉雅" => ChampionId::Taliyah,"تالون" => ChampionId::Talon,"Talon" => ChampionId::Talon,"Τάλον" => ChampionId::Talon,"タロン" => ChampionId::Talon,"탈론" => ChampionId::Talon,"Талон" => ChampionId::Talon,"刀锋之影" => ChampionId::Talon,"塔隆" => ChampionId::Talon,"تاريك" => ChampionId::Taric,"Taric" => ChampionId::Taric,"Τάρικ" => ChampionId::Taric,"タリック" => ChampionId::Taric,"타릭" => ChampionId::Taric,"Тарик" => ChampionId::Taric,"瓦洛兰之盾" => ChampionId::Taric,"塔里克" => ChampionId::Taric,"تيمو" => ChampionId::Teemo,"Teemo" => ChampionId::Teemo,"Τίμο" => ChampionId::Teemo,"ティーモ" => ChampionId::Teemo,"티모" => ChampionId::Teemo,"Тимо" => ChampionId::Teemo,"迅捷斥候" => ChampionId::Teemo,"提摩" => ChampionId::Teemo,"ثريش" => ChampionId::Thresh,"Thresh" => ChampionId::Thresh,"Θρες" => ChampionId::Thresh,"スレッシュ" => ChampionId::Thresh,"쓰레쉬" => ChampionId::Thresh,"Треш" => ChampionId::Thresh,"魂锁典狱长" => ChampionId::Thresh,"瑟雷西" => ChampionId::Thresh,"تريستانا" => ChampionId::Tristana,"Tristana" => ChampionId::Tristana,"Τριστάνα" => ChampionId::Tristana,"トリスターナ" => ChampionId::Tristana,"트리스타나" => ChampionId::Tristana,"Тристана" => ChampionId::Tristana,"麦林炮手" => ChampionId::Tristana,"崔丝塔娜" => ChampionId::Tristana,"崔絲塔娜" => ChampionId::Tristana,"ترندل" => ChampionId::Trundle,"Trundle" => ChampionId::Trundle,"Τραντλ" => ChampionId::Trundle,"トランドル" => ChampionId::Trundle,"트런들" => ChampionId::Trundle,"Трандл" => ChampionId::Trundle,"巨魔之王" => ChampionId::Trundle,"特朗德" => ChampionId::Trundle,"تريندامير" => ChampionId::Tryndamere,"Tryndamere" => ChampionId::Tryndamere,"Τρίνταμερ" => ChampionId::Tryndamere,"トリンダメア" => ChampionId::Tryndamere,"트린다미어" => ChampionId::Tryndamere,"Триндамир" => ChampionId::Tryndamere,"蛮族之王" => ChampionId::Tryndamere,"泰达米尔" => ChampionId::Tryndamere,"泰達米爾" => ChampionId::Tryndamere,"تويستد فايت" => ChampionId::TwistedFate,"Twisted Fate" => ChampionId::TwistedFate,"Τουίστεντ Φέιτ" => ChampionId::TwistedFate,"ツイステッド・フェイト" => ChampionId::TwistedFate,"트위스티드 페이트" => ChampionId::TwistedFate,"Твистед Фэйт" => ChampionId::TwistedFate,"卡牌大师" => ChampionId::TwistedFate,"逆命" => ChampionId::TwistedFate,"تويتش" => ChampionId::Twitch,"Twitch" => ChampionId::Twitch,"Τουίτς" => ChampionId::Twitch,"トゥイッチ" => ChampionId::Twitch,"트위치" => ChampionId::Twitch,"Твич" => ChampionId::Twitch,"瘟疫之源" => ChampionId::Twitch,"图奇" => ChampionId::Twitch,"圖奇" => ChampionId::Twitch,"أودير" => ChampionId::Udyr,"Udyr" => ChampionId::Udyr,"Ούντιρ" => ChampionId::Udyr,"ウディア" => ChampionId::Udyr,"우디르" => ChampionId::Udyr,"Удир" => ChampionId::Udyr,"兽灵行者" => ChampionId::Udyr,"乌迪尔" => ChampionId::Udyr,"烏迪爾" => ChampionId::Udyr,"أورغوت" => ChampionId::Urgot,"Urgot" => ChampionId::Urgot,"Ούργκοτ" => ChampionId::Urgot,"アーゴット" => ChampionId::Urgot,"우르곳" => ChampionId::Urgot,"Ургот" => ChampionId::Urgot,"无畏战车" => ChampionId::Urgot,"乌尔加特" => ChampionId::Urgot,"烏爾加特" => ChampionId::Urgot,"فاروس" => ChampionId::Varus,"Varus" => ChampionId::Varus,"Βάρους" => ChampionId::Varus,"ヴァルス" => ChampionId::Varus,"바루스" => ChampionId::Varus,"Варус" => ChampionId::Varus,"惩戒之箭" => ChampionId::Varus,"法洛士" => ChampionId::Varus,"فاين" => ChampionId::Vayne,"Vayne" => ChampionId::Vayne,"Βέιν" => ChampionId::Vayne,"ヴェイン" => ChampionId::Vayne,"베인" => ChampionId::Vayne,"Вейн" => ChampionId::Vayne,"暗夜猎手" => ChampionId::Vayne,"汎" => ChampionId::Vayne,"فيغار" => ChampionId::Veigar,"Veigar" => ChampionId::Veigar,"Βέιγκαρ" => ChampionId::Veigar,"ベイガー" => ChampionId::Veigar,"베이가" => ChampionId::Veigar,"Вейгар" => ChampionId::Veigar,"邪恶小法师" => ChampionId::Veigar,"维迦" => ChampionId::Veigar,"維迦" => ChampionId::Veigar,"فيلكوز" => ChampionId::Velkoz,"Vel'Koz" => ChampionId::Velkoz,"Βελ'Κοζ" => ChampionId::Velkoz,"ヴェル＝コズ" => ChampionId::Velkoz,"벨코즈" => ChampionId::Velkoz,"Вел'Коз" => ChampionId::Velkoz,"虚空之眼" => ChampionId::Velkoz,"威寇兹" => ChampionId::Velkoz,"威寇茲" => ChampionId::Velkoz,"فيكس" => ChampionId::Vex,"Vex" => ChampionId::Vex,"Βεξ" => ChampionId::Vex,"ヴェックス" => ChampionId::Vex,"벡스" => ChampionId::Vex,"Векс" => ChampionId::Vex,"愁云使者" => ChampionId::Vex,"薇可丝" => ChampionId::Vex,"薇可絲" => ChampionId::Vex,"فاي" => ChampionId::Vi,"Vi" => ChampionId::Vi,"Βάι" => ChampionId::Vi,"ヴァイ" => ChampionId::Vi,"바이" => ChampionId::Vi,"Вай" => ChampionId::Vi,"皮城执法官" => ChampionId::Vi,"菲艾" => ChampionId::Vi,"فييغو" => ChampionId::Viego,"Viego" => ChampionId::Viego,"Βιέγκο" => ChampionId::Viego,"ヴィエゴ" => ChampionId::Viego,"비에고" => ChampionId::Viego,"Виего" => ChampionId::Viego,"破败之王" => ChampionId::Viego,"维尔戈" => ChampionId::Viego,"維爾戈" => ChampionId::Viego,"فيكتور" => ChampionId::Viktor,"Viktor" => ChampionId::Viktor,"Βίκτορ" => ChampionId::Viktor,"ビクター" => ChampionId::Viktor,"빅토르" => ChampionId::Viktor,"Виктор" => ChampionId::Viktor,"奥术先驱" => ChampionId::Viktor,"维克特" => ChampionId::Viktor,"維克特" => ChampionId::Viktor,"فلاديمير" => ChampionId::Vladimir,"Vladimir" => ChampionId::Vladimir,"Βλάντιμιρ" => ChampionId::Vladimir,"ブラッドミア" => ChampionId::Vladimir,"블라디미르" => ChampionId::Vladimir,"Владимир" => ChampionId::Vladimir,"猩红收割者" => ChampionId::Vladimir,"弗拉迪米尔" => ChampionId::Vladimir,"弗拉迪米爾" => ChampionId::Vladimir,"فوليبير" => ChampionId::Volibear,"Volibear" => ChampionId::Volibear,"Βόλιμπεαρ" => ChampionId::Volibear,"ボリベア" => ChampionId::Volibear,"볼리베어" => ChampionId::Volibear,"Волибир" => ChampionId::Volibear,"不灭狂雷" => ChampionId::Volibear,"弗力贝尔" => ChampionId::Volibear,"弗力貝爾" => ChampionId::Volibear,"وارويك" => ChampionId::Warwick,"Warwick" => ChampionId::Warwick,"Γουόργουικ" => ChampionId::Warwick,"ワーウィック" => ChampionId::Warwick,"워윅" => ChampionId::Warwick,"Варвик" => ChampionId::Warwick,"祖安怒兽" => ChampionId::Warwick,"沃维克" => ChampionId::Warwick,"沃維克" => ChampionId::Warwick,"زايا" => ChampionId::Xayah,"Xayah" => ChampionId::Xayah,"Ζάια" => ChampionId::Xayah,"ザヤ" => ChampionId::Xayah,"자야" => ChampionId::Xayah,"Шая" => ChampionId::Xayah,"逆羽" => ChampionId::Xayah,"刹雅" => ChampionId::Xayah,"剎雅" => ChampionId::Xayah,"زيراث" => ChampionId::Xerath,"Xerath" => ChampionId::Xerath,"Ζέραθ" => ChampionId::Xerath,"ゼラス" => ChampionId::Xerath,"제라스" => ChampionId::Xerath,"Зерат" => ChampionId::Xerath,"远古巫灵" => ChampionId::Xerath,"齐勒斯" => ChampionId::Xerath,"齊勒斯" => ChampionId::Xerath,"شين جاو" => ChampionId::XinZhao,"Xin Zhao" => ChampionId::XinZhao,"Ζιν Ζάο" => ChampionId::XinZhao,"シン・ジャオ" => ChampionId::XinZhao,"신 짜오" => ChampionId::XinZhao,"Ксин Жао" => ChampionId::XinZhao,"德邦总管" => ChampionId::XinZhao,"赵信" => ChampionId::XinZhao,"趙信" => ChampionId::XinZhao,"ياسو" => ChampionId::Yasuo,"Yasuo" => ChampionId::Yasuo,"Υασούο" => ChampionId::Yasuo,"ヤスオ" => ChampionId::Yasuo,"야스오" => ChampionId::Yasuo,"Ясуо" => ChampionId::Yasuo,"疾风剑豪" => ChampionId::Yasuo,"犽宿" => ChampionId::Yasuo,"يوني" => ChampionId::Yone,"Yone" => ChampionId::Yone,"Γιόνε" => ChampionId::Yone,"ヨネ" => ChampionId::Yone,"요네" => ChampionId::Yone,"Ёнэ" => ChampionId::Yone,"封魔剑魂" => ChampionId::Yone,"犽凝" => ChampionId::Yone,"يوريك" => ChampionId::Yorick,"Yorick" => ChampionId::Yorick,"Γιόρικ" => ChampionId::Yorick,"ヨリック" => ChampionId::Yorick,"요릭" => ChampionId::Yorick,"Йорик" => ChampionId::Yorick,"牧魂人" => ChampionId::Yorick,"约瑞科" => ChampionId::Yorick,"約瑞科" => ChampionId::Yorick,"يونارا" => ChampionId::Yunara,"Yunara" => ChampionId::Yunara,"Γιουνάρα" => ChampionId::Yunara,"ユナラ" => ChampionId::Yunara,"유나라" => ChampionId::Yunara,"Юнара" => ChampionId::Yunara,"不破之誓" => ChampionId::Yunara,"尤娜拉" => ChampionId::Yunara,"يومي" => ChampionId::Yuumi,"Yuumi" => ChampionId::Yuumi,"Γιούμι" => ChampionId::Yuumi,"ユーミ" => ChampionId::Yuumi,"유미" => ChampionId::Yuumi,"Юми" => ChampionId::Yuumi,"魔法猫咪" => ChampionId::Yuumi,"悠咪" => ChampionId::Yuumi,"زاهان" => ChampionId::Zaahen,"Zaahen" => ChampionId::Zaahen,"Ζάαχεν" => ChampionId::Zaahen,"ザーヘン" => ChampionId::Zaahen,"자헨" => ChampionId::Zaahen,"Заахен" => ChampionId::Zaahen,"不落魔锋" => ChampionId::Zaahen,"扎罕" => ChampionId::Zaahen,"薩亨" => ChampionId::Zaahen,"زاك" => ChampionId::Zac,"Zac" => ChampionId::Zac,"Ζακ" => ChampionId::Zac,"ザック" => ChampionId::Zac,"자크" => ChampionId::Zac,"Зак" => ChampionId::Zac,"生化魔人" => ChampionId::Zac,"札克" => ChampionId::Zac,"زيد" => ChampionId::Zed,"Zed" => ChampionId::Zed,"Ζεντ" => ChampionId::Zed,"ゼド" => ChampionId::Zed,"제드" => ChampionId::Zed,"Зед" => ChampionId::Zed,"影流之主" => ChampionId::Zed,"劫" => ChampionId::Zed,"زيري" => ChampionId::Zeri,"Zeri" => ChampionId::Zeri,"Ζέρι" => ChampionId::Zeri,"ゼリ" => ChampionId::Zeri,"제리" => ChampionId::Zeri,"Зери" => ChampionId::Zeri,"祖安花火" => ChampionId::Zeri,"婕莉" => ChampionId::Zeri,"زيغز" => ChampionId::Ziggs,"Ziggs" => ChampionId::Ziggs,"Ζιγκζ" => ChampionId::Ziggs,"ジグス" => ChampionId::Ziggs,"직스" => ChampionId::Ziggs,"Зиггс" => ChampionId::Ziggs,"爆破鬼才" => ChampionId::Ziggs,"希格斯" => ChampionId::Ziggs,"زيليان" => ChampionId::Zilean,"Zilean" => ChampionId::Zilean,"Ζίλεαν" => ChampionId::Zilean,"ジリアン" => ChampionId::Zilean,"질리언" => ChampionId::Zilean,"Зилеан" => ChampionId::Zilean,"时光守护者" => ChampionId::Zilean,"极灵" => ChampionId::Zilean,"極靈" => ChampionId::Zilean,"زوي" => ChampionId::Zoe,"Zoe" => ChampionId::Zoe,"Ζόη" => ChampionId::Zoe,"Zoé" => ChampionId::Zoe,"ゾーイ" => ChampionId::Zoe,"조이" => ChampionId::Zoe,"Зои" => ChampionId::Zoe,"暮光星灵" => ChampionId::Zoe,"柔依" => ChampionId::Zoe,"زايرا" => ChampionId::Zyra,"Zyra" => ChampionId::Zyra,"Ζάιρα" => ChampionId::Zyra,"ザイラ" => ChampionId::Zyra,"자이라" => ChampionId::Zyra,"Зайра" => ChampionId::Zyra,"荆棘之兴" => ChampionId::Zyra,"枷萝" => ChampionId::Zyra,"枷蘿" => ChampionId::Zyra
};
pub static CHAMPION_CACHE: [&CachedChampion; 172] = [
    &AATROX,
    &AHRI,
    &AKALI,
    &AKSHAN,
    &ALISTAR,
    &AMBESSA,
    &AMUMU,
    &ANIVIA,
    &ANNIE,
    &APHELIOS,
    &ASHE,
    &AURELIONSOL,
    &AURORA,
    &AZIR,
    &BARD,
    &BELVETH,
    &BLITZCRANK,
    &BRAND,
    &BRAUM,
    &BRIAR,
    &CAITLYN,
    &CAMILLE,
    &CASSIOPEIA,
    &CHOGATH,
    &CORKI,
    &DARIUS,
    &DIANA,
    &DRMUNDO,
    &DRAVEN,
    &EKKO,
    &ELISE,
    &EVELYNN,
    &EZREAL,
    &FIDDLESTICKS,
    &FIORA,
    &FIZZ,
    &GALIO,
    &GANGPLANK,
    &GAREN,
    &GNAR,
    &GRAGAS,
    &GRAVES,
    &GWEN,
    &HECARIM,
    &HEIMERDINGER,
    &HWEI,
    &ILLAOI,
    &IRELIA,
    &IVERN,
    &JANNA,
    &JARVANIV,
    &JAX,
    &JAYCE,
    &JHIN,
    &JINX,
    &KSANTE,
    &KAISA,
    &KALISTA,
    &KARMA,
    &KARTHUS,
    &KASSADIN,
    &KATARINA,
    &KAYLE,
    &KAYN,
    &KENNEN,
    &KHAZIX,
    &KINDRED,
    &KLED,
    &KOGMAW,
    &LEBLANC,
    &LEESIN,
    &LEONA,
    &LILLIA,
    &LISSANDRA,
    &LUCIAN,
    &LULU,
    &LUX,
    &MALPHITE,
    &MALZAHAR,
    &MAOKAI,
    &MASTERYI,
    &MEL,
    &MILIO,
    &MISSFORTUNE,
    &MONKEYKING,
    &MORDEKAISER,
    &MORGANA,
    &NAAFIRI,
    &NAMI,
    &NASUS,
    &NAUTILUS,
    &NEEKO,
    &NIDALEE,
    &NILAH,
    &NOCTURNE,
    &NUNU,
    &OLAF,
    &ORIANNA,
    &ORNN,
    &PANTHEON,
    &POPPY,
    &PYKE,
    &QIYANA,
    &QUINN,
    &RAKAN,
    &RAMMUS,
    &REKSAI,
    &RELL,
    &RENATA,
    &RENEKTON,
    &RENGAR,
    &RIVEN,
    &RUMBLE,
    &RYZE,
    &SAMIRA,
    &SEJUANI,
    &SENNA,
    &SERAPHINE,
    &SETT,
    &SHACO,
    &SHEN,
    &SHYVANA,
    &SINGED,
    &SION,
    &SIVIR,
    &SKARNER,
    &SMOLDER,
    &SONA,
    &SORAKA,
    &SWAIN,
    &SYLAS,
    &SYNDRA,
    &TAHMKENCH,
    &TALIYAH,
    &TALON,
    &TARIC,
    &TEEMO,
    &THRESH,
    &TRISTANA,
    &TRUNDLE,
    &TRYNDAMERE,
    &TWISTEDFATE,
    &TWITCH,
    &UDYR,
    &URGOT,
    &VARUS,
    &VAYNE,
    &VEIGAR,
    &VELKOZ,
    &VEX,
    &VI,
    &VIEGO,
    &VIKTOR,
    &VLADIMIR,
    &VOLIBEAR,
    &WARWICK,
    &XAYAH,
    &XERATH,
    &XINZHAO,
    &YASUO,
    &YONE,
    &YORICK,
    &YUNARA,
    &YUUMI,
    &ZAAHEN,
    &ZAC,
    &ZED,
    &ZERI,
    &ZIGGS,
    &ZILEAN,
    &ZOE,
    &ZYRA,
];
pub const fn ability_const_eval(
    ctx: &EvalContext,
    champion_id: ChampionId,
    kind: AbilityId,
) -> f32 {
    match champion_id {
        ChampionId::Aatrox => match kind {
            AbilityId::P(AbilityName::Void) => aatrox_p_void(ctx),
            AbilityId::Q(AbilityName::Min) => aatrox_q_min(ctx),
            AbilityId::Q(AbilityName::_1Min) => aatrox_q_1min(ctx),
            AbilityId::Q(AbilityName::_2Min) => aatrox_q_2min(ctx),
            AbilityId::Q(AbilityName::_3Min) => aatrox_q_3min(ctx),
            AbilityId::Q(AbilityName::Max) => aatrox_q_max(ctx),
            AbilityId::Q(AbilityName::_1Max) => aatrox_q_1max(ctx),
            AbilityId::Q(AbilityName::_2Max) => aatrox_q_2max(ctx),
            AbilityId::Q(AbilityName::_3Max) => aatrox_q_3max(ctx),
            AbilityId::W(AbilityName::Min) => aatrox_w_min(ctx),
            AbilityId::W(AbilityName::Max) => aatrox_w_max(ctx),
            _ => panic!("Invalid AbilityId for 'Aatrox'"),
        },
        ChampionId::Ahri => match kind {
            AbilityId::Q(AbilityName::Min) => ahri_q_min(ctx),
            AbilityId::Q(AbilityName::Max) => ahri_q_max(ctx),
            AbilityId::W(AbilityName::_1) => ahri_w_1(ctx),
            AbilityId::W(AbilityName::Min) => ahri_w_min(ctx),
            AbilityId::W(AbilityName::Max) => ahri_w_max(ctx),
            AbilityId::E(AbilityName::Void) => ahri_e_void(ctx),
            AbilityId::R(AbilityName::Min) => ahri_r_min(ctx),
            AbilityId::R(AbilityName::Max) => ahri_r_max(ctx),
            _ => panic!("Invalid AbilityId for 'Ahri'"),
        },
        ChampionId::Akali => match kind {
            AbilityId::P(AbilityName::Void) => akali_p_void(ctx),
            AbilityId::Q(AbilityName::Void) => akali_q_void(ctx),
            AbilityId::E(AbilityName::_1Min) => akali_e_1min(ctx),
            AbilityId::E(AbilityName::Max) => akali_e_max(ctx),
            AbilityId::E(AbilityName::_1Max) => akali_e_1max(ctx),
            AbilityId::R(AbilityName::_1) => akali_r_1(ctx),
            AbilityId::R(AbilityName::_2Min) => akali_r_2min(ctx),
            AbilityId::R(AbilityName::_2Max) => akali_r_2max(ctx),
            _ => panic!("Invalid AbilityId for 'Akali'"),
        },
        ChampionId::Akshan => match kind {
            AbilityId::P(AbilityName::Void) => akshan_p_void(ctx),
            AbilityId::Q(AbilityName::Min) => akshan_q_min(ctx),
            AbilityId::Q(AbilityName::Max) => akshan_q_max(ctx),
            AbilityId::E(AbilityName::Void) => akshan_e_void(ctx),
            AbilityId::R(AbilityName::_1Min) => akshan_r_1min(ctx),
            AbilityId::R(AbilityName::_2Min) => akshan_r_2min(ctx),
            AbilityId::R(AbilityName::_1Max) => akshan_r_1max(ctx),
            AbilityId::R(AbilityName::_2Max) => akshan_r_2max(ctx),
            _ => panic!("Invalid AbilityId for 'Akshan'"),
        },
        ChampionId::Alistar => match kind {
            AbilityId::Q(AbilityName::_1) => alistar_q_1(ctx),
            AbilityId::W(AbilityName::_1) => alistar_w_1(ctx),
            AbilityId::E(AbilityName::_1) => alistar_e_1(ctx),
            AbilityId::E(AbilityName::_2) => alistar_e_2(ctx),
            AbilityId::R(AbilityName::_1) => alistar_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Alistar'"),
        },
        ChampionId::Ambessa => match kind {
            AbilityId::Q(AbilityName::_3) => ambessa_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => ambessa_q_4(ctx),
            AbilityId::W(AbilityName::_1) => ambessa_w_1(ctx),
            AbilityId::W(AbilityName::_2) => ambessa_w_2(ctx),
            AbilityId::E(AbilityName::_1) => ambessa_e_1(ctx),
            AbilityId::E(AbilityName::_2) => ambessa_e_2(ctx),
            AbilityId::R(AbilityName::_1) => ambessa_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Ambessa'"),
        },
        ChampionId::Amumu => match kind {
            AbilityId::Q(AbilityName::_1) => amumu_q_1(ctx),
            AbilityId::W(AbilityName::_1) => amumu_w_1(ctx),
            AbilityId::E(AbilityName::_1) => amumu_e_1(ctx),
            AbilityId::E(AbilityName::_2) => amumu_e_2(ctx),
            AbilityId::R(AbilityName::_1) => amumu_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Amumu'"),
        },
        ChampionId::Anivia => match kind {
            AbilityId::Q(AbilityName::_1) => anivia_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => anivia_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => anivia_q_3(ctx),
            AbilityId::E(AbilityName::_1) => anivia_e_1(ctx),
            AbilityId::E(AbilityName::_2) => anivia_e_2(ctx),
            AbilityId::R(AbilityName::_1) => anivia_r_1(ctx),
            AbilityId::R(AbilityName::_2) => anivia_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Anivia'"),
        },
        ChampionId::Annie => match kind {
            AbilityId::Q(AbilityName::_1) => annie_q_1(ctx),
            AbilityId::W(AbilityName::_1) => annie_w_1(ctx),
            AbilityId::E(AbilityName::_1) => annie_e_1(ctx),
            AbilityId::R(AbilityName::_1) => annie_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Annie'"),
        },
        ChampionId::Aphelios => match kind {
            AbilityId::P(AbilityName::_1) => aphelios_p_1(ctx),
            _ => panic!("Invalid AbilityId for 'Aphelios'"),
        },
        ChampionId::Ashe => match kind {
            AbilityId::Q(AbilityName::_1) => ashe_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => ashe_q_2(ctx),
            AbilityId::W(AbilityName::_1) => ashe_w_1(ctx),
            AbilityId::R(AbilityName::_1) => ashe_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Ashe'"),
        },
        ChampionId::AurelionSol => match kind {
            AbilityId::Q(AbilityName::_1) => aurelionsol_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => aurelionsol_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => aurelionsol_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => aurelionsol_q_4(ctx),
            AbilityId::W(AbilityName::_1) => aurelionsol_w_1(ctx),
            AbilityId::E(AbilityName::_1) => aurelionsol_e_1(ctx),
            AbilityId::E(AbilityName::_2) => aurelionsol_e_2(ctx),
            AbilityId::R(AbilityName::_2) => aurelionsol_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'AurelionSol'"),
        },
        ChampionId::Aurora => match kind {
            AbilityId::Q(AbilityName::_1) => aurora_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => aurora_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => aurora_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => aurora_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => aurora_q_5(ctx),
            AbilityId::E(AbilityName::_1) => aurora_e_1(ctx),
            AbilityId::R(AbilityName::_1) => aurora_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Aurora'"),
        },
        ChampionId::Azir => match kind {
            AbilityId::Q(AbilityName::_1) => azir_q_1(ctx),
            AbilityId::W(AbilityName::_1) => azir_w_1(ctx),
            AbilityId::E(AbilityName::_1) => azir_e_1(ctx),
            AbilityId::R(AbilityName::_1) => azir_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Azir'"),
        },
        ChampionId::Bard => match kind {
            AbilityId::Q(AbilityName::_1) => bard_q_1(ctx),
            _ => panic!("Invalid AbilityId for 'Bard'"),
        },
        ChampionId::Belveth => match kind {
            AbilityId::Q(AbilityName::_1) => belveth_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => belveth_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => belveth_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => belveth_q_4(ctx),
            AbilityId::W(AbilityName::_1) => belveth_w_1(ctx),
            AbilityId::E(AbilityName::_1) => belveth_e_1(ctx),
            AbilityId::E(AbilityName::_2) => belveth_e_2(ctx),
            AbilityId::E(AbilityName::_3) => belveth_e_3(ctx),
            AbilityId::E(AbilityName::_4) => belveth_e_4(ctx),
            AbilityId::E(AbilityName::_5) => belveth_e_5(ctx),
            AbilityId::R(AbilityName::_1) => belveth_r_1(ctx),
            AbilityId::R(AbilityName::_2) => belveth_r_2(ctx),
            AbilityId::R(AbilityName::_3) => belveth_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Belveth'"),
        },
        ChampionId::Blitzcrank => match kind {
            AbilityId::Q(AbilityName::_1) => blitzcrank_q_1(ctx),
            AbilityId::R(AbilityName::_1) => blitzcrank_r_1(ctx),
            AbilityId::R(AbilityName::_2) => blitzcrank_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Blitzcrank'"),
        },
        ChampionId::Brand => match kind {
            AbilityId::Q(AbilityName::_1) => brand_q_1(ctx),
            AbilityId::W(AbilityName::_1) => brand_w_1(ctx),
            AbilityId::W(AbilityName::_2) => brand_w_2(ctx),
            AbilityId::E(AbilityName::_1) => brand_e_1(ctx),
            AbilityId::R(AbilityName::_1) => brand_r_1(ctx),
            AbilityId::R(AbilityName::_2) => brand_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Brand'"),
        },
        ChampionId::Braum => match kind {
            AbilityId::Q(AbilityName::_1) => braum_q_1(ctx),
            AbilityId::E(AbilityName::_1) => braum_e_1(ctx),
            AbilityId::R(AbilityName::_1) => braum_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Braum'"),
        },
        ChampionId::Briar => match kind {
            AbilityId::Q(AbilityName::_1) => briar_q_1(ctx),
            AbilityId::W(AbilityName::_1) => briar_w_1(ctx),
            AbilityId::E(AbilityName::_1) => briar_e_1(ctx),
            AbilityId::E(AbilityName::_2) => briar_e_2(ctx),
            AbilityId::E(AbilityName::_3) => briar_e_3(ctx),
            AbilityId::E(AbilityName::_4) => briar_e_4(ctx),
            AbilityId::R(AbilityName::_1) => briar_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Briar'"),
        },
        ChampionId::Caitlyn => match kind {
            AbilityId::Q(AbilityName::_1) => caitlyn_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => caitlyn_q_2(ctx),
            AbilityId::W(AbilityName::_1) => caitlyn_w_1(ctx),
            AbilityId::E(AbilityName::_1) => caitlyn_e_1(ctx),
            AbilityId::R(AbilityName::_1) => caitlyn_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Caitlyn'"),
        },
        ChampionId::Camille => match kind {
            AbilityId::Q(AbilityName::_1) => camille_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => camille_q_2(ctx),
            AbilityId::W(AbilityName::_1) => camille_w_1(ctx),
            AbilityId::W(AbilityName::_2) => camille_w_2(ctx),
            AbilityId::W(AbilityName::_3) => camille_w_3(ctx),
            AbilityId::W(AbilityName::_4) => camille_w_4(ctx),
            AbilityId::R(AbilityName::_1) => camille_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Camille'"),
        },
        ChampionId::Cassiopeia => match kind {
            AbilityId::Q(AbilityName::_1) => cassiopeia_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => cassiopeia_q_2(ctx),
            AbilityId::W(AbilityName::_1) => cassiopeia_w_1(ctx),
            AbilityId::W(AbilityName::_2) => cassiopeia_w_2(ctx),
            AbilityId::E(AbilityName::_1) => cassiopeia_e_1(ctx),
            AbilityId::E(AbilityName::_2) => cassiopeia_e_2(ctx),
            AbilityId::R(AbilityName::_1) => cassiopeia_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Cassiopeia'"),
        },
        ChampionId::Chogath => match kind {
            AbilityId::Q(AbilityName::_1) => chogath_q_1(ctx),
            AbilityId::W(AbilityName::_1) => chogath_w_1(ctx),
            AbilityId::E(AbilityName::_1) => chogath_e_1(ctx),
            AbilityId::E(AbilityName::_2) => chogath_e_2(ctx),
            AbilityId::R(AbilityName::_1) => chogath_r_1(ctx),
            AbilityId::R(AbilityName::_2) => chogath_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Chogath'"),
        },
        ChampionId::Corki => match kind {
            AbilityId::Q(AbilityName::_1) => corki_q_1(ctx),
            AbilityId::W(AbilityName::_1) => corki_w_1(ctx),
            AbilityId::W(AbilityName::_2) => corki_w_2(ctx),
            AbilityId::E(AbilityName::_1) => corki_e_1(ctx),
            AbilityId::E(AbilityName::_2) => corki_e_2(ctx),
            AbilityId::R(AbilityName::_1) => corki_r_1(ctx),
            AbilityId::R(AbilityName::_2) => corki_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Corki'"),
        },
        ChampionId::Darius => match kind {
            AbilityId::Q(AbilityName::_1) => darius_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => darius_q_2(ctx),
            AbilityId::W(AbilityName::_1) => darius_w_1(ctx),
            AbilityId::R(AbilityName::_1) => darius_r_1(ctx),
            AbilityId::R(AbilityName::_2) => darius_r_2(ctx),
            AbilityId::R(AbilityName::_3) => darius_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Darius'"),
        },
        ChampionId::Diana => match kind {
            AbilityId::Q(AbilityName::_1) => diana_q_1(ctx),
            AbilityId::W(AbilityName::_1) => diana_w_1(ctx),
            AbilityId::W(AbilityName::_2) => diana_w_2(ctx),
            AbilityId::E(AbilityName::_1) => diana_e_1(ctx),
            AbilityId::R(AbilityName::_1) => diana_r_1(ctx),
            AbilityId::R(AbilityName::_2) => diana_r_2(ctx),
            AbilityId::R(AbilityName::_3) => diana_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Diana'"),
        },
        ChampionId::DrMundo => match kind {
            AbilityId::Q(AbilityName::_1) => drmundo_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => drmundo_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => drmundo_q_3(ctx),
            AbilityId::W(AbilityName::_1) => drmundo_w_1(ctx),
            AbilityId::W(AbilityName::_2) => drmundo_w_2(ctx),
            AbilityId::W(AbilityName::_3) => drmundo_w_3(ctx),
            AbilityId::E(AbilityName::_1) => drmundo_e_1(ctx),
            AbilityId::E(AbilityName::_2) => drmundo_e_2(ctx),
            AbilityId::E(AbilityName::_3) => drmundo_e_3(ctx),
            _ => panic!("Invalid AbilityId for 'DrMundo'"),
        },
        ChampionId::Draven => match kind {
            AbilityId::Q(AbilityName::_1) => draven_q_1(ctx),
            AbilityId::E(AbilityName::_1) => draven_e_1(ctx),
            AbilityId::R(AbilityName::_1) => draven_r_1(ctx),
            AbilityId::R(AbilityName::_2) => draven_r_2(ctx),
            AbilityId::R(AbilityName::_3) => draven_r_3(ctx),
            AbilityId::R(AbilityName::_4) => draven_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Draven'"),
        },
        ChampionId::Ekko => match kind {
            AbilityId::Q(AbilityName::_1) => ekko_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => ekko_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => ekko_q_3(ctx),
            AbilityId::E(AbilityName::_1) => ekko_e_1(ctx),
            AbilityId::R(AbilityName::_1) => ekko_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Ekko'"),
        },
        ChampionId::Elise => match kind {
            AbilityId::Q(AbilityName::_3) => elise_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => elise_q_4(ctx),
            AbilityId::W(AbilityName::_1) => elise_w_1(ctx),
            _ => panic!("Invalid AbilityId for 'Elise'"),
        },
        ChampionId::Evelynn => match kind {
            AbilityId::Q(AbilityName::_1) => evelynn_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => evelynn_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => evelynn_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => evelynn_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => evelynn_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => evelynn_q_6(ctx),
            AbilityId::W(AbilityName::_1) => evelynn_w_1(ctx),
            AbilityId::E(AbilityName::_2) => evelynn_e_2(ctx),
            AbilityId::R(AbilityName::_1) => evelynn_r_1(ctx),
            AbilityId::R(AbilityName::_2) => evelynn_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Evelynn'"),
        },
        ChampionId::Ezreal => match kind {
            AbilityId::Q(AbilityName::_1) => ezreal_q_1(ctx),
            AbilityId::W(AbilityName::_1) => ezreal_w_1(ctx),
            AbilityId::E(AbilityName::_1) => ezreal_e_1(ctx),
            AbilityId::R(AbilityName::_1) => ezreal_r_1(ctx),
            AbilityId::R(AbilityName::_2) => ezreal_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Ezreal'"),
        },
        ChampionId::Fiddlesticks => match kind {
            AbilityId::Q(AbilityName::_1) => fiddlesticks_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => fiddlesticks_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => fiddlesticks_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => fiddlesticks_q_4(ctx),
            AbilityId::W(AbilityName::_1) => fiddlesticks_w_1(ctx),
            AbilityId::W(AbilityName::_2) => fiddlesticks_w_2(ctx),
            AbilityId::W(AbilityName::_3) => fiddlesticks_w_3(ctx),
            AbilityId::W(AbilityName::_4) => fiddlesticks_w_4(ctx),
            AbilityId::E(AbilityName::_1) => fiddlesticks_e_1(ctx),
            AbilityId::R(AbilityName::_1) => fiddlesticks_r_1(ctx),
            AbilityId::R(AbilityName::_2) => fiddlesticks_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Fiddlesticks'"),
        },
        ChampionId::Fiora => match kind {
            AbilityId::Q(AbilityName::_1) => fiora_q_1(ctx),
            AbilityId::W(AbilityName::_1) => fiora_w_1(ctx),
            AbilityId::E(AbilityName::_1) => fiora_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Fiora'"),
        },
        ChampionId::Fizz => match kind {
            AbilityId::Q(AbilityName::_1) => fizz_q_1(ctx),
            AbilityId::W(AbilityName::_1) => fizz_w_1(ctx),
            AbilityId::W(AbilityName::_2) => fizz_w_2(ctx),
            AbilityId::W(AbilityName::_3) => fizz_w_3(ctx),
            AbilityId::W(AbilityName::_4) => fizz_w_4(ctx),
            AbilityId::E(AbilityName::_1) => fizz_e_1(ctx),
            AbilityId::R(AbilityName::_1) => fizz_r_1(ctx),
            AbilityId::R(AbilityName::_2) => fizz_r_2(ctx),
            AbilityId::R(AbilityName::_3) => fizz_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Fizz'"),
        },
        ChampionId::Galio => match kind {
            AbilityId::Q(AbilityName::_1) => galio_q_1(ctx),
            AbilityId::W(AbilityName::_1) => galio_w_1(ctx),
            AbilityId::W(AbilityName::_2) => galio_w_2(ctx),
            AbilityId::W(AbilityName::_3) => galio_w_3(ctx),
            AbilityId::W(AbilityName::_4) => galio_w_4(ctx),
            AbilityId::E(AbilityName::_1) => galio_e_1(ctx),
            AbilityId::E(AbilityName::_2) => galio_e_2(ctx),
            AbilityId::R(AbilityName::_1) => galio_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Galio'"),
        },
        ChampionId::Gangplank => match kind {
            AbilityId::Q(AbilityName::_1) => gangplank_q_1(ctx),
            AbilityId::E(AbilityName::_1) => gangplank_e_1(ctx),
            AbilityId::R(AbilityName::_1) => gangplank_r_1(ctx),
            AbilityId::R(AbilityName::_2) => gangplank_r_2(ctx),
            AbilityId::R(AbilityName::_3) => gangplank_r_3(ctx),
            AbilityId::R(AbilityName::_4) => gangplank_r_4(ctx),
            AbilityId::R(AbilityName::_5) => gangplank_r_5(ctx),
            AbilityId::R(AbilityName::_6) => gangplank_r_6(ctx),
            AbilityId::R(AbilityName::_7) => gangplank_r_7(ctx),
            _ => panic!("Invalid AbilityId for 'Gangplank'"),
        },
        ChampionId::Garen => match kind {
            AbilityId::Q(AbilityName::_1) => garen_q_1(ctx),
            AbilityId::W(AbilityName::_1) => garen_w_1(ctx),
            AbilityId::E(AbilityName::_1) => garen_e_1(ctx),
            AbilityId::E(AbilityName::_2) => garen_e_2(ctx),
            AbilityId::R(AbilityName::_1) => garen_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Garen'"),
        },
        ChampionId::Gnar => match kind {
            AbilityId::Q(AbilityName::_2) => gnar_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => gnar_q_3(ctx),
            AbilityId::W(AbilityName::_1) => gnar_w_1(ctx),
            AbilityId::E(AbilityName::_1) => gnar_e_1(ctx),
            AbilityId::E(AbilityName::_2) => gnar_e_2(ctx),
            AbilityId::R(AbilityName::_1) => gnar_r_1(ctx),
            AbilityId::R(AbilityName::_2) => gnar_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Gnar'"),
        },
        ChampionId::Gragas => match kind {
            AbilityId::Q(AbilityName::_1) => gragas_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => gragas_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => gragas_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => gragas_q_4(ctx),
            AbilityId::W(AbilityName::_1) => gragas_w_1(ctx),
            AbilityId::W(AbilityName::_2) => gragas_w_2(ctx),
            AbilityId::W(AbilityName::_3) => gragas_w_3(ctx),
            AbilityId::E(AbilityName::_1) => gragas_e_1(ctx),
            AbilityId::R(AbilityName::_1) => gragas_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Gragas'"),
        },
        ChampionId::Graves => match kind {
            AbilityId::Q(AbilityName::_1) => graves_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => graves_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => graves_q_3(ctx),
            AbilityId::W(AbilityName::_1) => graves_w_1(ctx),
            AbilityId::R(AbilityName::_1) => graves_r_1(ctx),
            AbilityId::R(AbilityName::_2) => graves_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Graves'"),
        },
        ChampionId::Gwen => match kind {
            AbilityId::Q(AbilityName::_1) => gwen_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => gwen_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => gwen_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => gwen_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => gwen_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => gwen_q_6(ctx),
            AbilityId::Q(AbilityName::_7) => gwen_q_7(ctx),
            AbilityId::Q(AbilityName::_8) => gwen_q_8(ctx),
            AbilityId::E(AbilityName::_1) => gwen_e_1(ctx),
            AbilityId::R(AbilityName::_1) => gwen_r_1(ctx),
            AbilityId::R(AbilityName::_2) => gwen_r_2(ctx),
            AbilityId::R(AbilityName::_3) => gwen_r_3(ctx),
            AbilityId::R(AbilityName::_4) => gwen_r_4(ctx),
            AbilityId::R(AbilityName::_5) => gwen_r_5(ctx),
            _ => panic!("Invalid AbilityId for 'Gwen'"),
        },
        ChampionId::Hecarim => match kind {
            AbilityId::Q(AbilityName::_1) => hecarim_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => hecarim_q_2(ctx),
            AbilityId::W(AbilityName::_1) => hecarim_w_1(ctx),
            AbilityId::W(AbilityName::_2) => hecarim_w_2(ctx),
            AbilityId::E(AbilityName::_1) => hecarim_e_1(ctx),
            AbilityId::E(AbilityName::_2) => hecarim_e_2(ctx),
            AbilityId::R(AbilityName::_1) => hecarim_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Hecarim'"),
        },
        ChampionId::Heimerdinger => match kind {
            AbilityId::W(AbilityName::_2) => heimerdinger_w_2(ctx),
            AbilityId::W(AbilityName::_3) => heimerdinger_w_3(ctx),
            AbilityId::W(AbilityName::_4) => heimerdinger_w_4(ctx),
            AbilityId::W(AbilityName::_5) => heimerdinger_w_5(ctx),
            AbilityId::W(AbilityName::_6) => heimerdinger_w_6(ctx),
            AbilityId::W(AbilityName::_7) => heimerdinger_w_7(ctx),
            AbilityId::W(AbilityName::_8) => heimerdinger_w_8(ctx),
            AbilityId::E(AbilityName::_1) => heimerdinger_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Heimerdinger'"),
        },
        ChampionId::Hwei => match kind {
            AbilityId::R(AbilityName::_1) => hwei_r_1(ctx),
            AbilityId::R(AbilityName::_2) => hwei_r_2(ctx),
            AbilityId::R(AbilityName::_3) => hwei_r_3(ctx),
            AbilityId::R(AbilityName::_4) => hwei_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Hwei'"),
        },
        ChampionId::Illaoi => match kind {
            AbilityId::Q(AbilityName::_1) => illaoi_q_1(ctx),
            AbilityId::W(AbilityName::_1) => illaoi_w_1(ctx),
            AbilityId::W(AbilityName::_2) => illaoi_w_2(ctx),
            AbilityId::E(AbilityName::_1) => illaoi_e_1(ctx),
            AbilityId::R(AbilityName::_1) => illaoi_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Illaoi'"),
        },
        ChampionId::Irelia => match kind {
            AbilityId::Q(AbilityName::_1) => irelia_q_1(ctx),
            AbilityId::W(AbilityName::_1) => irelia_w_1(ctx),
            AbilityId::W(AbilityName::_2) => irelia_w_2(ctx),
            AbilityId::E(AbilityName::_1) => irelia_e_1(ctx),
            AbilityId::R(AbilityName::_1) => irelia_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Irelia'"),
        },
        ChampionId::Ivern => match kind {
            AbilityId::Q(AbilityName::_1) => ivern_q_1(ctx),
            AbilityId::W(AbilityName::_1) => ivern_w_1(ctx),
            AbilityId::W(AbilityName::_2) => ivern_w_2(ctx),
            AbilityId::E(AbilityName::_1) => ivern_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Ivern'"),
        },
        ChampionId::Janna => match kind {
            AbilityId::Q(AbilityName::_1) => janna_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => janna_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => janna_q_3(ctx),
            AbilityId::W(AbilityName::_1) => janna_w_1(ctx),
            AbilityId::E(AbilityName::_1) => janna_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Janna'"),
        },
        ChampionId::JarvanIV => match kind {
            AbilityId::Q(AbilityName::_1) => jarvaniv_q_1(ctx),
            AbilityId::E(AbilityName::_1) => jarvaniv_e_1(ctx),
            AbilityId::R(AbilityName::_1) => jarvaniv_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'JarvanIV'"),
        },
        ChampionId::Jax => match kind {
            AbilityId::Q(AbilityName::_1) => jax_q_1(ctx),
            AbilityId::W(AbilityName::_1) => jax_w_1(ctx),
            AbilityId::E(AbilityName::_1) => jax_e_1(ctx),
            AbilityId::E(AbilityName::_2) => jax_e_2(ctx),
            AbilityId::R(AbilityName::_1) => jax_r_1(ctx),
            AbilityId::R(AbilityName::_2) => jax_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Jax'"),
        },
        ChampionId::Jayce => match kind {
            AbilityId::Q(AbilityName::_2) => jayce_q_2(ctx),
            AbilityId::W(AbilityName::_2) => jayce_w_2(ctx),
            AbilityId::W(AbilityName::_3) => jayce_w_3(ctx),
            AbilityId::E(AbilityName::_1) => jayce_e_1(ctx),
            AbilityId::E(AbilityName::_2) => jayce_e_2(ctx),
            _ => panic!("Invalid AbilityId for 'Jayce'"),
        },
        ChampionId::Jhin => match kind {
            AbilityId::Q(AbilityName::_1) => jhin_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => jhin_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => jhin_q_3(ctx),
            AbilityId::W(AbilityName::_1) => jhin_w_1(ctx),
            AbilityId::W(AbilityName::_2) => jhin_w_2(ctx),
            AbilityId::E(AbilityName::_1) => jhin_e_1(ctx),
            AbilityId::E(AbilityName::_2) => jhin_e_2(ctx),
            AbilityId::R(AbilityName::_1) => jhin_r_1(ctx),
            AbilityId::R(AbilityName::_2) => jhin_r_2(ctx),
            AbilityId::R(AbilityName::_3) => jhin_r_3(ctx),
            AbilityId::R(AbilityName::_4) => jhin_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Jhin'"),
        },
        ChampionId::Jinx => match kind {
            AbilityId::W(AbilityName::_1) => jinx_w_1(ctx),
            AbilityId::E(AbilityName::_1) => jinx_e_1(ctx),
            AbilityId::R(AbilityName::_1) => jinx_r_1(ctx),
            AbilityId::R(AbilityName::_2) => jinx_r_2(ctx),
            AbilityId::R(AbilityName::_3) => jinx_r_3(ctx),
            AbilityId::R(AbilityName::_4) => jinx_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Jinx'"),
        },
        ChampionId::KSante => match kind {
            AbilityId::Q(AbilityName::_1) => ksante_q_1(ctx),
            AbilityId::W(AbilityName::_1) => ksante_w_1(ctx),
            AbilityId::W(AbilityName::_2) => ksante_w_2(ctx),
            AbilityId::W(AbilityName::_3) => ksante_w_3(ctx),
            AbilityId::W(AbilityName::_4) => ksante_w_4(ctx),
            AbilityId::W(AbilityName::_5) => ksante_w_5(ctx),
            AbilityId::R(AbilityName::_1) => ksante_r_1(ctx),
            AbilityId::R(AbilityName::_2) => ksante_r_2(ctx),
            AbilityId::R(AbilityName::_3) => ksante_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'KSante'"),
        },
        ChampionId::Kaisa => match kind {
            AbilityId::Q(AbilityName::_1) => kaisa_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => kaisa_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => kaisa_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => kaisa_q_4(ctx),
            AbilityId::W(AbilityName::_1) => kaisa_w_1(ctx),
            _ => panic!("Invalid AbilityId for 'Kaisa'"),
        },
        ChampionId::Kalista => match kind {
            AbilityId::Q(AbilityName::_1) => kalista_q_1(ctx),
            AbilityId::W(AbilityName::_1) => kalista_w_1(ctx),
            AbilityId::W(AbilityName::_2) => kalista_w_2(ctx),
            AbilityId::E(AbilityName::_1) => kalista_e_1(ctx),
            AbilityId::E(AbilityName::_2) => kalista_e_2(ctx),
            _ => panic!("Invalid AbilityId for 'Kalista'"),
        },
        ChampionId::Karma => match kind {
            AbilityId::Q(AbilityName::_2) => karma_q_2(ctx),
            AbilityId::W(AbilityName::_1) => karma_w_1(ctx),
            AbilityId::W(AbilityName::_2) => karma_w_2(ctx),
            _ => panic!("Invalid AbilityId for 'Karma'"),
        },
        ChampionId::Karthus => match kind {
            AbilityId::Q(AbilityName::_1) => karthus_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => karthus_q_2(ctx),
            AbilityId::E(AbilityName::_1) => karthus_e_1(ctx),
            AbilityId::E(AbilityName::_2) => karthus_e_2(ctx),
            AbilityId::R(AbilityName::_1) => karthus_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Karthus'"),
        },
        ChampionId::Kassadin => match kind {
            AbilityId::Q(AbilityName::_1) => kassadin_q_1(ctx),
            AbilityId::W(AbilityName::_1) => kassadin_w_1(ctx),
            AbilityId::E(AbilityName::_1) => kassadin_e_1(ctx),
            AbilityId::R(AbilityName::_1) => kassadin_r_1(ctx),
            AbilityId::R(AbilityName::_2) => kassadin_r_2(ctx),
            AbilityId::R(AbilityName::_3) => kassadin_r_3(ctx),
            AbilityId::R(AbilityName::_4) => kassadin_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Kassadin'"),
        },
        ChampionId::Katarina => match kind {
            AbilityId::Q(AbilityName::_1) => katarina_q_1(ctx),
            AbilityId::E(AbilityName::_1) => katarina_e_1(ctx),
            AbilityId::R(AbilityName::_1) => katarina_r_1(ctx),
            AbilityId::R(AbilityName::_2) => katarina_r_2(ctx),
            AbilityId::R(AbilityName::_3) => katarina_r_3(ctx),
            AbilityId::R(AbilityName::_4) => katarina_r_4(ctx),
            AbilityId::R(AbilityName::_5) => katarina_r_5(ctx),
            _ => panic!("Invalid AbilityId for 'Katarina'"),
        },
        ChampionId::Kayle => match kind {
            AbilityId::Q(AbilityName::_1) => kayle_q_1(ctx),
            AbilityId::E(AbilityName::_1) => kayle_e_1(ctx),
            AbilityId::E(AbilityName::_2) => kayle_e_2(ctx),
            AbilityId::R(AbilityName::_1) => kayle_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Kayle'"),
        },
        ChampionId::Kayn => match kind {
            AbilityId::Q(AbilityName::_1) => kayn_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => kayn_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => kayn_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => kayn_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => kayn_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => kayn_q_6(ctx),
            AbilityId::W(AbilityName::_1) => kayn_w_1(ctx),
            AbilityId::R(AbilityName::_1) => kayn_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Kayn'"),
        },
        ChampionId::Kennen => match kind {
            AbilityId::Q(AbilityName::_1) => kennen_q_1(ctx),
            AbilityId::W(AbilityName::_1) => kennen_w_1(ctx),
            AbilityId::W(AbilityName::_2) => kennen_w_2(ctx),
            AbilityId::E(AbilityName::_1) => kennen_e_1(ctx),
            AbilityId::E(AbilityName::_2) => kennen_e_2(ctx),
            AbilityId::R(AbilityName::_1) => kennen_r_1(ctx),
            AbilityId::R(AbilityName::_2) => kennen_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Kennen'"),
        },
        ChampionId::Khazix => match kind {
            AbilityId::Q(AbilityName::_1) => khazix_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => khazix_q_2(ctx),
            AbilityId::W(AbilityName::_1) => khazix_w_1(ctx),
            AbilityId::E(AbilityName::_1) => khazix_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Khazix'"),
        },
        ChampionId::Kindred => match kind {
            AbilityId::Q(AbilityName::_1) => kindred_q_1(ctx),
            AbilityId::W(AbilityName::_1) => kindred_w_1(ctx),
            AbilityId::W(AbilityName::_2) => kindred_w_2(ctx),
            AbilityId::E(AbilityName::_1) => kindred_e_1(ctx),
            AbilityId::E(AbilityName::_2) => kindred_e_2(ctx),
            _ => panic!("Invalid AbilityId for 'Kindred'"),
        },
        ChampionId::Kled => match kind {
            AbilityId::Q(AbilityName::_2) => kled_q_2(ctx),
            AbilityId::Q(AbilityName::_4) => kled_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => kled_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => kled_q_6(ctx),
            AbilityId::Q(AbilityName::_7) => kled_q_7(ctx),
            AbilityId::W(AbilityName::_1) => kled_w_1(ctx),
            AbilityId::W(AbilityName::_2) => kled_w_2(ctx),
            AbilityId::E(AbilityName::_1) => kled_e_1(ctx),
            AbilityId::E(AbilityName::_2) => kled_e_2(ctx),
            AbilityId::R(AbilityName::_1) => kled_r_1(ctx),
            AbilityId::R(AbilityName::_2) => kled_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Kled'"),
        },
        ChampionId::KogMaw => match kind {
            AbilityId::Q(AbilityName::_1) => kogmaw_q_1(ctx),
            AbilityId::W(AbilityName::_1) => kogmaw_w_1(ctx),
            AbilityId::E(AbilityName::_1) => kogmaw_e_1(ctx),
            AbilityId::R(AbilityName::_1) => kogmaw_r_1(ctx),
            AbilityId::R(AbilityName::_2) => kogmaw_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'KogMaw'"),
        },
        ChampionId::Leblanc => match kind {
            AbilityId::Q(AbilityName::_1) => leblanc_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => leblanc_q_2(ctx),
            AbilityId::W(AbilityName::_1) => leblanc_w_1(ctx),
            AbilityId::E(AbilityName::_1) => leblanc_e_1(ctx),
            AbilityId::E(AbilityName::_2) => leblanc_e_2(ctx),
            AbilityId::E(AbilityName::_3) => leblanc_e_3(ctx),
            AbilityId::R(AbilityName::_1) => leblanc_r_1(ctx),
            AbilityId::R(AbilityName::_2) => leblanc_r_2(ctx),
            AbilityId::R(AbilityName::_3) => leblanc_r_3(ctx),
            AbilityId::R(AbilityName::_4) => leblanc_r_4(ctx),
            AbilityId::R(AbilityName::_5) => leblanc_r_5(ctx),
            AbilityId::R(AbilityName::_6) => leblanc_r_6(ctx),
            AbilityId::R(AbilityName::_7) => leblanc_r_7(ctx),
            _ => panic!("Invalid AbilityId for 'Leblanc'"),
        },
        ChampionId::LeeSin => match kind {
            AbilityId::Q(AbilityName::_2) => leesin_q_2(ctx),
            AbilityId::E(AbilityName::_1) => leesin_e_1(ctx),
            AbilityId::R(AbilityName::_1) => leesin_r_1(ctx),
            AbilityId::R(AbilityName::_2) => leesin_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'LeeSin'"),
        },
        ChampionId::Leona => match kind {
            AbilityId::Q(AbilityName::_1) => leona_q_1(ctx),
            AbilityId::W(AbilityName::_1) => leona_w_1(ctx),
            AbilityId::W(AbilityName::_2) => leona_w_2(ctx),
            AbilityId::E(AbilityName::_1) => leona_e_1(ctx),
            AbilityId::R(AbilityName::_1) => leona_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Leona'"),
        },
        ChampionId::Lillia => match kind {
            AbilityId::Q(AbilityName::_1) => lillia_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => lillia_q_2(ctx),
            AbilityId::W(AbilityName::_1) => lillia_w_1(ctx),
            AbilityId::W(AbilityName::_2) => lillia_w_2(ctx),
            AbilityId::W(AbilityName::_3) => lillia_w_3(ctx),
            AbilityId::W(AbilityName::_4) => lillia_w_4(ctx),
            AbilityId::E(AbilityName::_1) => lillia_e_1(ctx),
            AbilityId::R(AbilityName::_1) => lillia_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Lillia'"),
        },
        ChampionId::Lissandra => match kind {
            AbilityId::Q(AbilityName::_1) => lissandra_q_1(ctx),
            AbilityId::W(AbilityName::_1) => lissandra_w_1(ctx),
            AbilityId::E(AbilityName::_1) => lissandra_e_1(ctx),
            AbilityId::R(AbilityName::_1) => lissandra_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Lissandra'"),
        },
        ChampionId::Lucian => match kind {
            AbilityId::Q(AbilityName::_1) => lucian_q_1(ctx),
            AbilityId::W(AbilityName::_1) => lucian_w_1(ctx),
            AbilityId::R(AbilityName::_1) => lucian_r_1(ctx),
            AbilityId::R(AbilityName::_2) => lucian_r_2(ctx),
            AbilityId::R(AbilityName::_3) => lucian_r_3(ctx),
            AbilityId::R(AbilityName::_4) => lucian_r_4(ctx),
            AbilityId::R(AbilityName::_5) => lucian_r_5(ctx),
            AbilityId::R(AbilityName::_6) => lucian_r_6(ctx),
            _ => panic!("Invalid AbilityId for 'Lucian'"),
        },
        ChampionId::Lulu => match kind {
            AbilityId::Q(AbilityName::_1) => lulu_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => lulu_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => lulu_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => lulu_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => lulu_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => lulu_q_6(ctx),
            AbilityId::E(AbilityName::_1) => lulu_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Lulu'"),
        },
        ChampionId::Lux => match kind {
            AbilityId::Q(AbilityName::_1) => lux_q_1(ctx),
            AbilityId::E(AbilityName::_1) => lux_e_1(ctx),
            AbilityId::R(AbilityName::_1) => lux_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Lux'"),
        },
        ChampionId::Malphite => match kind {
            AbilityId::Q(AbilityName::_1) => malphite_q_1(ctx),
            AbilityId::W(AbilityName::_1) => malphite_w_1(ctx),
            AbilityId::W(AbilityName::_2) => malphite_w_2(ctx),
            AbilityId::E(AbilityName::_1) => malphite_e_1(ctx),
            AbilityId::R(AbilityName::_1) => malphite_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Malphite'"),
        },
        ChampionId::Malzahar => match kind {
            AbilityId::Q(AbilityName::_1) => malzahar_q_1(ctx),
            AbilityId::W(AbilityName::_1) => malzahar_w_1(ctx),
            AbilityId::W(AbilityName::_2) => malzahar_w_2(ctx),
            AbilityId::E(AbilityName::_1) => malzahar_e_1(ctx),
            AbilityId::E(AbilityName::_2) => malzahar_e_2(ctx),
            AbilityId::R(AbilityName::_1) => malzahar_r_1(ctx),
            AbilityId::R(AbilityName::_2) => malzahar_r_2(ctx),
            AbilityId::R(AbilityName::_3) => malzahar_r_3(ctx),
            AbilityId::R(AbilityName::_4) => malzahar_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Malzahar'"),
        },
        ChampionId::Maokai => match kind {
            AbilityId::Q(AbilityName::_1) => maokai_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => maokai_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => maokai_q_3(ctx),
            AbilityId::W(AbilityName::_1) => maokai_w_1(ctx),
            AbilityId::E(AbilityName::_1) => maokai_e_1(ctx),
            AbilityId::E(AbilityName::_2) => maokai_e_2(ctx),
            AbilityId::E(AbilityName::_3) => maokai_e_3(ctx),
            AbilityId::R(AbilityName::_1) => maokai_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Maokai'"),
        },
        ChampionId::MasterYi => match kind {
            AbilityId::Q(AbilityName::_1) => masteryi_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => masteryi_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => masteryi_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => masteryi_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => masteryi_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => masteryi_q_6(ctx),
            AbilityId::Q(AbilityName::_7) => masteryi_q_7(ctx),
            AbilityId::W(AbilityName::_1) => masteryi_w_1(ctx),
            AbilityId::W(AbilityName::_2) => masteryi_w_2(ctx),
            AbilityId::E(AbilityName::_1) => masteryi_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'MasterYi'"),
        },
        ChampionId::Mel => match kind {
            AbilityId::Q(AbilityName::_1) => mel_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => mel_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => mel_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => mel_q_4(ctx),
            AbilityId::W(AbilityName::_1) => mel_w_1(ctx),
            AbilityId::E(AbilityName::_1) => mel_e_1(ctx),
            AbilityId::E(AbilityName::_2) => mel_e_2(ctx),
            AbilityId::E(AbilityName::_3) => mel_e_3(ctx),
            AbilityId::E(AbilityName::_4) => mel_e_4(ctx),
            AbilityId::E(AbilityName::_5) => mel_e_5(ctx),
            AbilityId::E(AbilityName::_6) => mel_e_6(ctx),
            AbilityId::R(AbilityName::_1) => mel_r_1(ctx),
            AbilityId::R(AbilityName::_2) => mel_r_2(ctx),
            AbilityId::R(AbilityName::_3) => mel_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Mel'"),
        },
        ChampionId::Milio => match kind {
            AbilityId::Q(AbilityName::_1) => milio_q_1(ctx),
            _ => panic!("Invalid AbilityId for 'Milio'"),
        },
        ChampionId::MissFortune => match kind {
            AbilityId::Q(AbilityName::_1) => missfortune_q_1(ctx),
            AbilityId::E(AbilityName::_1) => missfortune_e_1(ctx),
            AbilityId::E(AbilityName::_2) => missfortune_e_2(ctx),
            AbilityId::R(AbilityName::_1) => missfortune_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'MissFortune'"),
        },
        ChampionId::MonkeyKing => match kind {
            AbilityId::Q(AbilityName::_1) => monkeyking_q_1(ctx),
            AbilityId::W(AbilityName::_1) => monkeyking_w_1(ctx),
            AbilityId::E(AbilityName::_1) => monkeyking_e_1(ctx),
            AbilityId::R(AbilityName::_1) => monkeyking_r_1(ctx),
            AbilityId::R(AbilityName::_2) => monkeyking_r_2(ctx),
            AbilityId::R(AbilityName::_3) => monkeyking_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'MonkeyKing'"),
        },
        ChampionId::Mordekaiser => match kind {
            AbilityId::Q(AbilityName::_1) => mordekaiser_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => mordekaiser_q_2(ctx),
            AbilityId::E(AbilityName::_1) => mordekaiser_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Mordekaiser'"),
        },
        ChampionId::Morgana => match kind {
            AbilityId::Q(AbilityName::_1) => morgana_q_1(ctx),
            AbilityId::W(AbilityName::_1) => morgana_w_1(ctx),
            AbilityId::W(AbilityName::_2) => morgana_w_2(ctx),
            AbilityId::W(AbilityName::_3) => morgana_w_3(ctx),
            AbilityId::W(AbilityName::_4) => morgana_w_4(ctx),
            AbilityId::R(AbilityName::_1) => morgana_r_1(ctx),
            AbilityId::R(AbilityName::_2) => morgana_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Morgana'"),
        },
        ChampionId::Naafiri => match kind {
            AbilityId::Q(AbilityName::_1) => naafiri_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => naafiri_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => naafiri_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => naafiri_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => naafiri_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => naafiri_q_6(ctx),
            AbilityId::Q(AbilityName::_7) => naafiri_q_7(ctx),
            AbilityId::Q(AbilityName::_8) => naafiri_q_8(ctx),
            AbilityId::E(AbilityName::_1) => naafiri_e_1(ctx),
            AbilityId::E(AbilityName::_2) => naafiri_e_2(ctx),
            AbilityId::E(AbilityName::_3) => naafiri_e_3(ctx),
            AbilityId::R(AbilityName::_1) => naafiri_r_1(ctx),
            AbilityId::R(AbilityName::_2) => naafiri_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Naafiri'"),
        },
        ChampionId::Nami => match kind {
            AbilityId::Q(AbilityName::_1) => nami_q_1(ctx),
            AbilityId::W(AbilityName::_1) => nami_w_1(ctx),
            AbilityId::W(AbilityName::_2) => nami_w_2(ctx),
            AbilityId::E(AbilityName::_1) => nami_e_1(ctx),
            AbilityId::E(AbilityName::_2) => nami_e_2(ctx),
            AbilityId::R(AbilityName::_1) => nami_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Nami'"),
        },
        ChampionId::Nasus => match kind {
            AbilityId::Q(AbilityName::_1) => nasus_q_1(ctx),
            AbilityId::E(AbilityName::_1) => nasus_e_1(ctx),
            AbilityId::E(AbilityName::_2) => nasus_e_2(ctx),
            AbilityId::E(AbilityName::_3) => nasus_e_3(ctx),
            AbilityId::R(AbilityName::_1) => nasus_r_1(ctx),
            AbilityId::R(AbilityName::_2) => nasus_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Nasus'"),
        },
        ChampionId::Nautilus => match kind {
            AbilityId::Q(AbilityName::_1) => nautilus_q_1(ctx),
            AbilityId::W(AbilityName::_1) => nautilus_w_1(ctx),
            AbilityId::E(AbilityName::_1) => nautilus_e_1(ctx),
            AbilityId::E(AbilityName::_2) => nautilus_e_2(ctx),
            AbilityId::E(AbilityName::_3) => nautilus_e_3(ctx),
            AbilityId::E(AbilityName::_4) => nautilus_e_4(ctx),
            AbilityId::E(AbilityName::_5) => nautilus_e_5(ctx),
            AbilityId::E(AbilityName::_6) => nautilus_e_6(ctx),
            AbilityId::R(AbilityName::_1) => nautilus_r_1(ctx),
            AbilityId::R(AbilityName::_2) => nautilus_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Nautilus'"),
        },
        ChampionId::Neeko => match kind {
            AbilityId::Q(AbilityName::_1) => neeko_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => neeko_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => neeko_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => neeko_q_4(ctx),
            AbilityId::W(AbilityName::_1) => neeko_w_1(ctx),
            AbilityId::E(AbilityName::_1) => neeko_e_1(ctx),
            AbilityId::R(AbilityName::_1) => neeko_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Neeko'"),
        },
        ChampionId::Nidalee => match kind {
            AbilityId::Q(AbilityName::_3) => nidalee_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => nidalee_q_4(ctx),
            AbilityId::W(AbilityName::_2) => nidalee_w_2(ctx),
            AbilityId::W(AbilityName::_3) => nidalee_w_3(ctx),
            AbilityId::E(AbilityName::_1) => nidalee_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Nidalee'"),
        },
        ChampionId::Nilah => match kind {
            AbilityId::Q(AbilityName::_1) => nilah_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => nilah_q_2(ctx),
            AbilityId::E(AbilityName::_1) => nilah_e_1(ctx),
            AbilityId::R(AbilityName::_1) => nilah_r_1(ctx),
            AbilityId::R(AbilityName::_2) => nilah_r_2(ctx),
            AbilityId::R(AbilityName::_3) => nilah_r_3(ctx),
            AbilityId::R(AbilityName::_4) => nilah_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Nilah'"),
        },
        ChampionId::Nocturne => match kind {
            AbilityId::Q(AbilityName::_1) => nocturne_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => nocturne_q_2(ctx),
            AbilityId::E(AbilityName::_1) => nocturne_e_1(ctx),
            AbilityId::E(AbilityName::_2) => nocturne_e_2(ctx),
            AbilityId::R(AbilityName::_1) => nocturne_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Nocturne'"),
        },
        ChampionId::Nunu => match kind {
            AbilityId::Q(AbilityName::_1) => nunu_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => nunu_q_2(ctx),
            AbilityId::W(AbilityName::_1) => nunu_w_1(ctx),
            AbilityId::W(AbilityName::_2) => nunu_w_2(ctx),
            AbilityId::W(AbilityName::_3) => nunu_w_3(ctx),
            AbilityId::W(AbilityName::_4) => nunu_w_4(ctx),
            AbilityId::E(AbilityName::_1) => nunu_e_1(ctx),
            AbilityId::E(AbilityName::_2) => nunu_e_2(ctx),
            AbilityId::E(AbilityName::_3) => nunu_e_3(ctx),
            AbilityId::E(AbilityName::_4) => nunu_e_4(ctx),
            AbilityId::R(AbilityName::_1) => nunu_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Nunu'"),
        },
        ChampionId::Olaf => match kind {
            AbilityId::Q(AbilityName::_1) => olaf_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => olaf_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => olaf_q_3(ctx),
            AbilityId::E(AbilityName::_1) => olaf_e_1(ctx),
            AbilityId::R(AbilityName::_1) => olaf_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Olaf'"),
        },
        ChampionId::Orianna => match kind {
            AbilityId::Q(AbilityName::_1) => orianna_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => orianna_q_2(ctx),
            AbilityId::W(AbilityName::_1) => orianna_w_1(ctx),
            AbilityId::E(AbilityName::_1) => orianna_e_1(ctx),
            AbilityId::R(AbilityName::_1) => orianna_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Orianna'"),
        },
        ChampionId::Ornn => match kind {
            AbilityId::Q(AbilityName::_1) => ornn_q_1(ctx),
            AbilityId::W(AbilityName::_1) => ornn_w_1(ctx),
            AbilityId::W(AbilityName::_2) => ornn_w_2(ctx),
            AbilityId::W(AbilityName::_3) => ornn_w_3(ctx),
            AbilityId::W(AbilityName::_4) => ornn_w_4(ctx),
            AbilityId::W(AbilityName::_5) => ornn_w_5(ctx),
            AbilityId::W(AbilityName::_6) => ornn_w_6(ctx),
            AbilityId::E(AbilityName::_1) => ornn_e_1(ctx),
            AbilityId::R(AbilityName::_1) => ornn_r_1(ctx),
            AbilityId::R(AbilityName::_2) => ornn_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Ornn'"),
        },
        ChampionId::Pantheon => match kind {
            AbilityId::Q(AbilityName::_1) => pantheon_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => pantheon_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => pantheon_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => pantheon_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => pantheon_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => pantheon_q_6(ctx),
            AbilityId::W(AbilityName::_1) => pantheon_w_1(ctx),
            AbilityId::E(AbilityName::_1) => pantheon_e_1(ctx),
            AbilityId::R(AbilityName::_1) => pantheon_r_1(ctx),
            AbilityId::R(AbilityName::_2) => pantheon_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Pantheon'"),
        },
        ChampionId::Poppy => match kind {
            AbilityId::Q(AbilityName::_1) => poppy_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => poppy_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => poppy_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => poppy_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => poppy_q_5(ctx),
            AbilityId::W(AbilityName::_1) => poppy_w_1(ctx),
            AbilityId::E(AbilityName::_1) => poppy_e_1(ctx),
            AbilityId::E(AbilityName::_2) => poppy_e_2(ctx),
            AbilityId::R(AbilityName::_1) => poppy_r_1(ctx),
            AbilityId::R(AbilityName::_2) => poppy_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Poppy'"),
        },
        ChampionId::Pyke => match kind {
            AbilityId::Q(AbilityName::_1) => pyke_q_1(ctx),
            AbilityId::E(AbilityName::_1) => pyke_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Pyke'"),
        },
        ChampionId::Qiyana => match kind {
            AbilityId::Q(AbilityName::_1) => qiyana_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => qiyana_q_2(ctx),
            AbilityId::W(AbilityName::_1) => qiyana_w_1(ctx),
            AbilityId::E(AbilityName::_1) => qiyana_e_1(ctx),
            AbilityId::R(AbilityName::_1) => qiyana_r_1(ctx),
            AbilityId::R(AbilityName::_2) => qiyana_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Qiyana'"),
        },
        ChampionId::Quinn => match kind {
            AbilityId::Q(AbilityName::_1) => quinn_q_1(ctx),
            AbilityId::E(AbilityName::_1) => quinn_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Quinn'"),
        },
        ChampionId::Rakan => match kind {
            AbilityId::Q(AbilityName::_1) => rakan_q_1(ctx),
            AbilityId::W(AbilityName::_1) => rakan_w_1(ctx),
            AbilityId::R(AbilityName::_1) => rakan_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Rakan'"),
        },
        ChampionId::Rammus => match kind {
            AbilityId::Q(AbilityName::_1) => rammus_q_1(ctx),
            AbilityId::E(AbilityName::_1) => rammus_e_1(ctx),
            AbilityId::R(AbilityName::_1) => rammus_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Rammus'"),
        },
        ChampionId::RekSai => match kind {
            AbilityId::Q(AbilityName::_2) => reksai_q_2(ctx),
            AbilityId::W(AbilityName::_1) => reksai_w_1(ctx),
            AbilityId::E(AbilityName::_1) => reksai_e_1(ctx),
            AbilityId::E(AbilityName::_2) => reksai_e_2(ctx),
            AbilityId::R(AbilityName::_1) => reksai_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'RekSai'"),
        },
        ChampionId::Rell => match kind {
            AbilityId::Q(AbilityName::_1) => rell_q_1(ctx),
            AbilityId::W(AbilityName::_2) => rell_w_2(ctx),
            AbilityId::E(AbilityName::_1) => rell_e_1(ctx),
            AbilityId::R(AbilityName::_1) => rell_r_1(ctx),
            AbilityId::R(AbilityName::_2) => rell_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Rell'"),
        },
        ChampionId::Renata => match kind {
            AbilityId::Q(AbilityName::_1) => renata_q_1(ctx),
            AbilityId::E(AbilityName::_1) => renata_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Renata'"),
        },
        ChampionId::Renekton => match kind {
            AbilityId::Q(AbilityName::_1) => renekton_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => renekton_q_2(ctx),
            AbilityId::W(AbilityName::_1) => renekton_w_1(ctx),
            AbilityId::W(AbilityName::_2) => renekton_w_2(ctx),
            AbilityId::W(AbilityName::_3) => renekton_w_3(ctx),
            AbilityId::E(AbilityName::_2) => renekton_e_2(ctx),
            AbilityId::R(AbilityName::_1) => renekton_r_1(ctx),
            AbilityId::R(AbilityName::_2) => renekton_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Renekton'"),
        },
        ChampionId::Rengar => match kind {
            AbilityId::Q(AbilityName::_1) => rengar_q_1(ctx),
            AbilityId::W(AbilityName::_1) => rengar_w_1(ctx),
            AbilityId::E(AbilityName::_1) => rengar_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Rengar'"),
        },
        ChampionId::Riven => match kind {
            AbilityId::Q(AbilityName::_1) => riven_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => riven_q_2(ctx),
            AbilityId::W(AbilityName::_1) => riven_w_1(ctx),
            _ => panic!("Invalid AbilityId for 'Riven'"),
        },
        ChampionId::Rumble => match kind {
            AbilityId::Q(AbilityName::_1) => rumble_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => rumble_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => rumble_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => rumble_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => rumble_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => rumble_q_6(ctx),
            AbilityId::Q(AbilityName::_7) => rumble_q_7(ctx),
            AbilityId::Q(AbilityName::_8) => rumble_q_8(ctx),
            AbilityId::E(AbilityName::_1) => rumble_e_1(ctx),
            AbilityId::E(AbilityName::_2) => rumble_e_2(ctx),
            AbilityId::E(AbilityName::_3) => rumble_e_3(ctx),
            AbilityId::E(AbilityName::_4) => rumble_e_4(ctx),
            AbilityId::R(AbilityName::_1) => rumble_r_1(ctx),
            AbilityId::R(AbilityName::_2) => rumble_r_2(ctx),
            AbilityId::R(AbilityName::_3) => rumble_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Rumble'"),
        },
        ChampionId::Ryze => match kind {
            AbilityId::Q(AbilityName::_1) => ryze_q_1(ctx),
            AbilityId::W(AbilityName::_1) => ryze_w_1(ctx),
            AbilityId::E(AbilityName::_1) => ryze_e_1(ctx),
            AbilityId::R(AbilityName::_1) => ryze_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Ryze'"),
        },
        ChampionId::Samira => match kind {
            AbilityId::Q(AbilityName::_1) => samira_q_1(ctx),
            AbilityId::W(AbilityName::_1) => samira_w_1(ctx),
            AbilityId::W(AbilityName::_2) => samira_w_2(ctx),
            AbilityId::E(AbilityName::_1) => samira_e_1(ctx),
            AbilityId::R(AbilityName::_1) => samira_r_1(ctx),
            AbilityId::R(AbilityName::_2) => samira_r_2(ctx),
            AbilityId::R(AbilityName::_3) => samira_r_3(ctx),
            AbilityId::R(AbilityName::_4) => samira_r_4(ctx),
            _ => panic!("Invalid AbilityId for 'Samira'"),
        },
        ChampionId::Sejuani => match kind {
            AbilityId::Q(AbilityName::_1) => sejuani_q_1(ctx),
            AbilityId::W(AbilityName::_1) => sejuani_w_1(ctx),
            AbilityId::W(AbilityName::_2) => sejuani_w_2(ctx),
            AbilityId::W(AbilityName::_3) => sejuani_w_3(ctx),
            AbilityId::E(AbilityName::_1) => sejuani_e_1(ctx),
            AbilityId::R(AbilityName::_1) => sejuani_r_1(ctx),
            AbilityId::R(AbilityName::_2) => sejuani_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Sejuani'"),
        },
        ChampionId::Senna => match kind {
            AbilityId::Q(AbilityName::_1) => senna_q_1(ctx),
            AbilityId::W(AbilityName::_1) => senna_w_1(ctx),
            AbilityId::R(AbilityName::_1) => senna_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Senna'"),
        },
        ChampionId::Seraphine => match kind {
            AbilityId::Q(AbilityName::_1) => seraphine_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => seraphine_q_2(ctx),
            AbilityId::E(AbilityName::_1) => seraphine_e_1(ctx),
            AbilityId::E(AbilityName::_2) => seraphine_e_2(ctx),
            AbilityId::R(AbilityName::_1) => seraphine_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Seraphine'"),
        },
        ChampionId::Sett => match kind {
            AbilityId::Q(AbilityName::_1) => sett_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => sett_q_2(ctx),
            AbilityId::W(AbilityName::_1) => sett_w_1(ctx),
            AbilityId::E(AbilityName::_1) => sett_e_1(ctx),
            AbilityId::E(AbilityName::_2) => sett_e_2(ctx),
            AbilityId::R(AbilityName::_1) => sett_r_1(ctx),
            AbilityId::R(AbilityName::_2) => sett_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Sett'"),
        },
        ChampionId::Shaco => match kind {
            AbilityId::Q(AbilityName::_1) => shaco_q_1(ctx),
            AbilityId::W(AbilityName::_1) => shaco_w_1(ctx),
            AbilityId::W(AbilityName::_2) => shaco_w_2(ctx),
            AbilityId::W(AbilityName::_3) => shaco_w_3(ctx),
            AbilityId::W(AbilityName::_4) => shaco_w_4(ctx),
            AbilityId::W(AbilityName::_5) => shaco_w_5(ctx),
            AbilityId::E(AbilityName::_1) => shaco_e_1(ctx),
            AbilityId::E(AbilityName::_2) => shaco_e_2(ctx),
            AbilityId::R(AbilityName::_1) => shaco_r_1(ctx),
            AbilityId::R(AbilityName::_2) => shaco_r_2(ctx),
            AbilityId::R(AbilityName::_3) => shaco_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Shaco'"),
        },
        ChampionId::Shen => match kind {
            AbilityId::Q(AbilityName::_1) => shen_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => shen_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => shen_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => shen_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => shen_q_5(ctx),
            AbilityId::E(AbilityName::_1) => shen_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Shen'"),
        },
        ChampionId::Shyvana => match kind {
            AbilityId::Q(AbilityName::_1) => shyvana_q_1(ctx),
            AbilityId::W(AbilityName::_1) => shyvana_w_1(ctx),
            AbilityId::W(AbilityName::_2) => shyvana_w_2(ctx),
            AbilityId::E(AbilityName::_1) => shyvana_e_1(ctx),
            AbilityId::E(AbilityName::_2) => shyvana_e_2(ctx),
            AbilityId::R(AbilityName::_1) => shyvana_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Shyvana'"),
        },
        ChampionId::Singed => match kind {
            AbilityId::Q(AbilityName::_1) => singed_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => singed_q_2(ctx),
            AbilityId::E(AbilityName::_1) => singed_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Singed'"),
        },
        ChampionId::Sion => match kind {
            AbilityId::Q(AbilityName::_1) => sion_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => sion_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => sion_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => sion_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => sion_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => sion_q_6(ctx),
            AbilityId::Q(AbilityName::_7) => sion_q_7(ctx),
            AbilityId::W(AbilityName::_1) => sion_w_1(ctx),
            AbilityId::E(AbilityName::_1) => sion_e_1(ctx),
            AbilityId::R(AbilityName::_1) => sion_r_1(ctx),
            AbilityId::R(AbilityName::_2) => sion_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Sion'"),
        },
        ChampionId::Sivir => match kind {
            AbilityId::Q(AbilityName::_1) => sivir_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => sivir_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => sivir_q_3(ctx),
            AbilityId::W(AbilityName::_1) => sivir_w_1(ctx),
            AbilityId::W(AbilityName::_2) => sivir_w_2(ctx),
            AbilityId::W(AbilityName::_3) => sivir_w_3(ctx),
            AbilityId::W(AbilityName::_4) => sivir_w_4(ctx),
            _ => panic!("Invalid AbilityId for 'Sivir'"),
        },
        ChampionId::Skarner => match kind {
            AbilityId::Q(AbilityName::_2) => skarner_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => skarner_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => skarner_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => skarner_q_5(ctx),
            AbilityId::W(AbilityName::_1) => skarner_w_1(ctx),
            AbilityId::E(AbilityName::_1) => skarner_e_1(ctx),
            AbilityId::R(AbilityName::_1) => skarner_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Skarner'"),
        },
        ChampionId::Smolder => match kind {
            AbilityId::Q(AbilityName::_1) => smolder_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => smolder_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => smolder_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => smolder_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => smolder_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => smolder_q_6(ctx),
            AbilityId::W(AbilityName::_1) => smolder_w_1(ctx),
            AbilityId::W(AbilityName::_2) => smolder_w_2(ctx),
            AbilityId::W(AbilityName::_3) => smolder_w_3(ctx),
            AbilityId::E(AbilityName::_1) => smolder_e_1(ctx),
            AbilityId::E(AbilityName::_2) => smolder_e_2(ctx),
            AbilityId::R(AbilityName::_1) => smolder_r_1(ctx),
            AbilityId::R(AbilityName::_2) => smolder_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Smolder'"),
        },
        ChampionId::Sona => match kind {
            AbilityId::Q(AbilityName::_1) => sona_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => sona_q_2(ctx),
            AbilityId::W(AbilityName::_1) => sona_w_1(ctx),
            AbilityId::R(AbilityName::_1) => sona_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Sona'"),
        },
        ChampionId::Soraka => match kind {
            AbilityId::Q(AbilityName::_1) => soraka_q_1(ctx),
            AbilityId::E(AbilityName::_1) => soraka_e_1(ctx),
            AbilityId::E(AbilityName::_2) => soraka_e_2(ctx),
            _ => panic!("Invalid AbilityId for 'Soraka'"),
        },
        ChampionId::Swain => match kind {
            AbilityId::Q(AbilityName::_1) => swain_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => swain_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => swain_q_3(ctx),
            AbilityId::W(AbilityName::_1) => swain_w_1(ctx),
            AbilityId::W(AbilityName::_2) => swain_w_2(ctx),
            AbilityId::E(AbilityName::_1) => swain_e_1(ctx),
            AbilityId::R(AbilityName::_1) => swain_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Swain'"),
        },
        ChampionId::Sylas => match kind {
            AbilityId::Q(AbilityName::_1) => sylas_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => sylas_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => sylas_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => sylas_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => sylas_q_5(ctx),
            AbilityId::W(AbilityName::_1) => sylas_w_1(ctx),
            _ => panic!("Invalid AbilityId for 'Sylas'"),
        },
        ChampionId::Syndra => match kind {
            AbilityId::Q(AbilityName::_1) => syndra_q_1(ctx),
            AbilityId::W(AbilityName::_1) => syndra_w_1(ctx),
            AbilityId::W(AbilityName::_2) => syndra_w_2(ctx),
            AbilityId::W(AbilityName::_3) => syndra_w_3(ctx),
            AbilityId::E(AbilityName::_1) => syndra_e_1(ctx),
            AbilityId::R(AbilityName::_1) => syndra_r_1(ctx),
            AbilityId::R(AbilityName::_2) => syndra_r_2(ctx),
            AbilityId::R(AbilityName::_3) => syndra_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Syndra'"),
        },
        ChampionId::TahmKench => match kind {
            AbilityId::Q(AbilityName::_1) => tahmkench_q_1(ctx),
            AbilityId::W(AbilityName::_1) => tahmkench_w_1(ctx),
            AbilityId::E(AbilityName::_1) => tahmkench_e_1(ctx),
            AbilityId::E(AbilityName::_2) => tahmkench_e_2(ctx),
            _ => panic!("Invalid AbilityId for 'TahmKench'"),
        },
        ChampionId::Taliyah => match kind {
            AbilityId::Q(AbilityName::_1) => taliyah_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => taliyah_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => taliyah_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => taliyah_q_4(ctx),
            AbilityId::E(AbilityName::_1) => taliyah_e_1(ctx),
            AbilityId::E(AbilityName::_2) => taliyah_e_2(ctx),
            AbilityId::E(AbilityName::_3) => taliyah_e_3(ctx),
            _ => panic!("Invalid AbilityId for 'Taliyah'"),
        },
        ChampionId::Talon => match kind {
            AbilityId::Q(AbilityName::_1) => talon_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => talon_q_2(ctx),
            AbilityId::W(AbilityName::_1) => talon_w_1(ctx),
            AbilityId::W(AbilityName::_2) => talon_w_2(ctx),
            AbilityId::W(AbilityName::_3) => talon_w_3(ctx),
            AbilityId::R(AbilityName::_1) => talon_r_1(ctx),
            AbilityId::R(AbilityName::_2) => talon_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Talon'"),
        },
        ChampionId::Taric => match kind {
            AbilityId::E(AbilityName::_1) => taric_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Taric'"),
        },
        ChampionId::Teemo => match kind {
            AbilityId::Q(AbilityName::_1) => teemo_q_1(ctx),
            AbilityId::E(AbilityName::_1) => teemo_e_1(ctx),
            AbilityId::E(AbilityName::_2) => teemo_e_2(ctx),
            AbilityId::E(AbilityName::_3) => teemo_e_3(ctx),
            AbilityId::E(AbilityName::_4) => teemo_e_4(ctx),
            AbilityId::E(AbilityName::_5) => teemo_e_5(ctx),
            AbilityId::E(AbilityName::_6) => teemo_e_6(ctx),
            AbilityId::R(AbilityName::_1) => teemo_r_1(ctx),
            AbilityId::R(AbilityName::_2) => teemo_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Teemo'"),
        },
        ChampionId::Thresh => match kind {
            AbilityId::Q(AbilityName::_1) => thresh_q_1(ctx),
            AbilityId::E(AbilityName::_1) => thresh_e_1(ctx),
            AbilityId::E(AbilityName::_2) => thresh_e_2(ctx),
            AbilityId::E(AbilityName::_3) => thresh_e_3(ctx),
            AbilityId::R(AbilityName::_1) => thresh_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Thresh'"),
        },
        ChampionId::Tristana => match kind {
            AbilityId::W(AbilityName::_1) => tristana_w_1(ctx),
            AbilityId::E(AbilityName::_1) => tristana_e_1(ctx),
            AbilityId::E(AbilityName::_2) => tristana_e_2(ctx),
            AbilityId::E(AbilityName::_3) => tristana_e_3(ctx),
            AbilityId::E(AbilityName::_4) => tristana_e_4(ctx),
            AbilityId::E(AbilityName::_5) => tristana_e_5(ctx),
            AbilityId::R(AbilityName::_1) => tristana_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Tristana'"),
        },
        ChampionId::Trundle => match kind {
            AbilityId::Q(AbilityName::_1) => trundle_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => trundle_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => trundle_q_3(ctx),
            AbilityId::R(AbilityName::_1) => trundle_r_1(ctx),
            AbilityId::R(AbilityName::_2) => trundle_r_2(ctx),
            AbilityId::R(AbilityName::_3) => trundle_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Trundle'"),
        },
        ChampionId::Tryndamere => match kind {
            AbilityId::Q(AbilityName::_1) => tryndamere_q_1(ctx),
            AbilityId::W(AbilityName::_1) => tryndamere_w_1(ctx),
            AbilityId::E(AbilityName::_1) => tryndamere_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'Tryndamere'"),
        },
        ChampionId::TwistedFate => match kind {
            AbilityId::Q(AbilityName::_1) => twistedfate_q_1(ctx),
            AbilityId::W(AbilityName::_1) => twistedfate_w_1(ctx),
            AbilityId::W(AbilityName::_2) => twistedfate_w_2(ctx),
            AbilityId::W(AbilityName::_3) => twistedfate_w_3(ctx),
            AbilityId::E(AbilityName::_1) => twistedfate_e_1(ctx),
            _ => panic!("Invalid AbilityId for 'TwistedFate'"),
        },
        ChampionId::Twitch => match kind {
            AbilityId::E(AbilityName::_1) => twitch_e_1(ctx),
            AbilityId::E(AbilityName::_2) => twitch_e_2(ctx),
            AbilityId::E(AbilityName::_3) => twitch_e_3(ctx),
            AbilityId::E(AbilityName::_4) => twitch_e_4(ctx),
            AbilityId::R(AbilityName::_1) => twitch_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Twitch'"),
        },
        ChampionId::Udyr => match kind {
            AbilityId::Q(AbilityName::_1) => udyr_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => udyr_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => udyr_q_3(ctx),
            AbilityId::R(AbilityName::_1) => udyr_r_1(ctx),
            AbilityId::R(AbilityName::_2) => udyr_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Udyr'"),
        },
        ChampionId::Urgot => match kind {
            AbilityId::Q(AbilityName::_1) => urgot_q_1(ctx),
            AbilityId::W(AbilityName::_1) => urgot_w_1(ctx),
            AbilityId::E(AbilityName::_1) => urgot_e_1(ctx),
            AbilityId::R(AbilityName::_1) => urgot_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Urgot'"),
        },
        ChampionId::Varus => match kind {
            AbilityId::Q(AbilityName::_1) => varus_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => varus_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => varus_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => varus_q_4(ctx),
            AbilityId::W(AbilityName::_1) => varus_w_1(ctx),
            AbilityId::W(AbilityName::_2) => varus_w_2(ctx),
            AbilityId::W(AbilityName::_3) => varus_w_3(ctx),
            AbilityId::W(AbilityName::_4) => varus_w_4(ctx),
            AbilityId::W(AbilityName::_5) => varus_w_5(ctx),
            AbilityId::W(AbilityName::_6) => varus_w_6(ctx),
            AbilityId::W(AbilityName::_7) => varus_w_7(ctx),
            AbilityId::E(AbilityName::_1) => varus_e_1(ctx),
            AbilityId::R(AbilityName::_1) => varus_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Varus'"),
        },
        ChampionId::Vayne => match kind {
            AbilityId::Q(AbilityName::_1) => vayne_q_1(ctx),
            AbilityId::W(AbilityName::_1) => vayne_w_1(ctx),
            AbilityId::W(AbilityName::_2) => vayne_w_2(ctx),
            AbilityId::E(AbilityName::_1) => vayne_e_1(ctx),
            AbilityId::E(AbilityName::_2) => vayne_e_2(ctx),
            AbilityId::E(AbilityName::_3) => vayne_e_3(ctx),
            AbilityId::R(AbilityName::_1) => vayne_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Vayne'"),
        },
        ChampionId::Veigar => match kind {
            AbilityId::Q(AbilityName::_1) => veigar_q_1(ctx),
            AbilityId::W(AbilityName::_1) => veigar_w_1(ctx),
            AbilityId::R(AbilityName::_1) => veigar_r_1(ctx),
            AbilityId::R(AbilityName::_2) => veigar_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Veigar'"),
        },
        ChampionId::Velkoz => match kind {
            AbilityId::Q(AbilityName::_1) => velkoz_q_1(ctx),
            AbilityId::W(AbilityName::_1) => velkoz_w_1(ctx),
            AbilityId::W(AbilityName::_2) => velkoz_w_2(ctx),
            AbilityId::W(AbilityName::_3) => velkoz_w_3(ctx),
            AbilityId::E(AbilityName::_1) => velkoz_e_1(ctx),
            AbilityId::R(AbilityName::_1) => velkoz_r_1(ctx),
            AbilityId::R(AbilityName::_2) => velkoz_r_2(ctx),
            _ => panic!("Invalid AbilityId for 'Velkoz'"),
        },
        ChampionId::Vex => match kind {
            AbilityId::Q(AbilityName::_1) => vex_q_1(ctx),
            AbilityId::W(AbilityName::_1) => vex_w_1(ctx),
            AbilityId::E(AbilityName::_1) => vex_e_1(ctx),
            AbilityId::R(AbilityName::_1) => vex_r_1(ctx),
            AbilityId::R(AbilityName::_2) => vex_r_2(ctx),
            AbilityId::R(AbilityName::_3) => vex_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Vex'"),
        },
        ChampionId::Vi => match kind {
            AbilityId::Q(AbilityName::_1) => vi_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => vi_q_2(ctx),
            AbilityId::W(AbilityName::_1) => vi_w_1(ctx),
            AbilityId::E(AbilityName::_1) => vi_e_1(ctx),
            AbilityId::R(AbilityName::_1) => vi_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Vi'"),
        },
        ChampionId::Viego => match kind {
            AbilityId::Q(AbilityName::_1) => viego_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => viego_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => viego_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => viego_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => viego_q_5(ctx),
            AbilityId::Q(AbilityName::_6) => viego_q_6(ctx),
            AbilityId::W(AbilityName::_1) => viego_w_1(ctx),
            AbilityId::R(AbilityName::_1) => viego_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Viego'"),
        },
        ChampionId::Viktor => match kind {
            AbilityId::Q(AbilityName::_1) => viktor_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => viktor_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => viktor_q_3(ctx),
            AbilityId::E(AbilityName::_1) => viktor_e_1(ctx),
            AbilityId::E(AbilityName::_2) => viktor_e_2(ctx),
            AbilityId::E(AbilityName::_3) => viktor_e_3(ctx),
            AbilityId::R(AbilityName::_1) => viktor_r_1(ctx),
            AbilityId::R(AbilityName::_2) => viktor_r_2(ctx),
            AbilityId::R(AbilityName::_3) => viktor_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Viktor'"),
        },
        ChampionId::Vladimir => match kind {
            AbilityId::Q(AbilityName::_1) => vladimir_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => vladimir_q_2(ctx),
            AbilityId::W(AbilityName::_1) => vladimir_w_1(ctx),
            AbilityId::W(AbilityName::_2) => vladimir_w_2(ctx),
            AbilityId::E(AbilityName::_1) => vladimir_e_1(ctx),
            AbilityId::E(AbilityName::_2) => vladimir_e_2(ctx),
            AbilityId::R(AbilityName::_1) => vladimir_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Vladimir'"),
        },
        ChampionId::Volibear => match kind {
            AbilityId::Q(AbilityName::_1) => volibear_q_1(ctx),
            AbilityId::W(AbilityName::_1) => volibear_w_1(ctx),
            AbilityId::E(AbilityName::_1) => volibear_e_1(ctx),
            AbilityId::R(AbilityName::_1) => volibear_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Volibear'"),
        },
        ChampionId::Warwick => match kind {
            AbilityId::Q(AbilityName::_1) => warwick_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => warwick_q_2(ctx),
            AbilityId::E(AbilityName::_1) => warwick_e_1(ctx),
            AbilityId::R(AbilityName::_1) => warwick_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Warwick'"),
        },
        ChampionId::Xayah => match kind {
            AbilityId::Q(AbilityName::_1) => xayah_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => xayah_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => xayah_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => xayah_q_4(ctx),
            AbilityId::E(AbilityName::_1) => xayah_e_1(ctx),
            AbilityId::E(AbilityName::_2) => xayah_e_2(ctx),
            AbilityId::R(AbilityName::_1) => xayah_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Xayah'"),
        },
        ChampionId::Xerath => match kind {
            AbilityId::Q(AbilityName::_1) => xerath_q_1(ctx),
            AbilityId::W(AbilityName::_1) => xerath_w_1(ctx),
            AbilityId::W(AbilityName::_2) => xerath_w_2(ctx),
            AbilityId::E(AbilityName::_1) => xerath_e_1(ctx),
            AbilityId::R(AbilityName::_1) => xerath_r_1(ctx),
            AbilityId::R(AbilityName::_2) => xerath_r_2(ctx),
            AbilityId::R(AbilityName::_3) => xerath_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Xerath'"),
        },
        ChampionId::XinZhao => match kind {
            AbilityId::Q(AbilityName::_1) => xinzhao_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => xinzhao_q_2(ctx),
            AbilityId::W(AbilityName::_1) => xinzhao_w_1(ctx),
            AbilityId::W(AbilityName::_2) => xinzhao_w_2(ctx),
            AbilityId::W(AbilityName::_3) => xinzhao_w_3(ctx),
            AbilityId::E(AbilityName::_1) => xinzhao_e_1(ctx),
            AbilityId::R(AbilityName::_1) => xinzhao_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'XinZhao'"),
        },
        ChampionId::Yasuo => match kind {
            AbilityId::Q(AbilityName::_1) => yasuo_q_1(ctx),
            AbilityId::E(AbilityName::_1) => yasuo_e_1(ctx),
            AbilityId::E(AbilityName::_2) => yasuo_e_2(ctx),
            AbilityId::E(AbilityName::_3) => yasuo_e_3(ctx),
            AbilityId::E(AbilityName::_4) => yasuo_e_4(ctx),
            AbilityId::R(AbilityName::_1) => yasuo_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Yasuo'"),
        },
        ChampionId::Yone => match kind {
            AbilityId::Q(AbilityName::_1) => yone_q_1(ctx),
            AbilityId::W(AbilityName::_1) => yone_w_1(ctx),
            AbilityId::W(AbilityName::_2) => yone_w_2(ctx),
            AbilityId::W(AbilityName::_3) => yone_w_3(ctx),
            AbilityId::E(AbilityName::_1) => yone_e_1(ctx),
            AbilityId::R(AbilityName::_1) => yone_r_1(ctx),
            AbilityId::R(AbilityName::_2) => yone_r_2(ctx),
            AbilityId::R(AbilityName::_3) => yone_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Yone'"),
        },
        ChampionId::Yorick => match kind {
            AbilityId::Q(AbilityName::_1) => yorick_q_1(ctx),
            AbilityId::E(AbilityName::_1) => yorick_e_1(ctx),
            AbilityId::E(AbilityName::_2) => yorick_e_2(ctx),
            AbilityId::E(AbilityName::_3) => yorick_e_3(ctx),
            _ => panic!("Invalid AbilityId for 'Yorick'"),
        },
        ChampionId::Yunara => match kind {
            AbilityId::Q(AbilityName::_1) => yunara_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => yunara_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => yunara_q_3(ctx),
            AbilityId::Q(AbilityName::_4) => yunara_q_4(ctx),
            AbilityId::Q(AbilityName::_5) => yunara_q_5(ctx),
            AbilityId::W(AbilityName::_1) => yunara_w_1(ctx),
            AbilityId::W(AbilityName::_2) => yunara_w_2(ctx),
            AbilityId::W(AbilityName::_3) => yunara_w_3(ctx),
            AbilityId::R(AbilityName::_1) => yunara_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Yunara'"),
        },
        ChampionId::Yuumi => match kind {
            AbilityId::Q(AbilityName::_1) => yuumi_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => yuumi_q_2(ctx),
            AbilityId::Q(AbilityName::_3) => yuumi_q_3(ctx),
            AbilityId::R(AbilityName::_1) => yuumi_r_1(ctx),
            AbilityId::R(AbilityName::_2) => yuumi_r_2(ctx),
            AbilityId::R(AbilityName::_3) => yuumi_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Yuumi'"),
        },
        ChampionId::Zaahen => match kind {
            AbilityId::Q(AbilityName::_1) => zaahen_q_1(ctx),
            AbilityId::E(AbilityName::_1) => zaahen_e_1(ctx),
            AbilityId::R(AbilityName::_1) => zaahen_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Zaahen'"),
        },
        ChampionId::Zac => match kind {
            AbilityId::Q(AbilityName::_1) => zac_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => zac_q_2(ctx),
            AbilityId::W(AbilityName::_1) => zac_w_1(ctx),
            AbilityId::W(AbilityName::_2) => zac_w_2(ctx),
            AbilityId::E(AbilityName::_1) => zac_e_1(ctx),
            AbilityId::R(AbilityName::_1) => zac_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Zac'"),
        },
        ChampionId::Zed => match kind {
            AbilityId::Q(AbilityName::_1) => zed_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => zed_q_2(ctx),
            AbilityId::R(AbilityName::_3) => zed_r_3(ctx),
            _ => panic!("Invalid AbilityId for 'Zed'"),
        },
        ChampionId::Zeri => match kind {
            AbilityId::Q(AbilityName::_1) => zeri_q_1(ctx),
            AbilityId::Q(AbilityName::_2) => zeri_q_2(ctx),
            _ => panic!("Invalid AbilityId for 'Zeri'"),
        },
        ChampionId::Ziggs => match kind {
            AbilityId::E(AbilityName::_1) => ziggs_e_1(ctx),
            AbilityId::E(AbilityName::_2) => ziggs_e_2(ctx),
            _ => panic!("Invalid AbilityId for 'Ziggs'"),
        },
        ChampionId::Zilean => match kind {
            AbilityId::Q(AbilityName::_1) => zilean_q_1(ctx),
            AbilityId::E(AbilityName::_1) => zilean_e_1(ctx),
            AbilityId::R(AbilityName::_1) => zilean_r_1(ctx),
            _ => panic!("Invalid AbilityId for 'Zilean'"),
        },
        ChampionId::Zoe => match kind {
            _ => panic!("Invalid AbilityId for 'Zoe'"),
        },
        ChampionId::Zyra => match kind {
            AbilityId::Q(AbilityName::_1) => zyra_q_1(ctx),
            _ => panic!("Invalid AbilityId for 'Zyra'"),
        },
    }
}
