use super::*;
pub static RUNE_GENERATOR: [Range<usize>; RuneId::VARIANTS] = [
    0..0,
    0..0,
    0..0,
    4680433..4681230,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    4703102..4704203,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    4731432..4732530,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    4738991..4739784,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
    0..0,
];
pub static RUNE_FORMULAS: [Range<usize>; RuneId::VARIANTS] = [
    4676557..4677172,
    4677172..4677778,
    4677778..4678399,
    4679443..4680433,
    4681230..4681854,
    4683712..4684961,
    4684961..4685576,
    4685576..4686191,
    4686191..4686812,
    4686812..4687421,
    4687421..4688021,
    4688021..4688619,
    4688619..4689231,
    4690159..4691069,
    4691069..4691667,
    4691667..4692277,
    4692277..4692878,
    4692878..4693493,
    4693493..4694104,
    4694104..4694701,
    4695529..4696421,
    4698629..4700048,
    4700048..4700648,
    4700648..4701246,
    4702176..4703102,
    4704203..4704827,
    4704827..4705436,
    4705436..4706051,
    4706051..4706659,
    4706659..4707272,
    4707272..4707890,
    4707890..4708490,
    4708490..4709108,
    4709645..4710489,
    4710489..4711107,
    4711107..4711705,
    4712787..4713789,
    4713789..4714387,
    4714387..4715008,
    4715008..4715644,
    4715644..4716262,
    4716262..4716859,
    4716859..4717481,
    4717481..4718085,
    4718085..4718688,
    4718688..4719307,
    4719307..4719929,
    4719929..4720539,
    4720539..4721155,
    4722100..4723349,
    4723349..4723970,
    4723970..4724582,
    4724582..4725215,
    4725215..4725821,
    4725821..4726430,
    4726430..4727042,
    4727042..4727646,
    4727646..4728241,
    4728241..4728853,
    4728853..4729453,
    4730471..4731432,
    4732530..4733150,
    4734092..4735024,
    4735024..4735652,
    4735652..4736267,
    4736267..4736891,
    4736891..4737495,
    4738199..4738991,
    4739784..4740390,
    4741302..4742210,
    4742210..4742816,
    4742816..4743444,
    4744166..4744979,
    4746055..4747053,
    4747053..4747667,
    4747667..4748284,
    4748284..4748901,
    4748901..4749514,
    4749514..4750132,
    4750132..4750741,
    4750741..4751336,
    4751336..4751954,
    4751954..4752561,
    4752561..4753188,
    4753188..4753798,
    4753798..4754401,
];
pub static RUNE_CLOSURES: [[[Range<usize>; 2]; 2]; RuneId::VARIANTS] = [
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4678399..4678921, 0..0], [4678921..4679443, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [
        [4681854..4682318, 4682318..4682783],
        [4682783..4683247, 4683247..4683712],
    ],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4689695..4690159, 0..0], [4689231..4689695, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4695115..4695529, 0..0], [4694701..4695115, 0..0]],
    [
        [4696421..4697001, 4697001..4697525],
        [4697525..4698105, 4698105..4698629],
    ],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4701711..4702176, 0..0], [4701246..4701711, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4709108..4709376, 0..0], [4709376..4709645, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4712246..4712787, 0..0], [4711705..4712246, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4721155..4721627, 0..0], [4721627..4722100, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4729453..4729962, 0..0], [4729962..4730471, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4733150..4733621, 0..0], [4733621..4734092, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4737847..4738199, 0..0], [4737495..4737847, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4740390..4740846, 0..0], [4740846..4741302, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[4743444..4743805, 0..0], [4743805..4744166, 0..0]],
    [[4744979..4745517, 0..0], [4745517..4746055, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
    [[0..0, 0..0], [0..0, 0..0]],
];

pub static ABSOLUTE_FOCUS: Rune = Rune {
    name: "Absolute Focus",
    metadata: TypeMetadata {
        kind: RuneId::AbsoluteFocus,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8233,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static ABSORB_LIFE: Rune = Rune {
    name: "Absorb Life",
    metadata: TypeMetadata {
        kind: RuneId::AbsorbLife,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 9101,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static ADAPTIVE_FORCE: Rune = Rune {
    name: "Adaptive Force Shard",
    metadata: TypeMetadata {
        kind: RuneId::AdaptiveForce,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8982,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static AFTERSHOCK: Rune = Rune {
    name: "Aftershock",
    metadata: TypeMetadata {
        kind: RuneId::Aftershock,
        damage_type: Magic,
        attributes: Undefined,
    },
    melee: [aftershock_min, zero],
    ranged: [aftershock_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8439,
    identifiers: [
        [
            &[BonusHealth, Level, MagicMultiplier] as &[_],
            &[MagicMultiplier],
        ],
        [
            &[BonusHealth, Level, MagicMultiplier] as &[_],
            &[MagicMultiplier],
        ],
    ],
};

pub const fn aftershock_min(ctx: &Ctx) -> f32 {
    5f32 * (5f32
        + 0.016 * ctx.bonus_health
        + 1.1179999999999999 * (-1f32 + ctx.level))
}

pub static APPROACH_VELOCITY: Rune = Rune {
    name: "Approach Velocity",
    metadata: TypeMetadata {
        kind: RuneId::ApproachVelocity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8410,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static ARCANE_COMET: Rune = Rune {
    name: "Arcane Comet",
    metadata: TypeMetadata {
        kind: RuneId::ArcaneComet,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [arcane_comet_min, arcane_comet_max],
    ranged: [arcane_comet_min, arcane_comet_max],
    deals_damage: [true, true, true, true],
    riot_id: 8229,
    identifiers: [
        [
            &[AbilityPower, BonusAd, Level] as &[_],
            &[AbilityPower, BonusAd, Level],
        ],
        [
            &[AbilityPower, BonusAd, Level] as &[_],
            &[AbilityPower, BonusAd, Level],
        ],
    ],
};

pub const fn arcane_comet_min(ctx: &Ctx) -> f32 {
    5f32 * (2f32 + 0.01 * ctx.ability_power + 0.02 * ctx.bonus_ad + ctx.level)
}

pub const fn arcane_comet_max(ctx: &Ctx) -> f32 {
    10f32 * (2f32 + 0.01 * ctx.ability_power + 0.02 * ctx.bonus_ad + ctx.level)
}

pub static ATTACK_SPEED: Rune = Rune {
    name: "Attack Speed Shard",
    metadata: TypeMetadata {
        kind: RuneId::AttackSpeed,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8983,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static AXIOM_ARCANIST: Rune = Rune {
    name: "Axiom Arcanist",
    metadata: TypeMetadata {
        kind: RuneId::AxiomArcanist,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8224,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static BISCUIT_DELIVERY: Rune = Rune {
    name: "Biscuit Delivery",
    metadata: TypeMetadata {
        kind: RuneId::BiscuitDelivery,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8345,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static BONE_PLATING: Rune = Rune {
    name: "Bone Plating",
    metadata: TypeMetadata {
        kind: RuneId::BonePlating,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8473,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CASH_BACK: Rune = Rune {
    name: "Cash Back",
    metadata: TypeMetadata {
        kind: RuneId::CashBack,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8321,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CELERITY: Rune = Rune {
    name: "Celerity",
    metadata: TypeMetadata {
        kind: RuneId::Celerity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8234,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CELESTIAL_BODY: Rune = Rune {
    name: "Celestial Body",
    metadata: TypeMetadata {
        kind: RuneId::CelestialBody,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CHEAP_SHOT: Rune = Rune {
    name: "Cheap Shot",
    metadata: TypeMetadata {
        kind: RuneId::CheapShot,
        damage_type: True,
        attributes: Undefined,
    },
    melee: [cheap_shot_min, zero],
    ranged: [cheap_shot_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8126,
    identifiers: [[&[Level] as &[_], &[]], [&[Level] as &[_], &[]]],
};

pub const fn cheap_shot_min(ctx: &Ctx) -> f32 {
    5f32 * (34f32 + 7f32 * (-1f32 + ctx.level)) / 17f32
}

pub static CHRYSALIS: Rune = Rune {
    name: "Chrysalis",
    metadata: TypeMetadata {
        kind: RuneId::Chrysalis,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CONDITIONING: Rune = Rune {
    name: "Conditioning",
    metadata: TypeMetadata {
        kind: RuneId::Conditioning,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8429,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CONQUEROR: Rune = Rune {
    name: "Conqueror",
    metadata: TypeMetadata {
        kind: RuneId::Conqueror,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8010,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static COSMIC_INSIGHT: Rune = Rune {
    name: "Cosmic Insight",
    metadata: TypeMetadata {
        kind: RuneId::CosmicInsight,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8347,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static COUP_DE_GRACE: Rune = Rune {
    name: "Coup de Grace",
    metadata: TypeMetadata {
        kind: RuneId::CoupDeGrace,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8014,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static CUT_DOWN: Rune = Rune {
    name: "Cut Down",
    metadata: TypeMetadata {
        kind: RuneId::CutDown,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8017,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static DARK_HARVEST: Rune = Rune {
    name: "Dark Harvest",
    metadata: TypeMetadata {
        kind: RuneId::DarkHarvest,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [dark_harvest_min, zero],
    ranged: [dark_harvest_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8128,
    identifiers: [
        [&[AbilityPower, BonusAd, Stacks] as &[_], &[]],
        [&[AbilityPower, BonusAd, Stacks] as &[_], &[]],
    ],
};

pub const fn dark_harvest_min(ctx: &Ctx) -> f32 {
    30f32 + 0.05 * ctx.ability_power + 0.1 * ctx.bonus_ad + 11f32 * ctx.stacks
}

pub static DEATHFIRE_TOUCH: Rune = Rune {
    name: "Deathfire Touch",
    metadata: TypeMetadata {
        kind: RuneId::DeathfireTouch,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [deathfire_touch_min, deathfire_touch_max],
    ranged: [deathfire_touch_min, deathfire_touch_max],
    deals_damage: [true, true, true, true],
    riot_id: 8992,
    identifiers: [
        [
            &[AbilityPower, BonusAd, Level] as &[_],
            &[AbilityPower, BonusAd, Level],
        ],
        [
            &[AbilityPower, BonusAd, Level] as &[_],
            &[AbilityPower, BonusAd, Level],
        ],
    ],
};

pub const fn deathfire_touch_min(ctx: &Ctx) -> f32 {
    0.0125 * ctx.ability_power
        + 0.035 * ctx.bonus_ad
        + 3f32 / 2f32
        + 9f32 * (-1f32 + ctx.level) / 34f32
}

pub const fn deathfire_touch_max(ctx: &Ctx) -> f32 {
    2.63 + 0.02188 * ctx.ability_power
        + 0.06125 * ctx.bonus_ad
        + 0.46 * (-1f32 + ctx.level)
}

pub static DEEP_WARD: Rune = Rune {
    name: "Deep Ward",
    metadata: TypeMetadata {
        kind: RuneId::DeepWard,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8141,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static DEMOLISH: Rune = Rune {
    name: "Demolish",
    metadata: TypeMetadata {
        kind: RuneId::Demolish,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8446,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static ELECTROCUTE: Rune = Rune {
    name: "Electrocute",
    metadata: TypeMetadata {
        kind: RuneId::Electrocute,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [electrocute_min, zero],
    ranged: [electrocute_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8112,
    identifiers: [
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
    ],
};

pub const fn electrocute_min(ctx: &Ctx) -> f32 {
    10f32 * (6f32 + 0.005 * ctx.ability_power + 0.01 * ctx.bonus_ad + ctx.level)
}

pub static EYEBALL_COLLECTION: Rune = Rune {
    name: "Eyeball Collection",
    metadata: TypeMetadata {
        kind: RuneId::EyeballCollection,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static FIRST_STRIKE: Rune = Rune {
    name: "First Strike",
    metadata: TypeMetadata {
        kind: RuneId::FirstStrike,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8369,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static FLEET_FOOTWORK: Rune = Rune {
    name: "Fleet Footwork",
    metadata: TypeMetadata {
        kind: RuneId::FleetFootwork,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8021,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static FONT_OF_LIFE: Rune = Rune {
    name: "Font of Life",
    metadata: TypeMetadata {
        kind: RuneId::FontOfLife,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8463,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static FUTURES_MARKET: Rune = Rune {
    name: "Future's Market",
    metadata: TypeMetadata {
        kind: RuneId::FuturesMarket,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static GATHERING_STORM: Rune = Rune {
    name: "Gathering Storm",
    metadata: TypeMetadata {
        kind: RuneId::GatheringStorm,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8236,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static GHOST_PORO: Rune = Rune {
    name: "Ghost Poro",
    metadata: TypeMetadata {
        kind: RuneId::GhostPoro,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static GLACIAL_AUGMENT: Rune = Rune {
    name: "Glacial Augment",
    metadata: TypeMetadata {
        kind: RuneId::GlacialAugment,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8351,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static GRASP_OF_THE_UNDYING: Rune = Rune {
    name: "Grasp of the Undying",
    metadata: TypeMetadata {
        kind: RuneId::GraspOfTheUndying,
        damage_type: Magic,
        attributes: Undefined,
    },
    melee: [grasp_of_the_undying_melee_min, zero],
    ranged: [grasp_of_the_undying_ranged_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8437,
    identifiers: [
        [&[MagicMultiplier, MaxHealth] as &[_], &[MagicMultiplier]],
        [&[MagicMultiplier, MaxHealth] as &[_], &[MagicMultiplier]],
    ],
};

pub const fn grasp_of_the_undying_melee_min(ctx: &Ctx) -> f32 {
    3.5 * ctx.max_health
}

pub const fn grasp_of_the_undying_ranged_min(ctx: &Ctx) -> f32 {
    1.4 * ctx.max_health
}

pub static GRISLY_MEMENTOS: Rune = Rune {
    name: "Grisly Mementos",
    metadata: TypeMetadata {
        kind: RuneId::GrislyMementos,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8140,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static GUARDIAN: Rune = Rune {
    name: "Guardian",
    metadata: TypeMetadata {
        kind: RuneId::Guardian,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8465,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static HAIL_OF_BLADES: Rune = Rune {
    name: "Hail of Blades",
    metadata: TypeMetadata {
        kind: RuneId::HailOfBlades,
        damage_type: True,
        attributes: Undefined,
    },
    melee: [hail_of_blades_min, zero],
    ranged: [hail_of_blades_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 9923,
    identifiers: [
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
    ],
};

pub const fn hail_of_blades_min(ctx: &Ctx) -> f32 {
    4f32 + 0.06 * ctx.ability_power
        + 0.08 * ctx.bonus_ad
        + 16f32 * (-1f32 + ctx.level) / 17f32
}

pub static HEALTH: Rune = Rune {
    name: "Health Shard",
    metadata: TypeMetadata {
        kind: RuneId::Health,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8980,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static HEALTH_SCALING: Rune = Rune {
    name: "Health Scaling Shard",
    metadata: TypeMetadata {
        kind: RuneId::HealthScaling,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8981,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static HEXTECH_FLASHTRAPTION: Rune = Rune {
    name: "Hextech Flashtraption",
    metadata: TypeMetadata {
        kind: RuneId::HextechFlashtraption,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8306,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static INGENIOUS_HUNTER: Rune = Rune {
    name: "Ingenious Hunter",
    metadata: TypeMetadata {
        kind: RuneId::IngeniousHunter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static IRON_SKIN: Rune = Rune {
    name: "Iron Skin",
    metadata: TypeMetadata {
        kind: RuneId::IronSkin,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static JACK_OF_ALL_TRADES: Rune = Rune {
    name: "Jack of All Trades",
    metadata: TypeMetadata {
        kind: RuneId::JackOfAllTrades,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static KLEPTOMANCY: Rune = Rune {
    name: "Kleptomancy",
    metadata: TypeMetadata {
        kind: RuneId::Kleptomancy,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static LAST_STAND: Rune = Rune {
    name: "Last Stand",
    metadata: TypeMetadata {
        kind: RuneId::LastStand,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8299,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static LEGEND_ALACRITY: Rune = Rune {
    name: "Legend: Alacrity",
    metadata: TypeMetadata {
        kind: RuneId::LegendAlacrity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 9104,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static LEGEND_BLOODLINE: Rune = Rune {
    name: "Legend: Bloodline",
    metadata: TypeMetadata {
        kind: RuneId::LegendBloodline,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 9103,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static LEGEND_HASTE: Rune = Rune {
    name: "Legend: Haste",
    metadata: TypeMetadata {
        kind: RuneId::LegendHaste,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 9105,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static LEGEND_TENACITY: Rune = Rune {
    name: "Legend: Tenacity",
    metadata: TypeMetadata {
        kind: RuneId::LegendTenacity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static LETHAL_TEMPO: Rune = Rune {
    name: "Lethal Tempo",
    metadata: TypeMetadata {
        kind: RuneId::LethalTempo,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [lethal_tempo_melee_min, zero],
    ranged: [lethal_tempo_ranged_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8008,
    identifiers: [[&[Level] as &[_], &[]], [&[Level] as &[_], &[]]],
};

pub const fn lethal_tempo_melee_min(ctx: &Ctx) -> f32 {
    3f32 * (51f32 + 7f32 * (-1f32 + ctx.level)) / 17f32
}

pub const fn lethal_tempo_ranged_min(ctx: &Ctx) -> f32 {
    2f32 * (51f32 + 7f32 * (-1f32 + ctx.level)) / 17f32
}

pub static MAGICAL_FOOTWEAR: Rune = Rune {
    name: "Magical Footwear",
    metadata: TypeMetadata {
        kind: RuneId::MagicalFootwear,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8304,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static MANAFLOW_BAND: Rune = Rune {
    name: "Manaflow Band",
    metadata: TypeMetadata {
        kind: RuneId::ManaflowBand,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8226,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static MINION_DEMATERIALIZER: Rune = Rune {
    name: "Minion Dematerializer",
    metadata: TypeMetadata {
        kind: RuneId::MinionDematerializer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static MIRROR_SHELL: Rune = Rune {
    name: "Mirror Shell",
    metadata: TypeMetadata {
        kind: RuneId::MirrorShell,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static NIMBUS_CLOAK: Rune = Rune {
    name: "Nimbus Cloak",
    metadata: TypeMetadata {
        kind: RuneId::NimbusCloak,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8275,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static NULLIFYING_ORB: Rune = Rune {
    name: "Nullifying Orb",
    metadata: TypeMetadata {
        kind: RuneId::NullifyingOrb,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static OVERGROWTH: Rune = Rune {
    name: "Overgrowth",
    metadata: TypeMetadata {
        kind: RuneId::Overgrowth,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8451,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static OVERHEAL: Rune = Rune {
    name: "Overheal",
    metadata: TypeMetadata {
        kind: RuneId::Overheal,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static PERFECT_TIMING: Rune = Rune {
    name: "Perfect Timing",
    metadata: TypeMetadata {
        kind: RuneId::PerfectTiming,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static PHASE_RUSH: Rune = Rune {
    name: "Phase Rush",
    metadata: TypeMetadata {
        kind: RuneId::PhaseRush,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static PREDATOR: Rune = Rune {
    name: "Predator",
    metadata: TypeMetadata {
        kind: RuneId::Predator,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [predator_min, zero],
    ranged: [predator_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 0,
    identifiers: [
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
    ],
};

pub const fn predator_min(ctx: &Ctx) -> f32 {
    20f32
        + 0.15 * ctx.ability_power
        + 0.25 * ctx.bonus_ad
        + 9.41 * (-1f32 + ctx.level)
}

pub static PRESENCE_OF_MIND: Rune = Rune {
    name: "Presence of Mind",
    metadata: TypeMetadata {
        kind: RuneId::PresenceOfMind,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8009,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static PRESS_THE_ATTACK: Rune = Rune {
    name: "Press the Attack",
    metadata: TypeMetadata {
        kind: RuneId::PressTheAttack,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [press_the_attack_min, zero],
    ranged: [press_the_attack_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8005,
    identifiers: [[&[Level] as &[_], &[]], [&[Level] as &[_], &[]]],
};

pub const fn press_the_attack_min(ctx: &Ctx) -> f32 {
    40f32 * (17f32 + 3f32 * (-1f32 + ctx.level)) / 17f32
}

pub static PROTOTYPE_OMNISTONE: Rune = Rune {
    name: "Prototype: Omnistone",
    metadata: TypeMetadata {
        kind: RuneId::PrototypeOmnistone,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static RAVENOUS_HUNTER: Rune = Rune {
    name: "Ravenous Hunter",
    metadata: TypeMetadata {
        kind: RuneId::RavenousHunter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static RELENTLESS_HUNTER: Rune = Rune {
    name: "Relentless Hunter",
    metadata: TypeMetadata {
        kind: RuneId::RelentlessHunter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8105,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static REVITALIZE: Rune = Rune {
    name: "Revitalize",
    metadata: TypeMetadata {
        kind: RuneId::Revitalize,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8453,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static SCORCH: Rune = Rune {
    name: "Scorch",
    metadata: TypeMetadata {
        kind: RuneId::Scorch,
        damage_type: Magic,
        attributes: Undefined,
    },
    melee: [scorch_min, zero],
    ranged: [scorch_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8237,
    identifiers: [
        [&[Level, MagicMultiplier] as &[_], &[MagicMultiplier]],
        [&[Level, MagicMultiplier] as &[_], &[MagicMultiplier]],
    ],
};

pub const fn scorch_min(ctx: &Ctx) -> f32 {
    20f32 * (16f32 + ctx.level) / 17f32
}

pub static SECOND_WIND: Rune = Rune {
    name: "Second Wind",
    metadata: TypeMetadata {
        kind: RuneId::SecondWind,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8444,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static SHIELD_BASH: Rune = Rune {
    name: "Shield Bash",
    metadata: TypeMetadata {
        kind: RuneId::ShieldBash,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [shield_bash_min, zero],
    ranged: [shield_bash_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8401,
    identifiers: [
        [&[BonusHealth, Level] as &[_], &[]],
        [&[BonusHealth, Level] as &[_], &[]],
    ],
};

pub const fn shield_bash_min(ctx: &Ctx) -> f32 {
    5f32 + 0.025 * ctx.bonus_health + 25f32 * (-1f32 + ctx.level) / 17f32
}

pub static SIXTH_SENSE: Rune = Rune {
    name: "Sixth Sense",
    metadata: TypeMetadata {
        kind: RuneId::SixthSense,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8137,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static STORMRAIDERS_SURGE: Rune = Rune {
    name: "Stormraider's Surge",
    metadata: TypeMetadata {
        kind: RuneId::StormraidersSurge,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8230,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static SUDDEN_IMPACT: Rune = Rune {
    name: "Sudden Impact",
    metadata: TypeMetadata {
        kind: RuneId::SuddenImpact,
        damage_type: True,
        attributes: Undefined,
    },
    melee: [sudden_impact_min, zero],
    ranged: [sudden_impact_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8143,
    identifiers: [[&[Level] as &[_], &[]], [&[Level] as &[_], &[]]],
};

pub const fn sudden_impact_min(ctx: &Ctx) -> f32 {
    20f32 + 3.53 * (-1f32 + ctx.level)
}

pub static SUMMON_AERY: Rune = Rune {
    name: "Summon Aery",
    metadata: TypeMetadata {
        kind: RuneId::SummonAery,
        damage_type: Adaptive,
        attributes: Undefined,
    },
    melee: [summon_aery_min, zero],
    ranged: [summon_aery_min, zero],
    deals_damage: [true, false, true, false],
    riot_id: 8214,
    identifiers: [
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
        [&[AbilityPower, BonusAd, Level] as &[_], &[]],
    ],
};

pub const fn summon_aery_min(ctx: &Ctx) -> f32 {
    10f32
        + 0.05 * ctx.ability_power
        + 0.1 * ctx.bonus_ad
        + 40f32 * (-1f32 + ctx.level) / 17f32
}

pub static TASTE_OF_BLOOD: Rune = Rune {
    name: "Taste of Blood",
    metadata: TypeMetadata {
        kind: RuneId::TasteOfBlood,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8139,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static THE_ULTIMATE_HAT: Rune = Rune {
    name: "The Ultimate Hat",
    metadata: TypeMetadata {
        kind: RuneId::TheUltimateHat,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static TIME_WARP_TONIC: Rune = Rune {
    name: "Time Warp Tonic",
    metadata: TypeMetadata {
        kind: RuneId::TimeWarpTonic,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8352,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static TRANSCENDENCE: Rune = Rune {
    name: "Transcendence",
    metadata: TypeMetadata {
        kind: RuneId::Transcendence,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8210,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static TREASURE_HUNTER: Rune = Rune {
    name: "Treasure Hunter",
    metadata: TypeMetadata {
        kind: RuneId::TreasureHunter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8135,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static TRIPLE_TONIC: Rune = Rune {
    name: "Triple Tonic",
    metadata: TypeMetadata {
        kind: RuneId::TripleTonic,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8313,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static TRIUMPH: Rune = Rune {
    name: "Triumph",
    metadata: TypeMetadata {
        kind: RuneId::Triumph,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 9111,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static ULTIMATE_HUNTER: Rune = Rune {
    name: "Ultimate Hunter",
    metadata: TypeMetadata {
        kind: RuneId::UltimateHunter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8106,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static UNFLINCHING: Rune = Rune {
    name: "Unflinching",
    metadata: TypeMetadata {
        kind: RuneId::Unflinching,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8242,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static UNSEALED_SPELLBOOK: Rune = Rune {
    name: "Unsealed Spellbook",
    metadata: TypeMetadata {
        kind: RuneId::UnsealedSpellbook,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8360,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static WATERWALKING: Rune = Rune {
    name: "Waterwalking",
    metadata: TypeMetadata {
        kind: RuneId::Waterwalking,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 8232,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub static ZOMBIE_WARD: Rune = Rune {
    name: "Zombie Ward",
    metadata: TypeMetadata {
        kind: RuneId::ZombieWard,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    melee: [zero, zero],
    ranged: [zero, zero],
    deals_damage: [false, false, false, false],
    riot_id: 0,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

#[derive(
    Clone,
    Copy,
    Debug,
    Decode,
    Deserialize,
    Eq,
    Encode,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(u8)]
pub enum RuneId {
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
    CelestialBody,
    CheapShot,
    Chrysalis,
    Conditioning,
    Conqueror,
    CosmicInsight,
    CoupDeGrace,
    CutDown,
    DarkHarvest,
    DeathfireTouch,
    DeepWard,
    Demolish,
    Electrocute,
    EyeballCollection,
    FirstStrike,
    FleetFootwork,
    FontOfLife,
    FuturesMarket,
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
    IngeniousHunter,
    IronSkin,
    JackOfAllTrades,
    Kleptomancy,
    LastStand,
    LegendAlacrity,
    LegendBloodline,
    LegendHaste,
    LegendTenacity,
    LethalTempo,
    MagicalFootwear,
    ManaflowBand,
    MinionDematerializer,
    MirrorShell,
    NimbusCloak,
    NullifyingOrb,
    Overgrowth,
    Overheal,
    PerfectTiming,
    PhaseRush,
    Predator,
    PresenceOfMind,
    PressTheAttack,
    PrototypeOmnistone,
    RavenousHunter,
    RelentlessHunter,
    Revitalize,
    Scorch,
    SecondWind,
    ShieldBash,
    SixthSense,
    StormraidersSurge,
    SuddenImpact,
    SummonAery,
    TasteOfBlood,
    TheUltimateHat,
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
    pub const VARIANTS: usize = 86;
    pub const fn debug(&self) -> &'static str {
        match self {
            Self::AbsoluteFocus => "AbsoluteFocus",
            Self::AbsorbLife => "AbsorbLife",
            Self::AdaptiveForce => "AdaptiveForce",
            Self::Aftershock => "Aftershock",
            Self::ApproachVelocity => "ApproachVelocity",
            Self::ArcaneComet => "ArcaneComet",
            Self::AttackSpeed => "AttackSpeed",
            Self::AxiomArcanist => "AxiomArcanist",
            Self::BiscuitDelivery => "BiscuitDelivery",
            Self::BonePlating => "BonePlating",
            Self::CashBack => "CashBack",
            Self::Celerity => "Celerity",
            Self::CelestialBody => "CelestialBody",
            Self::CheapShot => "CheapShot",
            Self::Chrysalis => "Chrysalis",
            Self::Conditioning => "Conditioning",
            Self::Conqueror => "Conqueror",
            Self::CosmicInsight => "CosmicInsight",
            Self::CoupDeGrace => "CoupDeGrace",
            Self::CutDown => "CutDown",
            Self::DarkHarvest => "DarkHarvest",
            Self::DeathfireTouch => "DeathfireTouch",
            Self::DeepWard => "DeepWard",
            Self::Demolish => "Demolish",
            Self::Electrocute => "Electrocute",
            Self::EyeballCollection => "EyeballCollection",
            Self::FirstStrike => "FirstStrike",
            Self::FleetFootwork => "FleetFootwork",
            Self::FontOfLife => "FontOfLife",
            Self::FuturesMarket => "FuturesMarket",
            Self::GatheringStorm => "GatheringStorm",
            Self::GhostPoro => "GhostPoro",
            Self::GlacialAugment => "GlacialAugment",
            Self::GraspOfTheUndying => "GraspOfTheUndying",
            Self::GrislyMementos => "GrislyMementos",
            Self::Guardian => "Guardian",
            Self::HailOfBlades => "HailOfBlades",
            Self::Health => "Health",
            Self::HealthScaling => "HealthScaling",
            Self::HextechFlashtraption => "HextechFlashtraption",
            Self::IngeniousHunter => "IngeniousHunter",
            Self::IronSkin => "IronSkin",
            Self::JackOfAllTrades => "JackOfAllTrades",
            Self::Kleptomancy => "Kleptomancy",
            Self::LastStand => "LastStand",
            Self::LegendAlacrity => "LegendAlacrity",
            Self::LegendBloodline => "LegendBloodline",
            Self::LegendHaste => "LegendHaste",
            Self::LegendTenacity => "LegendTenacity",
            Self::LethalTempo => "LethalTempo",
            Self::MagicalFootwear => "MagicalFootwear",
            Self::ManaflowBand => "ManaflowBand",
            Self::MinionDematerializer => "MinionDematerializer",
            Self::MirrorShell => "MirrorShell",
            Self::NimbusCloak => "NimbusCloak",
            Self::NullifyingOrb => "NullifyingOrb",
            Self::Overgrowth => "Overgrowth",
            Self::Overheal => "Overheal",
            Self::PerfectTiming => "PerfectTiming",
            Self::PhaseRush => "PhaseRush",
            Self::Predator => "Predator",
            Self::PresenceOfMind => "PresenceOfMind",
            Self::PressTheAttack => "PressTheAttack",
            Self::PrototypeOmnistone => "PrototypeOmnistone",
            Self::RavenousHunter => "RavenousHunter",
            Self::RelentlessHunter => "RelentlessHunter",
            Self::Revitalize => "Revitalize",
            Self::Scorch => "Scorch",
            Self::SecondWind => "SecondWind",
            Self::ShieldBash => "ShieldBash",
            Self::SixthSense => "SixthSense",
            Self::StormraidersSurge => "StormraidersSurge",
            Self::SuddenImpact => "SuddenImpact",
            Self::SummonAery => "SummonAery",
            Self::TasteOfBlood => "TasteOfBlood",
            Self::TheUltimateHat => "TheUltimateHat",
            Self::TimeWarpTonic => "TimeWarpTonic",
            Self::Transcendence => "Transcendence",
            Self::TreasureHunter => "TreasureHunter",
            Self::TripleTonic => "TripleTonic",
            Self::Triumph => "Triumph",
            Self::UltimateHunter => "UltimateHunter",
            Self::Unflinching => "Unflinching",
            Self::UnsealedSpellbook => "UnsealedSpellbook",
            Self::Waterwalking => "Waterwalking",
            Self::ZombieWard => "ZombieWard",
        }
    }
    pub const fn from_riot_id(id: u32) -> Option<Self> {
        match id {
            8233 => Some(Self::AbsoluteFocus),
            9101 => Some(Self::AbsorbLife),
            8982 => Some(Self::AdaptiveForce),
            8439 => Some(Self::Aftershock),
            8410 => Some(Self::ApproachVelocity),
            8229 => Some(Self::ArcaneComet),
            8983 => Some(Self::AttackSpeed),
            8224 => Some(Self::AxiomArcanist),
            8345 => Some(Self::BiscuitDelivery),
            8473 => Some(Self::BonePlating),
            8321 => Some(Self::CashBack),
            8234 => Some(Self::Celerity),
            0 => Some(Self::CelestialBody),
            8126 => Some(Self::CheapShot),
            0 => Some(Self::Chrysalis),
            8429 => Some(Self::Conditioning),
            8010 => Some(Self::Conqueror),
            8347 => Some(Self::CosmicInsight),
            8014 => Some(Self::CoupDeGrace),
            8017 => Some(Self::CutDown),
            8128 => Some(Self::DarkHarvest),
            8992 => Some(Self::DeathfireTouch),
            8141 => Some(Self::DeepWard),
            8446 => Some(Self::Demolish),
            8112 => Some(Self::Electrocute),
            0 => Some(Self::EyeballCollection),
            8369 => Some(Self::FirstStrike),
            8021 => Some(Self::FleetFootwork),
            8463 => Some(Self::FontOfLife),
            0 => Some(Self::FuturesMarket),
            8236 => Some(Self::GatheringStorm),
            0 => Some(Self::GhostPoro),
            8351 => Some(Self::GlacialAugment),
            8437 => Some(Self::GraspOfTheUndying),
            8140 => Some(Self::GrislyMementos),
            8465 => Some(Self::Guardian),
            9923 => Some(Self::HailOfBlades),
            8980 => Some(Self::Health),
            8981 => Some(Self::HealthScaling),
            8306 => Some(Self::HextechFlashtraption),
            0 => Some(Self::IngeniousHunter),
            0 => Some(Self::IronSkin),
            0 => Some(Self::JackOfAllTrades),
            0 => Some(Self::Kleptomancy),
            8299 => Some(Self::LastStand),
            9104 => Some(Self::LegendAlacrity),
            9103 => Some(Self::LegendBloodline),
            9105 => Some(Self::LegendHaste),
            0 => Some(Self::LegendTenacity),
            8008 => Some(Self::LethalTempo),
            8304 => Some(Self::MagicalFootwear),
            8226 => Some(Self::ManaflowBand),
            0 => Some(Self::MinionDematerializer),
            0 => Some(Self::MirrorShell),
            8275 => Some(Self::NimbusCloak),
            0 => Some(Self::NullifyingOrb),
            8451 => Some(Self::Overgrowth),
            0 => Some(Self::Overheal),
            0 => Some(Self::PerfectTiming),
            0 => Some(Self::PhaseRush),
            0 => Some(Self::Predator),
            8009 => Some(Self::PresenceOfMind),
            8005 => Some(Self::PressTheAttack),
            0 => Some(Self::PrototypeOmnistone),
            0 => Some(Self::RavenousHunter),
            8105 => Some(Self::RelentlessHunter),
            8453 => Some(Self::Revitalize),
            8237 => Some(Self::Scorch),
            8444 => Some(Self::SecondWind),
            8401 => Some(Self::ShieldBash),
            8137 => Some(Self::SixthSense),
            8230 => Some(Self::StormraidersSurge),
            8143 => Some(Self::SuddenImpact),
            8214 => Some(Self::SummonAery),
            8139 => Some(Self::TasteOfBlood),
            0 => Some(Self::TheUltimateHat),
            8352 => Some(Self::TimeWarpTonic),
            8210 => Some(Self::Transcendence),
            8135 => Some(Self::TreasureHunter),
            8313 => Some(Self::TripleTonic),
            9111 => Some(Self::Triumph),
            8106 => Some(Self::UltimateHunter),
            8242 => Some(Self::Unflinching),
            8360 => Some(Self::UnsealedSpellbook),
            8232 => Some(Self::Waterwalking),
            0 => Some(Self::ZombieWard),
            _ => None,
        }
    }
}
pub static RUNE_NAME_TO_ID: phf::Map<&str, RuneId> = phf::phf_map!("ABSOLUTE FOCUS" | "ABSOLUTEFOCUS" | "ABSOLUTE_FOCUS" | "Absolute Focus" | "AbsoluteFocus" | "Absolutefocus" | "absolute focus" | "absolute_focus" | "absolutefocus" => RuneId::AbsoluteFocus,"ABSORB LIFE" | "ABSORBLIFE" | "ABSORB_LIFE" | "Absorb Life" | "AbsorbLife" | "Absorblife" | "absorb life" | "absorb_life" | "absorblife" => RuneId::AbsorbLife,"ADAPTIVE FORCE SHARD" | "ADAPTIVEFORCE" | "ADAPTIVEFORCESHARD" | "ADAPTIVE_FORCE" | "ADAPTIVE_FORCE_SHARD" | "Adaptive Force Shard" | "AdaptiveForce" | "AdaptiveForceShard" | "Adaptiveforce" | "adaptive force shard" | "adaptive_force" | "adaptive_force_shard" | "adaptiveforce" | "adaptiveforceshard" => RuneId::AdaptiveForce,"AFTERSHOCK" | "Aftershock" | "aftershock" => RuneId::Aftershock,"APPROACH VELOCITY" | "APPROACHVELOCITY" | "APPROACH_VELOCITY" | "Approach Velocity" | "ApproachVelocity" | "Approachvelocity" | "approach velocity" | "approach_velocity" | "approachvelocity" => RuneId::ApproachVelocity,"ARCANE COMET" | "ARCANECOMET" | "ARCANE_COMET" | "Arcane Comet" | "ArcaneComet" | "Arcanecomet" | "arcane comet" | "arcane_comet" | "arcanecomet" => RuneId::ArcaneComet,"ATTACK SPEED SHARD" | "ATTACKSPEED" | "ATTACKSPEEDSHARD" | "ATTACK_SPEED" | "ATTACK_SPEED_SHARD" | "Attack Speed Shard" | "AttackSpeed" | "AttackSpeedShard" | "Attackspeed" | "attack speed shard" | "attack_speed" | "attack_speed_shard" | "attackspeed" | "attackspeedshard" => RuneId::AttackSpeed,"AXIOM ARCANIST" | "AXIOMARCANIST" | "AXIOM_ARCANIST" | "Axiom Arcanist" | "AxiomArcanist" | "Axiomarcanist" | "axiom arcanist" | "axiom_arcanist" | "axiomarcanist" => RuneId::AxiomArcanist,"BISCUIT DELIVERY" | "BISCUITDELIVERY" | "BISCUIT_DELIVERY" | "Biscuit Delivery" | "BiscuitDelivery" | "Biscuitdelivery" | "biscuit delivery" | "biscuit_delivery" | "biscuitdelivery" => RuneId::BiscuitDelivery,"BONE PLATING" | "BONEPLATING" | "BONE_PLATING" | "Bone Plating" | "BonePlating" | "Boneplating" | "bone plating" | "bone_plating" | "boneplating" => RuneId::BonePlating,"CASH BACK" | "CASHBACK" | "CASH_BACK" | "Cash Back" | "CashBack" | "Cashback" | "cash back" | "cash_back" | "cashback" => RuneId::CashBack,"CELERITY" | "Celerity" | "celerity" => RuneId::Celerity,"CELESTIAL BODY" | "CELESTIALBODY" | "CELESTIAL_BODY" | "Celestial Body" | "CelestialBody" | "Celestialbody" | "celestial body" | "celestial_body" | "celestialbody" => RuneId::CelestialBody,"CHEAP SHOT" | "CHEAPSHOT" | "CHEAP_SHOT" | "Cheap Shot" | "CheapShot" | "Cheapshot" | "cheap shot" | "cheap_shot" | "cheapshot" => RuneId::CheapShot,"CHRYSALIS" | "Chrysalis" | "chrysalis" => RuneId::Chrysalis,"CONDITIONING" | "Conditioning" | "conditioning" => RuneId::Conditioning,"CONQUEROR" | "Conqueror" | "conqueror" => RuneId::Conqueror,"COSMIC INSIGHT" | "COSMICINSIGHT" | "COSMIC_INSIGHT" | "Cosmic Insight" | "CosmicInsight" | "Cosmicinsight" | "cosmic insight" | "cosmic_insight" | "cosmicinsight" => RuneId::CosmicInsight,"COUP DE GRACE" | "COUPDEGRACE" | "COUP_DE_GRACE" | "Coup de Grace" | "CoupDeGrace" | "Coupdegrace" | "coup de grace" | "coup_de_grace" | "coupdegrace" => RuneId::CoupDeGrace,"CUT DOWN" | "CUTDOWN" | "CUT_DOWN" | "Cut Down" | "CutDown" | "Cutdown" | "cut down" | "cut_down" | "cutdown" => RuneId::CutDown,"DARK HARVEST" | "DARKHARVEST" | "DARK_HARVEST" | "Dark Harvest" | "DarkHarvest" | "Darkharvest" | "dark harvest" | "dark_harvest" | "darkharvest" => RuneId::DarkHarvest,"DEATHFIRE TOUCH" | "DEATHFIRETOUCH" | "DEATHFIRE_TOUCH" | "Deathfire Touch" | "DeathfireTouch" | "Deathfiretouch" | "deathfire touch" | "deathfire_touch" | "deathfiretouch" => RuneId::DeathfireTouch,"DEEP WARD" | "DEEPWARD" | "DEEP_WARD" | "Deep Ward" | "DeepWard" | "Deepward" | "deep ward" | "deep_ward" | "deepward" => RuneId::DeepWard,"DEMOLISH" | "Demolish" | "demolish" => RuneId::Demolish,"ELECTROCUTE" | "Electrocute" | "electrocute" => RuneId::Electrocute,"EYEBALL COLLECTION" | "EYEBALLCOLLECTION" | "EYEBALL_COLLECTION" | "Eyeball Collection" | "EyeballCollection" | "Eyeballcollection" | "eyeball collection" | "eyeball_collection" | "eyeballcollection" => RuneId::EyeballCollection,"FIRST STRIKE" | "FIRSTSTRIKE" | "FIRST_STRIKE" | "First Strike" | "FirstStrike" | "Firststrike" | "first strike" | "first_strike" | "firststrike" => RuneId::FirstStrike,"FLEET FOOTWORK" | "FLEETFOOTWORK" | "FLEET_FOOTWORK" | "Fleet Footwork" | "FleetFootwork" | "Fleetfootwork" | "fleet footwork" | "fleet_footwork" | "fleetfootwork" => RuneId::FleetFootwork,"FONT OF LIFE" | "FONTOFLIFE" | "FONT_OF_LIFE" | "Font of Life" | "FontOfLife" | "Fontoflife" | "font of life" | "font_of_life" | "fontoflife" => RuneId::FontOfLife,"FUTURE'S MARKET" | "FUTURESMARKET" | "FUTURES_MARKET" | "Future's Market" | "FuturesMarket" | "Futuresmarket" | "future's market" | "futures_market" | "futuresmarket" => RuneId::FuturesMarket,"GATHERING STORM" | "GATHERINGSTORM" | "GATHERING_STORM" | "Gathering Storm" | "GatheringStorm" | "Gatheringstorm" | "gathering storm" | "gathering_storm" | "gatheringstorm" => RuneId::GatheringStorm,"GHOST PORO" | "GHOSTPORO" | "GHOST_PORO" | "Ghost Poro" | "GhostPoro" | "Ghostporo" | "ghost poro" | "ghost_poro" | "ghostporo" => RuneId::GhostPoro,"GLACIAL AUGMENT" | "GLACIALAUGMENT" | "GLACIAL_AUGMENT" | "Glacial Augment" | "GlacialAugment" | "Glacialaugment" | "glacial augment" | "glacial_augment" | "glacialaugment" => RuneId::GlacialAugment,"GRASP OF THE UNDYING" | "GRASPOFTHEUNDYING" | "GRASP_OF_THE_UNDYING" | "Grasp of the Undying" | "GraspOfTheUndying" | "Graspoftheundying" | "grasp of the undying" | "grasp_of_the_undying" | "graspoftheundying" => RuneId::GraspOfTheUndying,"GRISLY MEMENTOS" | "GRISLYMEMENTOS" | "GRISLY_MEMENTOS" | "Grisly Mementos" | "GrislyMementos" | "Grislymementos" | "grisly mementos" | "grisly_mementos" | "grislymementos" => RuneId::GrislyMementos,"GUARDIAN" | "Guardian" | "guardian" => RuneId::Guardian,"HAIL OF BLADES" | "HAILOFBLADES" | "HAIL_OF_BLADES" | "Hail of Blades" | "HailOfBlades" | "Hailofblades" | "hail of blades" | "hail_of_blades" | "hailofblades" => RuneId::HailOfBlades,"HEALTH" | "HEALTH SHARD" | "HEALTHSHARD" | "HEALTH_SHARD" | "Health" | "Health Shard" | "HealthShard" | "health" | "health shard" | "health_shard" | "healthshard" => RuneId::Health,"HEALTH SCALING SHARD" | "HEALTHSCALING" | "HEALTHSCALINGSHARD" | "HEALTH_SCALING" | "HEALTH_SCALING_SHARD" | "Health Scaling Shard" | "HealthScaling" | "HealthScalingShard" | "Healthscaling" | "health scaling shard" | "health_scaling" | "health_scaling_shard" | "healthscaling" | "healthscalingshard" => RuneId::HealthScaling,"HEXTECH FLASHTRAPTION" | "HEXTECHFLASHTRAPTION" | "HEXTECH_FLASHTRAPTION" | "Hextech Flashtraption" | "HextechFlashtraption" | "Hextechflashtraption" | "hextech flashtraption" | "hextech_flashtraption" | "hextechflashtraption" => RuneId::HextechFlashtraption,"INGENIOUS HUNTER" | "INGENIOUSHUNTER" | "INGENIOUS_HUNTER" | "Ingenious Hunter" | "IngeniousHunter" | "Ingenioushunter" | "ingenious hunter" | "ingenious_hunter" | "ingenioushunter" => RuneId::IngeniousHunter,"IRON SKIN" | "IRONSKIN" | "IRON_SKIN" | "Iron Skin" | "IronSkin" | "Ironskin" | "iron skin" | "iron_skin" | "ironskin" => RuneId::IronSkin,"JACK OF ALL TRADES" | "JACKOFALLTRADES" | "JACK_OF_ALL_TRADES" | "Jack of All Trades" | "JackOfAllTrades" | "Jackofalltrades" | "jack of all trades" | "jack_of_all_trades" | "jackofalltrades" => RuneId::JackOfAllTrades,"KLEPTOMANCY" | "Kleptomancy" | "kleptomancy" => RuneId::Kleptomancy,"LAST STAND" | "LASTSTAND" | "LAST_STAND" | "Last Stand" | "LastStand" | "Laststand" | "last stand" | "last_stand" | "laststand" => RuneId::LastStand,"LEGEND: ALACRITY" | "LEGENDALACRITY" | "LEGEND_ALACRITY" | "Legend: Alacrity" | "LegendAlacrity" | "Legendalacrity" | "legend: alacrity" | "legend_alacrity" | "legendalacrity" => RuneId::LegendAlacrity,"LEGEND: BLOODLINE" | "LEGENDBLOODLINE" | "LEGEND_BLOODLINE" | "Legend: Bloodline" | "LegendBloodline" | "Legendbloodline" | "legend: bloodline" | "legend_bloodline" | "legendbloodline" => RuneId::LegendBloodline,"LEGEND: HASTE" | "LEGENDHASTE" | "LEGEND_HASTE" | "Legend: Haste" | "LegendHaste" | "Legendhaste" | "legend: haste" | "legend_haste" | "legendhaste" => RuneId::LegendHaste,"LEGEND: TENACITY" | "LEGENDTENACITY" | "LEGEND_TENACITY" | "Legend: Tenacity" | "LegendTenacity" | "Legendtenacity" | "legend: tenacity" | "legend_tenacity" | "legendtenacity" => RuneId::LegendTenacity,"LETHAL TEMPO" | "LETHALTEMPO" | "LETHAL_TEMPO" | "Lethal Tempo" | "LethalTempo" | "Lethaltempo" | "lethal tempo" | "lethal_tempo" | "lethaltempo" => RuneId::LethalTempo,"MAGICAL FOOTWEAR" | "MAGICALFOOTWEAR" | "MAGICAL_FOOTWEAR" | "Magical Footwear" | "MagicalFootwear" | "Magicalfootwear" | "magical footwear" | "magical_footwear" | "magicalfootwear" => RuneId::MagicalFootwear,"MANAFLOW BAND" | "MANAFLOWBAND" | "MANAFLOW_BAND" | "Manaflow Band" | "ManaflowBand" | "Manaflowband" | "manaflow band" | "manaflow_band" | "manaflowband" => RuneId::ManaflowBand,"MINION DEMATERIALIZER" | "MINIONDEMATERIALIZER" | "MINION_DEMATERIALIZER" | "Minion Dematerializer" | "MinionDematerializer" | "Miniondematerializer" | "minion dematerializer" | "minion_dematerializer" | "miniondematerializer" => RuneId::MinionDematerializer,"MIRROR SHELL" | "MIRRORSHELL" | "MIRROR_SHELL" | "Mirror Shell" | "MirrorShell" | "Mirrorshell" | "mirror shell" | "mirror_shell" | "mirrorshell" => RuneId::MirrorShell,"NIMBUS CLOAK" | "NIMBUSCLOAK" | "NIMBUS_CLOAK" | "Nimbus Cloak" | "NimbusCloak" | "Nimbuscloak" | "nimbus cloak" | "nimbus_cloak" | "nimbuscloak" => RuneId::NimbusCloak,"NULLIFYING ORB" | "NULLIFYINGORB" | "NULLIFYING_ORB" | "Nullifying Orb" | "NullifyingOrb" | "Nullifyingorb" | "nullifying orb" | "nullifying_orb" | "nullifyingorb" => RuneId::NullifyingOrb,"OVERGROWTH" | "Overgrowth" | "overgrowth" => RuneId::Overgrowth,"OVERHEAL" | "Overheal" | "overheal" => RuneId::Overheal,"PERFECT TIMING" | "PERFECTTIMING" | "PERFECT_TIMING" | "Perfect Timing" | "PerfectTiming" | "Perfecttiming" | "perfect timing" | "perfect_timing" | "perfecttiming" => RuneId::PerfectTiming,"PHASE RUSH" | "PHASERUSH" | "PHASE_RUSH" | "Phase Rush" | "PhaseRush" | "Phaserush" | "phase rush" | "phase_rush" | "phaserush" => RuneId::PhaseRush,"PREDATOR" | "Predator" | "predator" => RuneId::Predator,"PRESENCE OF MIND" | "PRESENCEOFMIND" | "PRESENCE_OF_MIND" | "Presence of Mind" | "PresenceOfMind" | "Presenceofmind" | "presence of mind" | "presence_of_mind" | "presenceofmind" => RuneId::PresenceOfMind,"PRESS THE ATTACK" | "PRESSTHEATTACK" | "PRESS_THE_ATTACK" | "Press the Attack" | "PressTheAttack" | "Presstheattack" | "press the attack" | "press_the_attack" | "presstheattack" => RuneId::PressTheAttack,"PROTOTYPE: OMNISTONE" | "PROTOTYPEOMNISTONE" | "PROTOTYPE_OMNISTONE" | "Prototype: Omnistone" | "PrototypeOmnistone" | "Prototypeomnistone" | "prototype: omnistone" | "prototype_omnistone" | "prototypeomnistone" => RuneId::PrototypeOmnistone,"RAVENOUS HUNTER" | "RAVENOUSHUNTER" | "RAVENOUS_HUNTER" | "Ravenous Hunter" | "RavenousHunter" | "Ravenoushunter" | "ravenous hunter" | "ravenous_hunter" | "ravenoushunter" => RuneId::RavenousHunter,"RELENTLESS HUNTER" | "RELENTLESSHUNTER" | "RELENTLESS_HUNTER" | "Relentless Hunter" | "RelentlessHunter" | "Relentlesshunter" | "relentless hunter" | "relentless_hunter" | "relentlesshunter" => RuneId::RelentlessHunter,"REVITALIZE" | "Revitalize" | "revitalize" => RuneId::Revitalize,"SCORCH" | "Scorch" | "scorch" => RuneId::Scorch,"SECOND WIND" | "SECONDWIND" | "SECOND_WIND" | "Second Wind" | "SecondWind" | "Secondwind" | "second wind" | "second_wind" | "secondwind" => RuneId::SecondWind,"SHIELD BASH" | "SHIELDBASH" | "SHIELD_BASH" | "Shield Bash" | "ShieldBash" | "Shieldbash" | "shield bash" | "shield_bash" | "shieldbash" => RuneId::ShieldBash,"SIXTH SENSE" | "SIXTHSENSE" | "SIXTH_SENSE" | "Sixth Sense" | "SixthSense" | "Sixthsense" | "sixth sense" | "sixth_sense" | "sixthsense" => RuneId::SixthSense,"STORMRAIDER'S SURGE" | "STORMRAIDERSSURGE" | "STORMRAIDERS_SURGE" | "Stormraider's Surge" | "StormraidersSurge" | "Stormraiderssurge" | "stormraider's surge" | "stormraiders_surge" | "stormraiderssurge" => RuneId::StormraidersSurge,"SUDDEN IMPACT" | "SUDDENIMPACT" | "SUDDEN_IMPACT" | "Sudden Impact" | "SuddenImpact" | "Suddenimpact" | "sudden impact" | "sudden_impact" | "suddenimpact" => RuneId::SuddenImpact,"SUMMON AERY" | "SUMMONAERY" | "SUMMON_AERY" | "Summon Aery" | "SummonAery" | "Summonaery" | "summon aery" | "summon_aery" | "summonaery" => RuneId::SummonAery,"TASTE OF BLOOD" | "TASTEOFBLOOD" | "TASTE_OF_BLOOD" | "Taste of Blood" | "TasteOfBlood" | "Tasteofblood" | "taste of blood" | "taste_of_blood" | "tasteofblood" => RuneId::TasteOfBlood,"THE ULTIMATE HAT" | "THEULTIMATEHAT" | "THE_ULTIMATE_HAT" | "The Ultimate Hat" | "TheUltimateHat" | "Theultimatehat" | "the ultimate hat" | "the_ultimate_hat" | "theultimatehat" => RuneId::TheUltimateHat,"TIME WARP TONIC" | "TIMEWARPTONIC" | "TIME_WARP_TONIC" | "Time Warp Tonic" | "TimeWarpTonic" | "Timewarptonic" | "time warp tonic" | "time_warp_tonic" | "timewarptonic" => RuneId::TimeWarpTonic,"TRANSCENDENCE" | "Transcendence" | "transcendence" => RuneId::Transcendence,"TREASURE HUNTER" | "TREASUREHUNTER" | "TREASURE_HUNTER" | "Treasure Hunter" | "TreasureHunter" | "Treasurehunter" | "treasure hunter" | "treasure_hunter" | "treasurehunter" => RuneId::TreasureHunter,"TRIPLE TONIC" | "TRIPLETONIC" | "TRIPLE_TONIC" | "Triple Tonic" | "TripleTonic" | "Tripletonic" | "triple tonic" | "triple_tonic" | "tripletonic" => RuneId::TripleTonic,"TRIUMPH" | "Triumph" | "triumph" => RuneId::Triumph,"ULTIMATE HUNTER" | "ULTIMATEHUNTER" | "ULTIMATE_HUNTER" | "Ultimate Hunter" | "UltimateHunter" | "Ultimatehunter" | "ultimate hunter" | "ultimate_hunter" | "ultimatehunter" => RuneId::UltimateHunter,"UNFLINCHING" | "Unflinching" | "unflinching" => RuneId::Unflinching,"UNSEALED SPELLBOOK" | "UNSEALEDSPELLBOOK" | "UNSEALED_SPELLBOOK" | "Unsealed Spellbook" | "UnsealedSpellbook" | "Unsealedspellbook" | "unsealed spellbook" | "unsealed_spellbook" | "unsealedspellbook" => RuneId::UnsealedSpellbook,"WATERWALKING" | "Waterwalking" | "waterwalking" => RuneId::Waterwalking,"ZOMBIE WARD" | "ZOMBIEWARD" | "ZOMBIE_WARD" | "Zombie Ward" | "ZombieWard" | "Zombieward" | "zombie ward" | "zombie_ward" | "zombieward" => RuneId::ZombieWard);
pub const fn rune_const_eval(
    ctx: &Ctx,
    rune_id: RuneId,
    attack_type: AttackType,
) -> [f32; 2] {
    match rune_id {
        RuneId::AbsoluteFocus => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::AbsorbLife => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::AdaptiveForce => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Aftershock => match attack_type {
            Melee => [aftershock_min(&ctx), zero(&ctx)],
            Ranged => [aftershock_min(&ctx), zero(&ctx)],
        },

        RuneId::ApproachVelocity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::ArcaneComet => match attack_type {
            Melee => [arcane_comet_min(&ctx), arcane_comet_max(&ctx)],
            Ranged => [arcane_comet_min(&ctx), arcane_comet_max(&ctx)],
        },

        RuneId::AttackSpeed => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::AxiomArcanist => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::BiscuitDelivery => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::BonePlating => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::CashBack => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Celerity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::CelestialBody => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::CheapShot => match attack_type {
            Melee => [cheap_shot_min(&ctx), zero(&ctx)],
            Ranged => [cheap_shot_min(&ctx), zero(&ctx)],
        },

        RuneId::Chrysalis => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Conditioning => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Conqueror => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::CosmicInsight => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::CoupDeGrace => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::CutDown => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::DarkHarvest => match attack_type {
            Melee => [dark_harvest_min(&ctx), zero(&ctx)],
            Ranged => [dark_harvest_min(&ctx), zero(&ctx)],
        },

        RuneId::DeathfireTouch => match attack_type {
            Melee => [deathfire_touch_min(&ctx), deathfire_touch_max(&ctx)],
            Ranged => [deathfire_touch_min(&ctx), deathfire_touch_max(&ctx)],
        },

        RuneId::DeepWard => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Demolish => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Electrocute => match attack_type {
            Melee => [electrocute_min(&ctx), zero(&ctx)],
            Ranged => [electrocute_min(&ctx), zero(&ctx)],
        },

        RuneId::EyeballCollection => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::FirstStrike => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::FleetFootwork => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::FontOfLife => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::FuturesMarket => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::GatheringStorm => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::GhostPoro => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::GlacialAugment => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::GraspOfTheUndying => match attack_type {
            Melee => [grasp_of_the_undying_melee_min(&ctx), zero(&ctx)],
            Ranged => [grasp_of_the_undying_melee_min(&ctx), zero(&ctx)],
        },

        RuneId::GrislyMementos => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Guardian => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::HailOfBlades => match attack_type {
            Melee => [hail_of_blades_min(&ctx), zero(&ctx)],
            Ranged => [hail_of_blades_min(&ctx), zero(&ctx)],
        },

        RuneId::Health => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::HealthScaling => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::HextechFlashtraption => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::IngeniousHunter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::IronSkin => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::JackOfAllTrades => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Kleptomancy => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::LastStand => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::LegendAlacrity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::LegendBloodline => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::LegendHaste => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::LegendTenacity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::LethalTempo => match attack_type {
            Melee => [lethal_tempo_melee_min(&ctx), zero(&ctx)],
            Ranged => [lethal_tempo_melee_min(&ctx), zero(&ctx)],
        },

        RuneId::MagicalFootwear => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::ManaflowBand => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::MinionDematerializer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::MirrorShell => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::NimbusCloak => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::NullifyingOrb => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Overgrowth => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Overheal => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::PerfectTiming => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::PhaseRush => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Predator => match attack_type {
            Melee => [predator_min(&ctx), zero(&ctx)],
            Ranged => [predator_min(&ctx), zero(&ctx)],
        },

        RuneId::PresenceOfMind => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::PressTheAttack => match attack_type {
            Melee => [press_the_attack_min(&ctx), zero(&ctx)],
            Ranged => [press_the_attack_min(&ctx), zero(&ctx)],
        },

        RuneId::PrototypeOmnistone => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::RavenousHunter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::RelentlessHunter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Revitalize => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Scorch => match attack_type {
            Melee => [scorch_min(&ctx), zero(&ctx)],
            Ranged => [scorch_min(&ctx), zero(&ctx)],
        },

        RuneId::SecondWind => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::ShieldBash => match attack_type {
            Melee => [shield_bash_min(&ctx), zero(&ctx)],
            Ranged => [shield_bash_min(&ctx), zero(&ctx)],
        },

        RuneId::SixthSense => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::StormraidersSurge => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::SuddenImpact => match attack_type {
            Melee => [sudden_impact_min(&ctx), zero(&ctx)],
            Ranged => [sudden_impact_min(&ctx), zero(&ctx)],
        },

        RuneId::SummonAery => match attack_type {
            Melee => [summon_aery_min(&ctx), zero(&ctx)],
            Ranged => [summon_aery_min(&ctx), zero(&ctx)],
        },

        RuneId::TasteOfBlood => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::TheUltimateHat => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::TimeWarpTonic => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Transcendence => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::TreasureHunter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::TripleTonic => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Triumph => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::UltimateHunter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Unflinching => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::UnsealedSpellbook => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::Waterwalking => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        RuneId::ZombieWard => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },
    }
}
pub static RUNE_CACHE: [&Rune; RuneId::VARIANTS] = [
    &ABSOLUTE_FOCUS,
    &ABSORB_LIFE,
    &ADAPTIVE_FORCE,
    &AFTERSHOCK,
    &APPROACH_VELOCITY,
    &ARCANE_COMET,
    &ATTACK_SPEED,
    &AXIOM_ARCANIST,
    &BISCUIT_DELIVERY,
    &BONE_PLATING,
    &CASH_BACK,
    &CELERITY,
    &CELESTIAL_BODY,
    &CHEAP_SHOT,
    &CHRYSALIS,
    &CONDITIONING,
    &CONQUEROR,
    &COSMIC_INSIGHT,
    &COUP_DE_GRACE,
    &CUT_DOWN,
    &DARK_HARVEST,
    &DEATHFIRE_TOUCH,
    &DEEP_WARD,
    &DEMOLISH,
    &ELECTROCUTE,
    &EYEBALL_COLLECTION,
    &FIRST_STRIKE,
    &FLEET_FOOTWORK,
    &FONT_OF_LIFE,
    &FUTURES_MARKET,
    &GATHERING_STORM,
    &GHOST_PORO,
    &GLACIAL_AUGMENT,
    &GRASP_OF_THE_UNDYING,
    &GRISLY_MEMENTOS,
    &GUARDIAN,
    &HAIL_OF_BLADES,
    &HEALTH,
    &HEALTH_SCALING,
    &HEXTECH_FLASHTRAPTION,
    &INGENIOUS_HUNTER,
    &IRON_SKIN,
    &JACK_OF_ALL_TRADES,
    &KLEPTOMANCY,
    &LAST_STAND,
    &LEGEND_ALACRITY,
    &LEGEND_BLOODLINE,
    &LEGEND_HASTE,
    &LEGEND_TENACITY,
    &LETHAL_TEMPO,
    &MAGICAL_FOOTWEAR,
    &MANAFLOW_BAND,
    &MINION_DEMATERIALIZER,
    &MIRROR_SHELL,
    &NIMBUS_CLOAK,
    &NULLIFYING_ORB,
    &OVERGROWTH,
    &OVERHEAL,
    &PERFECT_TIMING,
    &PHASE_RUSH,
    &PREDATOR,
    &PRESENCE_OF_MIND,
    &PRESS_THE_ATTACK,
    &PROTOTYPE_OMNISTONE,
    &RAVENOUS_HUNTER,
    &RELENTLESS_HUNTER,
    &REVITALIZE,
    &SCORCH,
    &SECOND_WIND,
    &SHIELD_BASH,
    &SIXTH_SENSE,
    &STORMRAIDERS_SURGE,
    &SUDDEN_IMPACT,
    &SUMMON_AERY,
    &TASTE_OF_BLOOD,
    &THE_ULTIMATE_HAT,
    &TIME_WARP_TONIC,
    &TRANSCENDENCE,
    &TREASURE_HUNTER,
    &TRIPLE_TONIC,
    &TRIUMPH,
    &ULTIMATE_HUNTER,
    &UNFLINCHING,
    &UNSEALED_SPELLBOOK,
    &WATERWALKING,
    &ZOMBIE_WARD,
];
