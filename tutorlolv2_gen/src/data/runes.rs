use super::*;
pub static RUNE_CACHE: [&CachedRune; 72] = [
    &ABILITY_HASTE_9994,
    &ABSOLUTE_FOCUS_8233,
    &ABSORB_LIFE_9101,
    &ADAPTIVE_FORCE_9990,
    &AFTERSHOCK_8439,
    &APPROACH_VELOCITY_8410,
    &ARCANE_COMET_8229,
    &ATTACK_SPEED_9992,
    &AXIOM_ARCANIST_8224,
    &BISCUIT_DELIVERY_8345,
    &BONE_PLATING_8473,
    &CASH_BACK_8321,
    &CELERITY_8234,
    &CHEAP_SHOT_8126,
    &CONDITIONING_8429,
    &CONQUEROR_8010,
    &COSMIC_INSIGHT_8347,
    &COUP_DE_GRACE_8014,
    &CUT_DOWN_8017,
    &DARK_HARVEST_8128,
    &DEEP_WARD_8141,
    &DEMOLISH_8446,
    &ELECTROCUTE_8112,
    &EYEBALL_COLLECTION_8120,
    &FIRST_STRIKE_8369,
    &FLEET_FOOTWORK_8021,
    &FONT_OF_LIFE_8463,
    &GATHERING_STORM_8236,
    &GHOST_PORO_8136,
    &GLACIAL_AUGMENT_8351,
    &GRASP_OF_THE_UNDYING_8437,
    &GRISLY_MEMENTOS_8140,
    &GUARDIAN_8465,
    &HAIL_OF_BLADES_9923,
    &HEALTH_9993,
    &HEALTH_SCALING_9991,
    &HEXTECH_FLASHTRAPTION_8306,
    &JACK_OF_ALL_TRADES_8316,
    &LAST_STAND_8299,
    &LEGEND_ALACRITY_9104,
    &LEGEND_BLOODLINE_9103,
    &LEGEND_HASTE_9105,
    &LETHAL_TEMPO_8008,
    &MAGICAL_FOOTWEAR_8304,
    &MANAFLOW_BAND_8226,
    &MOVE_SPEED_9996,
    &NIMBUS_CLOAK_8275,
    &OVERGROWTH_8451,
    &PHASE_RUSH_8230,
    &PREDATOR_8124,
    &PRESENCE_OF_MIND_8009,
    &PRESS_THE_ATTACK_8005,
    &RELENTLESS_HUNTER_8105,
    &REVITALIZE_8453,
    &SCORCH_8237,
    &SECOND_WIND_8444,
    &SHIELD_BASH_8401,
    &SIXTH_SENSE_8137,
    &SUDDEN_IMPACT_8143,
    &SUMMON_AERY_8214,
    &TASTE_OF_BLOOD_8139,
    &TENACITY_AND_SLOW_RESIST_9995,
    &TIME_WARP_TONIC_8352,
    &TRANSCENDENCE_8210,
    &TREASURE_HUNTER_8135,
    &TRIPLE_TONIC_8313,
    &TRIUMPH_9111,
    &ULTIMATE_HUNTER_8106,
    &UNFLINCHING_8242,
    &UNSEALED_SPELLBOOK_8360,
    &WATERWALKING_8232,
    &ZOMBIE_WARD_8138,
];
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum RuneId {
    AbilityHaste,
    AbsoluteFocus,
    AbsorbLife,
    AdaptiveForce,
    Aftershock,
    ApproachVelocity,
    ArcaneComet,
    AttackSpeed,
    AxiomArcanist,
    BiscuitDelivery,
    BonePlating,
    CashBack,
    Celerity,
    CheapShot,
    Conditioning,
    Conqueror,
    CosmicInsight,
    CoupDeGrace,
    CutDown,
    DarkHarvest,
    DeepWard,
    Demolish,
    Electrocute,
    EyeballCollection,
    FirstStrike,
    FleetFootwork,
    FontOfLife,
    GatheringStorm,
    GhostPoro,
    GlacialAugment,
    GraspOfTheUndying,
    GrislyMementos,
    Guardian,
    HailOfBlades,
    Health,
    HealthScaling,
    HextechFlashtraption,
    JackOfAllTrades,
    LastStand,
    LegendAlacrity,
    LegendBloodline,
    LegendHaste,
    LethalTempo,
    MagicalFootwear,
    ManaflowBand,
    MoveSpeed,
    NimbusCloak,
    Overgrowth,
    PhaseRush,
    Predator,
    PresenceOfMind,
    PressTheAttack,
    RelentlessHunter,
    Revitalize,
    Scorch,
    SecondWind,
    ShieldBash,
    SixthSense,
    SuddenImpact,
    SummonAery,
    TasteOfBlood,
    TenacityAndSlowResist,
    TimeWarpTonic,
    Transcendence,
    TreasureHunter,
    TripleTonic,
    Triumph,
    UltimateHunter,
    Unflinching,
    UnsealedSpellbook,
    Waterwalking,
    ZombieWard,
}
impl RuneId {
    pub const fn to_riot_id(&self) -> u32 {
        RUNE_CACHE[*self as usize].riot_id
    }
    pub const fn from_riot_id(id: u32) -> Option<Self> {
        match id {
            9994 => Some(Self::AbilityHaste),
            8233 => Some(Self::AbsoluteFocus),
            9101 => Some(Self::AbsorbLife),
            9990 => Some(Self::AdaptiveForce),
            8439 => Some(Self::Aftershock),
            8410 => Some(Self::ApproachVelocity),
            8229 => Some(Self::ArcaneComet),
            9992 => Some(Self::AttackSpeed),
            8224 => Some(Self::AxiomArcanist),
            8345 => Some(Self::BiscuitDelivery),
            8473 => Some(Self::BonePlating),
            8321 => Some(Self::CashBack),
            8234 => Some(Self::Celerity),
            8126 => Some(Self::CheapShot),
            8429 => Some(Self::Conditioning),
            8010 => Some(Self::Conqueror),
            8347 => Some(Self::CosmicInsight),
            8014 => Some(Self::CoupDeGrace),
            8017 => Some(Self::CutDown),
            8128 => Some(Self::DarkHarvest),
            8141 => Some(Self::DeepWard),
            8446 => Some(Self::Demolish),
            8112 => Some(Self::Electrocute),
            8120 => Some(Self::EyeballCollection),
            8369 => Some(Self::FirstStrike),
            8021 => Some(Self::FleetFootwork),
            8463 => Some(Self::FontOfLife),
            8236 => Some(Self::GatheringStorm),
            8136 => Some(Self::GhostPoro),
            8351 => Some(Self::GlacialAugment),
            8437 => Some(Self::GraspOfTheUndying),
            8140 => Some(Self::GrislyMementos),
            8465 => Some(Self::Guardian),
            9923 => Some(Self::HailOfBlades),
            9993 => Some(Self::Health),
            9991 => Some(Self::HealthScaling),
            8306 => Some(Self::HextechFlashtraption),
            8316 => Some(Self::JackOfAllTrades),
            8299 => Some(Self::LastStand),
            9104 => Some(Self::LegendAlacrity),
            9103 => Some(Self::LegendBloodline),
            9105 => Some(Self::LegendHaste),
            8008 => Some(Self::LethalTempo),
            8304 => Some(Self::MagicalFootwear),
            8226 => Some(Self::ManaflowBand),
            9996 => Some(Self::MoveSpeed),
            8275 => Some(Self::NimbusCloak),
            8451 => Some(Self::Overgrowth),
            8230 => Some(Self::PhaseRush),
            8124 => Some(Self::Predator),
            8009 => Some(Self::PresenceOfMind),
            8005 => Some(Self::PressTheAttack),
            8105 => Some(Self::RelentlessHunter),
            8453 => Some(Self::Revitalize),
            8237 => Some(Self::Scorch),
            8444 => Some(Self::SecondWind),
            8401 => Some(Self::ShieldBash),
            8137 => Some(Self::SixthSense),
            8143 => Some(Self::SuddenImpact),
            8214 => Some(Self::SummonAery),
            8139 => Some(Self::TasteOfBlood),
            9995 => Some(Self::TenacityAndSlowResist),
            8352 => Some(Self::TimeWarpTonic),
            8210 => Some(Self::Transcendence),
            8135 => Some(Self::TreasureHunter),
            8313 => Some(Self::TripleTonic),
            9111 => Some(Self::Triumph),
            8106 => Some(Self::UltimateHunter),
            8242 => Some(Self::Unflinching),
            8360 => Some(Self::UnsealedSpellbook),
            8232 => Some(Self::Waterwalking),
            8138 => Some(Self::ZombieWard),
            _ => None,
        }
    }
    pub const unsafe fn from_u8_unchecked(id: u8) -> Self {
        unsafe { core::mem::transmute(id) }
    }
    pub const fn from_u8(id: u8) -> Option<Self> {
        if id < 72 as u8 {
            Some(unsafe { Self::from_u8_unchecked(id) })
        } else {
            None
        }
    }
}
pub static ABILITY_HASTE_9994: CachedRune = CachedRune {
    name: "Ability Haste",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::AbilityHaste,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9994,
    internal_id: RuneId::AbilityHaste,
    undeclared: true,
};
pub static ABSOLUTE_FOCUS_8233: CachedRune = CachedRune {
    name: "Absolute Focus",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::AbsoluteFocus,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8233,
    internal_id: RuneId::AbsoluteFocus,
    undeclared: true,
};
pub static ABSORB_LIFE_9101: CachedRune = CachedRune {
    name: "Absorb Life",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::AbsorbLife,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9101,
    internal_id: RuneId::AbsorbLife,
    undeclared: true,
};
pub static ADAPTIVE_FORCE_9990: CachedRune = CachedRune {
    name: "Adaptive Force",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::AdaptiveForce,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9990,
    internal_id: RuneId::AdaptiveForce,
    undeclared: true,
};
pub static AFTERSHOCK_8439: CachedRune = CachedRune {
    name: "Aftershock",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::Aftershock,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8439,
    internal_id: RuneId::Aftershock,
    undeclared: false,
    melee_closure: aftershock_melee,
    ranged_closure: aftershock_ranged,
};
pub const fn aftershock_ranged(ctx: &EvalContext) -> f32 {
    (25f32 + 95f32 / 17f32 * (ctx.level - 1f32) + 0.08f32 * ctx.bonus_health)
        * ctx.magic_multiplier
}
pub const fn aftershock_melee(ctx: &EvalContext) -> f32 {
    (25f32 + 95f32 / 17f32 * (ctx.level - 1f32) + 0.08f32 * ctx.bonus_health)
        * ctx.magic_multiplier
}
pub static APPROACH_VELOCITY_8410: CachedRune = CachedRune {
    name: "Approach Velocity",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::ApproachVelocity,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8410,
    internal_id: RuneId::ApproachVelocity,
    undeclared: true,
};
pub static ARCANE_COMET_8229: CachedRune = CachedRune {
    name: "Arcane Comet",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::ArcaneComet,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8229,
    internal_id: RuneId::ArcaneComet,
    undeclared: false,
    melee_closure: arcane_comet_melee,
    ranged_closure: arcane_comet_ranged,
};
pub const fn arcane_comet_ranged(ctx: &EvalContext) -> f32 {
    (30f32
        + 100f32 / 17f32 * (ctx.level - 1f32)
        + (0.1f32 * ctx.bonus_ad + 0.05f32 * ctx.ap))
        * ctx.adaptative_damage
}
pub const fn arcane_comet_melee(ctx: &EvalContext) -> f32 {
    (30f32
        + 100f32 / 17f32 * (ctx.level - 1f32)
        + (0.1f32 * ctx.bonus_ad + 0.05f32 * ctx.ap))
        * ctx.adaptative_damage
}
pub static ATTACK_SPEED_9992: CachedRune = CachedRune {
    name: "Attack Speed",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::AttackSpeed,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9992,
    internal_id: RuneId::AttackSpeed,
    undeclared: true,
};
pub static AXIOM_ARCANIST_8224: CachedRune = CachedRune {
    name: "Axiom Arcanist",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::AxiomArcanist,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8224,
    internal_id: RuneId::AxiomArcanist,
    undeclared: true,
};
pub static BISCUIT_DELIVERY_8345: CachedRune = CachedRune {
    name: "Biscuit Delivery",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::BiscuitDelivery,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8345,
    internal_id: RuneId::BiscuitDelivery,
    undeclared: true,
};
pub static BONE_PLATING_8473: CachedRune = CachedRune {
    name: "Bone Plating",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::BonePlating,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8473,
    internal_id: RuneId::BonePlating,
    undeclared: true,
};
pub static CASH_BACK_8321: CachedRune = CachedRune {
    name: "Cash Back",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::CashBack,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8321,
    internal_id: RuneId::CashBack,
    undeclared: true,
};
pub static CELERITY_8234: CachedRune = CachedRune {
    name: "Celerity",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Celerity,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8234,
    internal_id: RuneId::Celerity,
    undeclared: true,
};
pub static CHEAP_SHOT_8126: CachedRune = CachedRune {
    name: "Cheap Shot",
    damage_type: DamageType::True,
    metadata: TypeMetadata {
        kind: RuneId::CheapShot,
        damage_type: DamageType::True,
        attributes: Attrs::Undefined,
    },
    riot_id: 8126,
    internal_id: RuneId::CheapShot,
    undeclared: false,
    melee_closure: cheap_shot_melee,
    ranged_closure: cheap_shot_ranged,
};
pub const fn cheap_shot_ranged(ctx: &EvalContext) -> f32 {
    10f32 + 35f32 / 17f32 * (ctx.level - 1f32)
}
pub const fn cheap_shot_melee(ctx: &EvalContext) -> f32 {
    10f32 + 35f32 / 17f32 * (ctx.level - 1f32)
}
pub static CONDITIONING_8429: CachedRune = CachedRune {
    name: "Conditioning",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Conditioning,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8429,
    internal_id: RuneId::Conditioning,
    undeclared: true,
};
pub static CONQUEROR_8010: CachedRune = CachedRune {
    name: "Conqueror",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Conqueror,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8010,
    internal_id: RuneId::Conqueror,
    undeclared: true,
};
pub static COSMIC_INSIGHT_8347: CachedRune = CachedRune {
    name: "Cosmic Insight",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::CosmicInsight,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8347,
    internal_id: RuneId::CosmicInsight,
    undeclared: true,
};
pub static COUP_DE_GRACE_8014: CachedRune = CachedRune {
    name: "Coup de Grace",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::CoupDeGrace,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8014,
    internal_id: RuneId::CoupDeGrace,
    undeclared: true,
};
pub static CUT_DOWN_8017: CachedRune = CachedRune {
    name: "Cut Down",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::CutDown,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8017,
    internal_id: RuneId::CutDown,
    undeclared: true,
};
pub static DARK_HARVEST_8128: CachedRune = CachedRune {
    name: "Dark Harvest",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::DarkHarvest,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8128,
    internal_id: RuneId::DarkHarvest,
    undeclared: false,
    melee_closure: dark_harvest_melee,
    ranged_closure: dark_harvest_ranged,
};
pub const fn dark_harvest_ranged(ctx: &EvalContext) -> f32 {
    (20f32
        + 60f32 / 17f32 * (ctx.level - 1f32)
        + 0.1f32 * ctx.bonus_ad
        + 0.05f32 * ctx.ap)
        * ctx.adaptative_damage
}
pub const fn dark_harvest_melee(ctx: &EvalContext) -> f32 {
    (20f32
        + 60f32 / 17f32 * (ctx.level - 1f32)
        + 0.1f32 * ctx.bonus_ad
        + 0.05f32 * ctx.ap)
        * ctx.adaptative_damage
}
pub static DEEP_WARD_8141: CachedRune = CachedRune {
    name: "Deep Ward",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::DeepWard,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8141,
    internal_id: RuneId::DeepWard,
    undeclared: true,
};
pub static DEMOLISH_8446: CachedRune = CachedRune {
    name: "Demolish",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Demolish,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8446,
    internal_id: RuneId::Demolish,
    undeclared: true,
};
pub static ELECTROCUTE_8112: CachedRune = CachedRune {
    name: "Electrocute",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::Electrocute,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8112,
    internal_id: RuneId::Electrocute,
    undeclared: false,
    melee_closure: electrocute_melee,
    ranged_closure: electrocute_ranged,
};
pub const fn electrocute_ranged(ctx: &EvalContext) -> f32 {
    (30f32
        + 190f32 / 17f32 * (ctx.level - 1f32)
        + 0.1f32 * ctx.bonus_ad
        + 0.05f32 * ctx.ap)
        * ctx.adaptative_damage
}
pub const fn electrocute_melee(ctx: &EvalContext) -> f32 {
    (30f32
        + 190f32 / 17f32 * (ctx.level - 1f32)
        + 0.1f32 * ctx.bonus_ad
        + 0.05f32 * ctx.ap)
        * ctx.adaptative_damage
}
pub static EYEBALL_COLLECTION_8120: CachedRune = CachedRune {
    name: "Eyeball Collection",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::EyeballCollection,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8120,
    internal_id: RuneId::EyeballCollection,
    undeclared: true,
};
pub static FIRST_STRIKE_8369: CachedRune = CachedRune {
    name: "First Strike",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::FirstStrike,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8369,
    internal_id: RuneId::FirstStrike,
    undeclared: true,
};
pub static FLEET_FOOTWORK_8021: CachedRune = CachedRune {
    name: "Fleet Footwork",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::FleetFootwork,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8021,
    internal_id: RuneId::FleetFootwork,
    undeclared: true,
};
pub static FONT_OF_LIFE_8463: CachedRune = CachedRune {
    name: "Font of Life",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::FontOfLife,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8463,
    internal_id: RuneId::FontOfLife,
    undeclared: true,
};
pub static GATHERING_STORM_8236: CachedRune = CachedRune {
    name: "Gathering Storm",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::GatheringStorm,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8236,
    internal_id: RuneId::GatheringStorm,
    undeclared: true,
};
pub static GHOST_PORO_8136: CachedRune = CachedRune {
    name: "Ghost Poro",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::GhostPoro,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8136,
    internal_id: RuneId::GhostPoro,
    undeclared: true,
};
pub static GLACIAL_AUGMENT_8351: CachedRune = CachedRune {
    name: "Glacial Augment",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::GlacialAugment,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8351,
    internal_id: RuneId::GlacialAugment,
    undeclared: true,
};
pub static GRASP_OF_THE_UNDYING_8437: CachedRune = CachedRune {
    name: "Grasp of the Undying",
    damage_type: DamageType::Magic,
    metadata: TypeMetadata {
        kind: RuneId::GraspOfTheUndying,
        damage_type: DamageType::Magic,
        attributes: Attrs::Undefined,
    },
    riot_id: 8437,
    internal_id: RuneId::GraspOfTheUndying,
    undeclared: false,
    melee_closure: grasp_of_the_undying_melee,
    ranged_closure: grasp_of_the_undying_ranged,
};
pub const fn grasp_of_the_undying_ranged(ctx: &EvalContext) -> f32 {
    0.021f32 * ctx.max_health * ctx.magic_multiplier
}
pub const fn grasp_of_the_undying_melee(ctx: &EvalContext) -> f32 {
    0.035f32 * ctx.max_health * ctx.magic_multiplier
}
pub static GRISLY_MEMENTOS_8140: CachedRune = CachedRune {
    name: "Grisly Mementos",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::GrislyMementos,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8140,
    internal_id: RuneId::GrislyMementos,
    undeclared: true,
};
pub static GUARDIAN_8465: CachedRune = CachedRune {
    name: "Guardian",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Guardian,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8465,
    internal_id: RuneId::Guardian,
    undeclared: true,
};
pub static HAIL_OF_BLADES_9923: CachedRune = CachedRune {
    name: "Hail of Blades",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::HailOfBlades,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9923,
    internal_id: RuneId::HailOfBlades,
    undeclared: true,
};
pub static HEALTH_9993: CachedRune = CachedRune {
    name: "Health",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Health,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9993,
    internal_id: RuneId::Health,
    undeclared: true,
};
pub static HEALTH_SCALING_9991: CachedRune = CachedRune {
    name: "Health Scaling",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::HealthScaling,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9991,
    internal_id: RuneId::HealthScaling,
    undeclared: true,
};
pub static HEXTECH_FLASHTRAPTION_8306: CachedRune = CachedRune {
    name: "Hextech Flashtraption",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::HextechFlashtraption,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8306,
    internal_id: RuneId::HextechFlashtraption,
    undeclared: true,
};
pub static JACK_OF_ALL_TRADES_8316: CachedRune = CachedRune {
    name: "Jack Of All Trades",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::JackOfAllTrades,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8316,
    internal_id: RuneId::JackOfAllTrades,
    undeclared: true,
};
pub static LAST_STAND_8299: CachedRune = CachedRune {
    name: "Last Stand",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::LastStand,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8299,
    internal_id: RuneId::LastStand,
    undeclared: true,
};
pub static LEGEND_ALACRITY_9104: CachedRune = CachedRune {
    name: "Legend: Alacrity",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::LegendAlacrity,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9104,
    internal_id: RuneId::LegendAlacrity,
    undeclared: true,
};
pub static LEGEND_BLOODLINE_9103: CachedRune = CachedRune {
    name: "Legend: Bloodline",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::LegendBloodline,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9103,
    internal_id: RuneId::LegendBloodline,
    undeclared: true,
};
pub static LEGEND_HASTE_9105: CachedRune = CachedRune {
    name: "Legend: Haste",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::LegendHaste,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9105,
    internal_id: RuneId::LegendHaste,
    undeclared: true,
};
pub static LETHAL_TEMPO_8008: CachedRune = CachedRune {
    name: "Lethal Tempo",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::LethalTempo,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8008,
    internal_id: RuneId::LethalTempo,
    undeclared: true,
};
pub static MAGICAL_FOOTWEAR_8304: CachedRune = CachedRune {
    name: "Magical Footwear",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::MagicalFootwear,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8304,
    internal_id: RuneId::MagicalFootwear,
    undeclared: true,
};
pub static MANAFLOW_BAND_8226: CachedRune = CachedRune {
    name: "Manaflow Band",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::ManaflowBand,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8226,
    internal_id: RuneId::ManaflowBand,
    undeclared: true,
};
pub static MOVE_SPEED_9996: CachedRune = CachedRune {
    name: "Move Speed",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::MoveSpeed,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9996,
    internal_id: RuneId::MoveSpeed,
    undeclared: true,
};
pub static NIMBUS_CLOAK_8275: CachedRune = CachedRune {
    name: "Nimbus Cloak",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::NimbusCloak,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8275,
    internal_id: RuneId::NimbusCloak,
    undeclared: true,
};
pub static OVERGROWTH_8451: CachedRune = CachedRune {
    name: "Overgrowth",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Overgrowth,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8451,
    internal_id: RuneId::Overgrowth,
    undeclared: true,
};
pub static PHASE_RUSH_8230: CachedRune = CachedRune {
    name: "Phase Rush",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::PhaseRush,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8230,
    internal_id: RuneId::PhaseRush,
    undeclared: true,
};
pub static PREDATOR_8124: CachedRune = CachedRune {
    name: "Predator",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::Predator,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8124,
    internal_id: RuneId::Predator,
    undeclared: false,
    melee_closure: predator_melee,
    ranged_closure: predator_ranged,
};
pub const fn predator_ranged(ctx: &EvalContext) -> f32 {
    (20f32
        + 160f32 / 17f32 * (ctx.level - 1f32)
        + (0.25f32 * ctx.bonus_ad + 0.15f32 * ctx.ap))
        * ctx.adaptative_damage
}
pub const fn predator_melee(ctx: &EvalContext) -> f32 {
    (20f32
        + 160f32 / 17f32 * (ctx.level - 1f32)
        + (0.25f32 * ctx.bonus_ad + 0.15f32 * ctx.ap))
        * ctx.adaptative_damage
}
pub static PRESENCE_OF_MIND_8009: CachedRune = CachedRune {
    name: "Presence of Mind",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::PresenceOfMind,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8009,
    internal_id: RuneId::PresenceOfMind,
    undeclared: true,
};
pub static PRESS_THE_ATTACK_8005: CachedRune = CachedRune {
    name: "Press The Attack",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::PressTheAttack,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8005,
    internal_id: RuneId::PressTheAttack,
    undeclared: false,
    melee_closure: press_the_attack_melee,
    ranged_closure: press_the_attack_ranged,
};
pub const fn press_the_attack_ranged(ctx: &EvalContext) -> f32 {
    (40f32 + 120f32 / 17f32 * (ctx.level - 1f32)) * ctx.adaptative_damage
}
pub const fn press_the_attack_melee(ctx: &EvalContext) -> f32 {
    (40f32 + 120f32 / 17f32 * (ctx.level - 1f32)) * ctx.adaptative_damage
}
pub static RELENTLESS_HUNTER_8105: CachedRune = CachedRune {
    name: "Relentless Hunter",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::RelentlessHunter,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8105,
    internal_id: RuneId::RelentlessHunter,
    undeclared: true,
};
pub static REVITALIZE_8453: CachedRune = CachedRune {
    name: "Revitalize",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Revitalize,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8453,
    internal_id: RuneId::Revitalize,
    undeclared: true,
};
pub static SCORCH_8237: CachedRune = CachedRune {
    name: "Scorch",
    damage_type: DamageType::Magic,
    metadata: TypeMetadata {
        kind: RuneId::Scorch,
        damage_type: DamageType::Magic,
        attributes: Attrs::Undefined,
    },
    riot_id: 8237,
    internal_id: RuneId::Scorch,
    undeclared: false,
    melee_closure: scorch_melee,
    ranged_closure: scorch_ranged,
};
pub const fn scorch_ranged(ctx: &EvalContext) -> f32 {
    (20f32 + 20f32 / 17f32 * (ctx.level - 1f32)) * ctx.magic_multiplier
}
pub const fn scorch_melee(ctx: &EvalContext) -> f32 {
    (20f32 + 20f32 / 17f32 * (ctx.level - 1f32)) * ctx.magic_multiplier
}
pub static SECOND_WIND_8444: CachedRune = CachedRune {
    name: "Second Wind",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::SecondWind,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8444,
    internal_id: RuneId::SecondWind,
    undeclared: true,
};
pub static SHIELD_BASH_8401: CachedRune = CachedRune {
    name: "Shield Bash",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::ShieldBash,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8401,
    internal_id: RuneId::ShieldBash,
    undeclared: true,
};
pub static SIXTH_SENSE_8137: CachedRune = CachedRune {
    name: "Sixth Sense",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::SixthSense,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8137,
    internal_id: RuneId::SixthSense,
    undeclared: true,
};
pub static SUDDEN_IMPACT_8143: CachedRune = CachedRune {
    name: "Sudden Impact",
    damage_type: DamageType::True,
    metadata: TypeMetadata {
        kind: RuneId::SuddenImpact,
        damage_type: DamageType::True,
        attributes: Attrs::Undefined,
    },
    riot_id: 8143,
    internal_id: RuneId::SuddenImpact,
    undeclared: false,
    melee_closure: sudden_impact_melee,
    ranged_closure: sudden_impact_ranged,
};
pub const fn sudden_impact_ranged(ctx: &EvalContext) -> f32 {
    20f32 + 60f32 / 17f32 * (ctx.level - 1f32)
}
pub const fn sudden_impact_melee(ctx: &EvalContext) -> f32 {
    20f32 + 60f32 / 17f32 * (ctx.level - 1f32)
}
pub static SUMMON_AERY_8214: CachedRune = CachedRune {
    name: "Summon Aery",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::SummonAery,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8214,
    internal_id: RuneId::SummonAery,
    undeclared: false,
    melee_closure: summon_aery_melee,
    ranged_closure: summon_aery_ranged,
};
pub const fn summon_aery_ranged(ctx: &EvalContext) -> f32 {
    (10f32
        + 40f32 / 17f32 * (ctx.level - 1f32)
        + 0.1f32 * ctx.bonus_ad
        + 0.05f32 * ctx.ap)
        * ctx.adaptative_damage
}
pub const fn summon_aery_melee(ctx: &EvalContext) -> f32 {
    (10f32
        + 40f32 / 17f32 * (ctx.level - 1f32)
        + 0.1f32 * ctx.bonus_ad
        + 0.05f32 * ctx.ap)
        * ctx.adaptative_damage
}
pub static TASTE_OF_BLOOD_8139: CachedRune = CachedRune {
    name: "Taste of Blood",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::TasteOfBlood,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8139,
    internal_id: RuneId::TasteOfBlood,
    undeclared: true,
};
pub static TENACITY_AND_SLOW_RESIST_9995: CachedRune = CachedRune {
    name: "Tenacity and Slow Resist",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::TenacityAndSlowResist,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9995,
    internal_id: RuneId::TenacityAndSlowResist,
    undeclared: true,
};
pub static TIME_WARP_TONIC_8352: CachedRune = CachedRune {
    name: "Time Warp Tonic",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::TimeWarpTonic,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8352,
    internal_id: RuneId::TimeWarpTonic,
    undeclared: true,
};
pub static TRANSCENDENCE_8210: CachedRune = CachedRune {
    name: "Transcendence",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Transcendence,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8210,
    internal_id: RuneId::Transcendence,
    undeclared: true,
};
pub static TREASURE_HUNTER_8135: CachedRune = CachedRune {
    name: "Treasure Hunter",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::TreasureHunter,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8135,
    internal_id: RuneId::TreasureHunter,
    undeclared: true,
};
pub static TRIPLE_TONIC_8313: CachedRune = CachedRune {
    name: "Triple Tonic",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::TripleTonic,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8313,
    internal_id: RuneId::TripleTonic,
    undeclared: true,
};
pub static TRIUMPH_9111: CachedRune = CachedRune {
    name: "Triumph",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Triumph,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 9111,
    internal_id: RuneId::Triumph,
    undeclared: true,
};
pub static ULTIMATE_HUNTER_8106: CachedRune = CachedRune {
    name: "Ultimate Hunter",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::UltimateHunter,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8106,
    internal_id: RuneId::UltimateHunter,
    undeclared: true,
};
pub static UNFLINCHING_8242: CachedRune = CachedRune {
    name: "Unflinching",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Unflinching,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8242,
    internal_id: RuneId::Unflinching,
    undeclared: true,
};
pub static UNSEALED_SPELLBOOK_8360: CachedRune = CachedRune {
    name: "Unsealed Spellbook",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::UnsealedSpellbook,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8360,
    internal_id: RuneId::UnsealedSpellbook,
    undeclared: true,
};
pub static WATERWALKING_8232: CachedRune = CachedRune {
    name: "Waterwalking",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::Waterwalking,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8232,
    internal_id: RuneId::Waterwalking,
    undeclared: true,
};
pub static ZOMBIE_WARD_8138: CachedRune = CachedRune {
    name: "Zombie Ward",
    damage_type: DamageType::Unknown,
    metadata: TypeMetadata {
        kind: RuneId::ZombieWard,
        damage_type: DamageType::Unknown,
        attributes: Attrs::Undefined,
    },
    melee_closure: zero,
    ranged_closure: zero,
    riot_id: 8138,
    internal_id: RuneId::ZombieWard,
    undeclared: true,
};
pub const fn rune_const_eval(
    ctx: &EvalContext,
    rune_id: RuneId,
    attack_type: AttackType,
) -> f32 {
    match rune_id {
        RuneId::AbilityHaste => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::AbsoluteFocus => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::AbsorbLife => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::AdaptiveForce => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Aftershock => match attack_type {
            AttackType::Melee => aftershock_melee(ctx),
            AttackType::Ranged => aftershock_ranged(ctx),
        },
        RuneId::ApproachVelocity => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::ArcaneComet => match attack_type {
            AttackType::Melee => arcane_comet_melee(ctx),
            AttackType::Ranged => arcane_comet_ranged(ctx),
        },
        RuneId::AttackSpeed => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::AxiomArcanist => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::BiscuitDelivery => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::BonePlating => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::CashBack => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Celerity => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::CheapShot => match attack_type {
            AttackType::Melee => cheap_shot_melee(ctx),
            AttackType::Ranged => cheap_shot_ranged(ctx),
        },
        RuneId::Conditioning => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Conqueror => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::CosmicInsight => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::CoupDeGrace => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::CutDown => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::DarkHarvest => match attack_type {
            AttackType::Melee => dark_harvest_melee(ctx),
            AttackType::Ranged => dark_harvest_ranged(ctx),
        },
        RuneId::DeepWard => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Demolish => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Electrocute => match attack_type {
            AttackType::Melee => electrocute_melee(ctx),
            AttackType::Ranged => electrocute_ranged(ctx),
        },
        RuneId::EyeballCollection => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::FirstStrike => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::FleetFootwork => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::FontOfLife => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::GatheringStorm => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::GhostPoro => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::GlacialAugment => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::GraspOfTheUndying => match attack_type {
            AttackType::Melee => grasp_of_the_undying_melee(ctx),
            AttackType::Ranged => grasp_of_the_undying_ranged(ctx),
        },
        RuneId::GrislyMementos => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Guardian => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::HailOfBlades => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Health => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::HealthScaling => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::HextechFlashtraption => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::JackOfAllTrades => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::LastStand => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::LegendAlacrity => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::LegendBloodline => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::LegendHaste => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::LethalTempo => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::MagicalFootwear => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::ManaflowBand => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::MoveSpeed => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::NimbusCloak => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Overgrowth => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::PhaseRush => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Predator => match attack_type {
            AttackType::Melee => predator_melee(ctx),
            AttackType::Ranged => predator_ranged(ctx),
        },
        RuneId::PresenceOfMind => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::PressTheAttack => match attack_type {
            AttackType::Melee => press_the_attack_melee(ctx),
            AttackType::Ranged => press_the_attack_ranged(ctx),
        },
        RuneId::RelentlessHunter => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Revitalize => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Scorch => match attack_type {
            AttackType::Melee => scorch_melee(ctx),
            AttackType::Ranged => scorch_ranged(ctx),
        },
        RuneId::SecondWind => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::ShieldBash => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::SixthSense => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::SuddenImpact => match attack_type {
            AttackType::Melee => sudden_impact_melee(ctx),
            AttackType::Ranged => sudden_impact_ranged(ctx),
        },
        RuneId::SummonAery => match attack_type {
            AttackType::Melee => summon_aery_melee(ctx),
            AttackType::Ranged => summon_aery_ranged(ctx),
        },
        RuneId::TasteOfBlood => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::TenacityAndSlowResist => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::TimeWarpTonic => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Transcendence => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::TreasureHunter => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::TripleTonic => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Triumph => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::UltimateHunter => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Unflinching => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::UnsealedSpellbook => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::Waterwalking => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
        RuneId::ZombieWard => match attack_type {
            AttackType::Melee => zero(ctx),
            AttackType::Ranged => zero(ctx),
        },
    }
}
