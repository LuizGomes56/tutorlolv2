//! Damage types, attributes and indexes.
use super::*;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    strum::EnumCount,
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
    strum::FromRepr,
)]
#[repr(u8)]
pub enum DamageIndex {
    #[default]
    Min,
    Max,
}

/// Defines what is the damage type of some entity.
/// - [`DamageType::Physical`] and [`DamageType::Magic`] Represents any damage related
///
/// to how much (armor or magic resistence) the enemy player has, and is affected by the
/// percent and flat values or (armor or magic) penetration of the current player
/// - [`DamageType::Mixed`] Damages of this type are treated as a special case of [`DamageType::True`],
///
/// where the closure has to multiply on its own the `physical_mod`
///
/// and `magic_mod` modifiers of the `math::DamageModifiers` struct. It is
/// usually used when a single ability or item deals both physical and magic damage in the same hit.
///
/// - [`DamageType::True`] Damages of this type are not affected by armor or magic resistence,
///
/// their values are in general irreducible.
///
/// - [`DamageType::Adaptive`] Damages of this type will vary into the [`DamageType::Physical`]
///
/// or [`DamageType::Magic`] depending on how much bonus armor or ability power the current player has.
/// - [`DamageType::Unknown`] is the default value when no damage type is set
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumIter,
    strum::VariantNames,
    strum::IntoStaticStr,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = types))]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptive,
    #[default]
    Unspecified,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value: &str = self.into();
        write!(f, "{value}")
    }
}

/// When the damage of an item, rune or ability happens, relative to basic attacks.
/// See [`Attrs`]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumIter,
    strum::VariantNames,
    strum::FromRepr,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u8)]
pub enum Trigger {
    /// Not tied to a basic attack: abilities, actives, procs of their own
    #[default]
    Direct,
    /// Every basic attack, as Nashor's Tooth or Blade of the Ruined King
    EveryHit,
    /// The next basic attack after casting an ability, as Sheen
    Spellblade,
    /// A basic attack once enough energized stacks are built, as Statikk Shiv
    Energized,
    /// Every few basic attacks, as Kraken Slayer's third attack
    NthHit,
    /// A basic attack once a charge or condition is met, as Heartsteel
    Charged,
}

/// How an ability relates to basic attacks. See [`Attrs`]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumIter,
    strum::VariantNames,
    strum::FromRepr,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u8)]
pub enum AttackRole {
    /// Neither an attack nor applying on-hit effects
    #[default]
    Standalone,
    /// Applies on-hit effects without being an attack, as Ezreal's Q
    AppliesOnhit,
    /// Counts as a basic attack, as Zeri's Q
    IsAttack,
}

/// Attributes of an ability, item or rune, packed in one byte so that
/// [`crate::TypeMetadata`] fits in 4 bytes:
///
/// - bits 0 to 2: the [`Trigger`]
/// - bit 3: whether it deals area damage
/// - bits 4 and 5: the [`AttackRole`]
/// - bits 6 and 7: unused
///
/// What the old `Onhit`, `OnhitMin` and `OnhitMax` meant is derived from them, see
/// [`Attrs::onhit_min`] and [`Attrs::onhit_max`]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(from = "AttrsFields", into = "AttrsFields")]
#[repr(transparent)]
pub struct Attrs(u8);

/// The fields of [`Attrs`], as serde writes them
#[derive(Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
struct AttrsFields {
    #[serde(default)]
    trigger: Trigger,
    #[serde(default)]
    area: bool,
    #[serde(default)]
    role: AttackRole,
}

impl From<AttrsFields> for Attrs {
    fn from(
        AttrsFields {
            trigger,
            area,
            role,
        }: AttrsFields,
    ) -> Self {
        Self::new(trigger, area, role)
    }
}

impl From<Attrs> for AttrsFields {
    fn from(attrs: Attrs) -> Self {
        Self {
            trigger: attrs.trigger(),
            area: attrs.is_area(),
            role: attrs.role(),
        }
    }
}

impl Attrs {
    /// No attribute: a direct, single target damage
    pub const NONE: Self = Self(0);

    const TRIGGER_MASK: u8 = 0b0000_0111;
    const AREA_BIT: u8 = 0b0000_1000;
    const ROLE_SHIFT: u8 = 4;
    const ROLE_MASK: u8 = 0b0011_0000;

    pub const fn new(trigger: Trigger, area: bool, role: AttackRole) -> Self {
        Self(
            trigger as u8
                | if area { Self::AREA_BIT } else { 0 }
                | (role as u8) << Self::ROLE_SHIFT,
        )
    }

    /// The packed byte, as [`Self::from_bits`] reads it
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Reads a byte written by [`Self::bits`]. Unknown triggers and roles read
    /// as their defaults
    pub const fn from_bits(bits: u8) -> Self {
        let trigger = match Trigger::from_repr(bits & Self::TRIGGER_MASK) {
            Some(trigger) => trigger,
            None => Trigger::Direct,
        };

        let role = match AttackRole::from_repr((bits & Self::ROLE_MASK) >> Self::ROLE_SHIFT) {
            Some(role) => role,
            None => AttackRole::Standalone,
        };

        Self::new(trigger, bits & Self::AREA_BIT != 0, role)
    }

    pub const fn trigger(self) -> Trigger {
        match Trigger::from_repr(self.0 & Self::TRIGGER_MASK) {
            Some(trigger) => trigger,
            None => Trigger::Direct,
        }
    }

    pub const fn is_area(self) -> bool {
        self.0 & Self::AREA_BIT != 0
    }

    pub const fn role(self) -> AttackRole {
        match AttackRole::from_repr((self.0 & Self::ROLE_MASK) >> Self::ROLE_SHIFT) {
            Some(role) => role,
            None => AttackRole::Standalone,
        }
    }

    pub const fn with_trigger(self, trigger: Trigger) -> Self {
        Self::new(trigger, self.is_area(), self.role())
    }

    pub const fn with_area(self, area: bool) -> Self {
        Self::new(self.trigger(), area, self.role())
    }

    pub const fn with_role(self, role: AttackRole) -> Self {
        Self::new(self.trigger(), self.is_area(), role)
    }

    /// Counts in the minimum on-hit damage: it applies on every basic attack
    pub const fn onhit_min(self) -> bool {
        matches!(self.trigger(), Trigger::EveryHit)
    }

    /// Counts in the maximum on-hit damage: it applies on some basic attack
    pub const fn onhit_max(self) -> bool {
        !matches!(self.trigger(), Trigger::Direct)
    }

    /// A combo that lists it without a basic attack next to it implies one
    pub const fn implies_attack(self) -> bool {
        !matches!(self.trigger(), Trigger::Direct)
    }

    /// Guinsoo's Rageblade's Seething Strike applies it twice on every third
    /// attack. To the best of current knowledge only every-hit effects are, so a
    /// change found in game is a change of this `match`
    pub const fn doubled_by_seething_strike(self) -> bool {
        matches!(self.trigger(), Trigger::EveryHit)
    }
}

impl From<Trigger> for Attrs {
    fn from(trigger: Trigger) -> Self {
        Self::new(trigger, false, AttackRole::Standalone)
    }
}

#[cfg(feature = "bake")]
impl databake::Bake for Attrs {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("types");
        let bits = self.0;
        databake::quote!(types::Attrs::from_bits(#bits))
    }
}

#[cfg(feature = "bake")]
impl databake::BakeSize for Attrs {
    fn borrows_size(&self) -> usize {
        0
    }
}

impl FromStr for DamageType {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PHYSICAL_DAMAGE" | "Physical" => Ok(DamageType::Physical),
            "MAGIC_DAMAGE" | "Magic" | "magic" => Ok(DamageType::Magic),
            "MIXED_DAMAGE" | "Mixed" | "Magic True" => Ok(DamageType::Mixed),
            "TRUE_DAMAGE" | "True" | "true" => Ok(DamageType::True),
            "ADAPTIVE_DAMAGE" | "adaptive" => Ok(DamageType::Adaptive),
            _ => Ok(DamageType::Unspecified),
        }
    }
}
