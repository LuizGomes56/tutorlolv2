//! Evaluation context of the generated closures, see [`Ctx`] and [`CtxVar`].
use super::*;

/// Creates the `CtxVar` and `Ctx` structs, associating
/// the appropriate names and numeric types that it will hold. This struct
/// is essential to the application since it is used to evaluate all the
/// generated closures contained in cache static variables.
///
/// `attacker` and `target` become fields of [`Ctx`], and also of [`CtxAttacker`]
/// and [`CtxTarget`], the parts that output tables store once per attacker state
/// and once per target. `methods` are derived from those fields,
/// so they have a [`CtxVar`] variant but no field: each one needs a `const fn`
/// of the same name in `impl Ctx`. Formulas still spell them as `ctx.name`, so
/// the wiki and the simplifier treat them as fields, and the build script turns
/// them into `ctx.name()` calls when it generates the closures
macro_rules! create_eval_struct {
    (
        attacker { $($attacker:ident),*$(,)? }
        target { $($target:ident),*$(,)? }
        methods { $($method:ident),*$(,)? }
    ) => {
        pastey::paste! {
            /// Defines a standard type that implements trait [`core::fmt::Display`]
            /// and is used to create constant closures in the static variables of
            /// this module. For example:
            /// [`CtxVar::QLevel`] is converted to: `ctx.q_level`
            #[derive(
                Copy,
                Clone,
                Debug,
                Eq,
                Hash,
                PartialEq,
                Ord,
                PartialOrd,
                Encode,
                Decode,
                Serialize,
                Deserialize,
                strum::FromRepr,
                strum::EnumIter,
                strum::VariantArray,
                strum::EnumCount,
                strum::EnumString,
            )]
            #[repr(u8)]
            #[cfg_attr(feature = "bake", derive(databake::Bake))]
            #[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
            #[strum(parse_err_ty = ParseError, parse_err_fn = CtxVar::parse_error)]
            pub enum CtxVar {
                $(
                    #[strum(serialize = "" $attacker, serialize = "" [<$attacker:camel>])]
                    [<$attacker:camel>],
                )*
                $(
                    #[strum(serialize = "" $target, serialize = "" [<$target:camel>])]
                    [<$target:camel>],
                )*
                $(
                    #[strum(serialize = "" $method, serialize = "" [<$method:camel>])]
                    [<$method:camel>],
                )*
            }

            impl CtxVar {
                const fn parse_error(_: &str) -> &'static str {
                    "CtxVar::from_str: Invalid variable provided"
                }

                pub const fn to_snake_case(&self) -> &'static str {
                    match self {
                        $(Self::[<$attacker:camel>] => stringify!($attacker),)*
                        $(Self::[<$target:camel>] => stringify!($target),)*
                        $(Self::[<$method:camel>] => stringify!($method),)*
                    }
                }

                /// Variable as written in formulas, such as `ctx.q_level`. Methods
                /// are written the same way, see [`Self::is_method`]
                pub const fn as_var(&self) -> &'static str {
                    match self {
                        $(
                            Self::[<$attacker:camel>] => concat!("ctx.", stringify!($attacker)),
                        )*
                        $(
                            Self::[<$target:camel>] => concat!("ctx.", stringify!($target)),
                        )*
                        $(
                            Self::[<$method:camel>] => concat!("ctx.", stringify!($method)),
                        )*
                    }
                }

                /// Whether this variable is derived by a method of [`Ctx`]
                /// rather than stored in a field
                pub const fn is_method(&self) -> bool {
                    match self {
                        $(Self::[<$method:camel>] => true,)*
                        _ => false,
                    }
                }
            }

            impl AsRef<str> for CtxVar {
                fn as_ref(&self) -> &str {
                    self.as_var()
                }
            }

            impl Ctx {
                /// Reads a field, or evaluates a method, of this context
                pub const fn get(&self, var: CtxVar) -> f32 {
                    match var {
                        $(CtxVar::[<$attacker:camel>] => self.$attacker,)*
                        $(CtxVar::[<$target:camel>] => self.$target,)*
                        $(CtxVar::[<$method:camel>] => self.$method(),)*
                    }
                }
            }

            #[derive(
                Clone,
                Copy,
                Debug,
                Decode,
                Default,
                Deserialize,
                Encode,
                PartialEq,
                PartialOrd,
                Serialize
            )]
            pub struct Ctx {
                $(pub $attacker: f32,)*
                $(pub $target: f32,)*
                /// Stored as integers, read as `f32` through methods such as
                /// [`Ctx::q_level`]. Last, so that it adds no padding
                pub ability_levels: AbilityLevels,
                /// Branches of the derived multipliers, see [`CtxFlags`]
                pub flags: CtxFlags,
                /// Changes of the game mode, read by the closures it changes. It
                /// fits in the padding after `flags`, so the size is unchanged
                pub map: MapMode,
            }

            /// Part of [`Ctx`] that comes from the attacker's state. A purchase
            /// only changes this part
            #[derive(
                Clone,
                Copy,
                Debug,
                Decode,
                Default,
                Deserialize,
                Encode,
                PartialEq,
                PartialOrd,
                Serialize
            )]
            pub struct CtxAttacker {
                $(pub $attacker: f32,)*
                pub ability_levels: AbilityLevels,
                /// Only the bits of [`CtxFlags::ATTACKER`]
                pub flags: CtxFlags,
                /// The map is the same for the whole game, so it comes with the attacker
                pub map: MapMode,
            }

            /// Part of [`Ctx`] that comes from the target
            #[derive(
                Clone,
                Copy,
                Debug,
                Decode,
                Default,
                Deserialize,
                Encode,
                PartialEq,
                PartialOrd,
                Serialize
            )]
            pub struct CtxTarget {
                $(pub $target: f32,)*
                /// Only the bits of [`CtxFlags::TARGET`]
                pub flags: CtxFlags,
            }

            impl Ctx {
                /// Joins the parts of an attacker and a target
                pub const fn new(attacker: &CtxAttacker, target: &CtxTarget) -> Self {
                    Self {
                        $($attacker: attacker.$attacker,)*
                        $($target: target.$target,)*
                        ability_levels: attacker.ability_levels,
                        flags: attacker.flags.union(target.flags),
                        map: attacker.map,
                    }
                }

                pub const fn attacker(&self) -> CtxAttacker {
                    CtxAttacker {
                        $($attacker: self.$attacker,)*
                        ability_levels: self.ability_levels,
                        flags: self.flags.intersection(CtxFlags::ATTACKER),
                        map: self.map,
                    }
                }

                pub const fn target(&self) -> CtxTarget {
                    CtxTarget {
                        $($target: self.$target,)*
                        flags: self.flags.intersection(CtxFlags::TARGET),
                    }
                }
            }

            impl ::core::fmt::Display for CtxVar {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{}", self.as_var())
                }
            }
        }
    };
}

create_eval_struct!(
    attacker {
        ability_power,
        armor,
        armor_penetration_flat,
        armor_penetration_percent,
        attack_damage,
        attack_speed,
        base_ad,
        base_armor,
        base_attack_speed,
        base_health,
        base_magic_resist,
        base_mana,
        crit_chance,
        crit_damage,
        current_health,
        current_mana,
        level,
        magic_penetration_flat,
        magic_penetration_percent,
        magic_resist,
        max_health,
        max_mana,
        stacks,
    }
    target {
        enemy_armor,
        enemy_bonus_armor,
        enemy_bonus_health,
        enemy_bonus_magic_resist,
        enemy_current_health,
        enemy_magic_resist,
        enemy_max_health,
    }
    methods {
        adaptive_damage,
        bonus_ad,
        bonus_armor,
        bonus_attack_speed,
        bonus_health,
        bonus_magic_resist,
        bonus_mana,
        bonus_move_speed,
        e_level,
        enemy_missing_health_flat,
        life_steal,
        magic_multiplier,
        missing_health_ratio,
        physical_multiplier,
        q_level,
        r_level,
        randuin_effect,
        rocksolid_effect,
        steelcaps_effect,
        unknown,
        w_level,
    }
);

/// Booleans and the adaptive type behind the multipliers of [`Ctx`], packed
/// so that the context fits in two cache lines
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Decode,
    Deserialize,
    Encode,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct CtxFlags(u8);

impl CtxFlags {
    /// The enemy owns Plated Steelcaps or Armored Advance
    pub const STEELCAPS: u8 = 1 << 0;
    /// The enemy owns Randuin's Omen
    pub const RANDUIN: u8 = 1 << 1;
    /// The enemy owns an item with the Rocksolid passive
    pub const ROCKSOLID: u8 = 1 << 2;
    /// Resists may go below zero after penetration, as for monsters
    pub const NEGATIVE_RESISTS: u8 = 1 << 3;
    /// Adaptive damage is magic rather than physical
    pub const MAGIC_ADAPTIVE: u8 = 1 << 4;

    const fn bit(set: bool, flag: u8) -> u8 {
        if set { flag } else { 0 }
    }

    /// Bits of the attacker's state
    pub const fn attacker(adaptive_type: AdaptiveType) -> Self {
        Self(Self::bit(
            matches!(adaptive_type, AdaptiveType::Magic),
            Self::MAGIC_ADAPTIVE,
        ))
    }

    /// Bits of the target
    pub const fn target(
        steelcaps: bool,
        randuin: bool,
        rocksolid: bool,
        accept_negatives: bool,
    ) -> Self {
        Self(
            Self::bit(steelcaps, Self::STEELCAPS)
                | Self::bit(randuin, Self::RANDUIN)
                | Self::bit(rocksolid, Self::ROCKSOLID)
                | Self::bit(accept_negatives, Self::NEGATIVE_RESISTS),
        )
    }

    /// Bits that come from the attacker's state
    pub const ATTACKER: Self = Self(Self::MAGIC_ADAPTIVE);
    /// Bits that come from the target
    pub const TARGET: Self =
        Self(Self::STEELCAPS | Self::RANDUIN | Self::ROCKSOLID | Self::NEGATIVE_RESISTS);

    pub const fn contains(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

/// Resist left after penetration and the damage multiplier it grants
#[derive(
    Copy, Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize,
)]
pub struct ResistValue {
    pub real: f32,
    pub modifier: f32,
}

impl ResistValue {
    /// `percent_pen` is in the range `[0.0, 100.0]`. Without
    /// `accept_negatives`, the resist left is at least zero
    pub const fn new(percent_pen: f32, flat_pen: f32, resist: f32, accept_negatives: bool) -> Self {
        let real = ((1.0 - percent_pen / 100.0) * resist - flat_pen).max(if accept_negatives {
            f32::NEG_INFINITY
        } else {
            0.0
        });
        let modifier = 100.0 / (100.0 + real);
        Self { real, modifier }
    }
}

impl Ctx {
    // These are checked against the values extracted from the wiki in
    // `tutorlolv2::libgen::check`, so the build fails if the wiki changes them

    /// Item Plated Steelcaps gives 10% damage reduction for basic attacks
    pub const STEEL_CAPS_PROTECTION: f32 = 0.9;

    /// Item Randuin's Omen gives 30% damage reduction against critical hits
    pub const RANDUIN_CRIT_PROTECTION: f32 = 0.7;

    /// Items with Rock Solid passive give up to 20% damage reduction
    pub const ROCKSOLID_PROTECTION: f32 = 0.8;

    const fn select(&self, flag: u8, value: f32) -> f32 {
        if self.flags.contains(flag) {
            value
        } else {
            1.0
        }
    }

    pub const fn steelcaps_effect(&self) -> f32 {
        self.select(CtxFlags::STEELCAPS, Self::STEEL_CAPS_PROTECTION)
    }

    pub const fn randuin_effect(&self) -> f32 {
        self.select(CtxFlags::RANDUIN, Self::RANDUIN_CRIT_PROTECTION)
    }

    pub const fn rocksolid_effect(&self) -> f32 {
        self.select(CtxFlags::ROCKSOLID, Self::ROCKSOLID_PROTECTION)
    }

    /// Multiplier of physical damage after the enemy's armor. Penetration is
    /// clamped as in the shred the enemy's resists were computed with
    pub const fn physical_multiplier(&self) -> f32 {
        ResistValue::new(
            self.armor_penetration_percent.clamp(0.0, 100.0),
            self.armor_penetration_flat,
            self.enemy_armor,
            self.flags.contains(CtxFlags::NEGATIVE_RESISTS),
        )
        .modifier
    }

    /// Multiplier of magic damage after the enemy's magic resist
    pub const fn magic_multiplier(&self) -> f32 {
        ResistValue::new(
            self.magic_penetration_percent.clamp(0.0, 100.0),
            self.magic_penetration_flat,
            self.enemy_magic_resist,
            self.flags.contains(CtxFlags::NEGATIVE_RESISTS),
        )
        .modifier
    }

    /// Multiplier of the damage type the adaptive damage resolves to
    pub const fn adaptive_damage(&self) -> f32 {
        match self.flags.contains(CtxFlags::MAGIC_ADAPTIVE) {
            true => self.magic_multiplier(),
            false => self.physical_multiplier(),
        }
    }

    /// Life steal is not modeled, so its scalings evaluate to zero
    pub const fn life_steal(&self) -> f32 {
        0.0
    }

    pub const fn bonus_ad(&self) -> f32 {
        self.attack_damage - self.base_ad
    }

    pub const fn bonus_armor(&self) -> f32 {
        self.armor - self.base_armor
    }

    pub const fn bonus_attack_speed(&self) -> f32 {
        self.attack_speed - self.base_attack_speed
    }

    pub const fn bonus_health(&self) -> f32 {
        self.max_health - self.base_health
    }

    pub const fn bonus_magic_resist(&self) -> f32 {
        self.magic_resist - self.base_magic_resist
    }

    pub const fn bonus_mana(&self) -> f32 {
        self.max_mana - self.base_mana
    }

    /// Movement speed is not modeled, so its scalings use a neutral value
    pub const fn bonus_move_speed(&self) -> f32 {
        1.0
    }

    pub const fn q_level(&self) -> f32 {
        self.ability_levels.q as f32
    }

    pub const fn w_level(&self) -> f32 {
        self.ability_levels.w as f32
    }

    pub const fn e_level(&self) -> f32 {
        self.ability_levels.e as f32
    }

    pub const fn r_level(&self) -> f32 {
        self.ability_levels.r as f32
    }

    /// Variables that the wiki parser could not identify, always zero
    pub const fn unknown(&self) -> f32 {
        0.0
    }

    /// Health the enemy is missing, as an amount rather than a ratio
    pub const fn enemy_missing_health_flat(&self) -> f32 {
        (self.enemy_max_health - self.enemy_current_health).max(0.0)
    }

    /// Ratio of the maximum health that is missing, from `0.0` at full health
    /// to `1.0` at zero health. The maximum health is at least `1.0` to avoid
    /// division by zero
    pub const fn missing_health_ratio(&self) -> f32 {
        (1.0 - self.current_health / self.max_health.max(1.0)).clamp(0.0, 1.0)
    }
}

impl CtxVar {
    pub const SKIP: usize = Self::SteelcapsEffect as usize + 1;

    pub const ENEMY_VARS: [Self; 7] = [
        CtxVar::EnemyArmor,
        CtxVar::EnemyBonusArmor,
        CtxVar::EnemyBonusHealth,
        CtxVar::EnemyBonusMagicResist,
        CtxVar::EnemyCurrentHealth,
        CtxVar::EnemyMagicResist,
        CtxVar::EnemyMaxHealth,
    ];
}
