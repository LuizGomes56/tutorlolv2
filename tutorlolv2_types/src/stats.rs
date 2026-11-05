//! Champion stats, attack and adaptive types.
use super::*;

/// A generic metadata holder that determines what buffs one item give
/// to the current player when bought. For example `StatName::AbilityPower(80)`
/// means that when bought, the player gets extra 80 ability power. This struct
/// weighs 4 bytes and the maximum stat buff for one item is [`u16::MAX`]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
    EnumCount,
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
    strum::FromRepr,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub enum StatName {
    AbilityHaste,
    AbilityPower,
    AdaptiveForce,
    Armor,
    ArmorPenetration,
    AttackDamage,
    AttackSpeed,
    BaseHealthRegen,
    BaseManaRegen,
    CritChance,
    CritDamage,
    GoldPer10Seconds,
    HealAndShieldPower,
    Health,
    Lethality,
    LifeSteal,
    MagicPenetration,
    MagicPenetrationPercent,
    MagicResist,
    Mana,
    MoveSpeed,
    MoveSpeedPercent,
    Omnivamp,
    Tenacity,
}

impl FromStr for StatName {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ah" => Ok(Self::AbilityHaste),
            "hp" => Ok(Self::Health),
            "mr" => Ok(Self::MagicResist),
            "ap" => Ok(Self::AbilityPower),
            "mana" => Ok(Self::Mana),
            "ms" => Ok(Self::MoveSpeedPercent),
            "hsp" => Ok(Self::HealAndShieldPower),
            "mp5" => Ok(Self::BaseManaRegen),
            "armor" => Ok(Self::Armor),
            "msflat" => Ok(Self::MoveSpeed),
            "crit" => Ok(Self::CritChance),
            "ad" => Ok(Self::AttackDamage),
            "armpen" => Ok(Self::ArmorPenetration),
            "lethality" => Ok(Self::Lethality),
            "as" => Ok(Self::AttackSpeed),
            "lifesteal" => Ok(Self::LifeSteal),
            "mpen" => Ok(Self::MagicPenetrationPercent),
            "mpenflat" => Ok(Self::MagicPenetration),
            "gp10" => Ok(Self::GoldPer10Seconds),
            "hp5" => Ok(Self::BaseHealthRegen),
            "tenacity" => Ok(Self::Tenacity),
            "spec" => Ok(Self::AdaptiveForce),
            "omnivamp" => Ok(Self::Omnivamp),
            "hp5flat" => Ok(Self::BaseHealthRegen),
            "critdamage" => Ok(Self::CritDamage),
            _ => Err("Unknown stat in StatName::from_str"),
        }
    }
}

/// A champion can have either melee or ranged damage. Ranged champions
/// often have some damage penalty for items and runes, which are considered
/// by branching over this enum
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
    Encode,
    Decode,
    Serialize,
    Deserialize,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::FromRepr,
    strum::EnumString,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
#[strum(parse_err_ty = ParseError, parse_err_fn = AttackType::parse_error)]
pub enum AttackType {
    #[default]
    #[strum(to_string = "Melee", serialize = "MELEE")]
    Melee,
    #[strum(to_string = "Ranged", serialize = "RANGED")]
    Ranged,
}

impl AttackType {
    const fn parse_error(_: &str) -> &'static str {
        "No matches when calling AttackType::from_str"
    }
}

/// Enum that holds the current adaptive type of some champion, which
/// can be either physical or magic. It is mainly used to determine if runes
/// should deal physical or magic damage, or to convert `Adaptive Force`
/// stats to either `Attack Damage` or `Ability Power`. Check the enum [`StatName`]
/// for more details about all the possibilities
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
    Encode,
    Decode,
    Serialize,
    Deserialize,
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::EnumString,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
#[strum(parse_err_ty = ParseError, parse_err_fn = AdaptiveType::parse_error)]
pub enum AdaptiveType {
    #[default]
    #[strum(to_string = "Physical", serialize = "PHYSICAL_DAMAGE")]
    Physical,
    #[strum(to_string = "Magic", serialize = "MAGIC_DAMAGE")]
    Magic,
}

impl AdaptiveType {
    const fn parse_error(_: &str) -> &'static str {
        "No matches when calling AdaptiveType::from_str"
    }

    pub const fn try_infer(bonus_attack_damage: f32, ability_power: f32) -> Option<Self> {
        let lhs = 0.35 * bonus_attack_damage;
        let rhs = 0.2 * ability_power;

        if lhs == rhs {
            None
        } else if lhs > rhs {
            Some(Self::Physical)
        } else {
            Some(Self::Magic)
        }
    }
}

impl TryFrom<(f32, f32)> for AdaptiveType {
    type Error = ();

    fn try_from(value: (f32, f32)) -> Result<Self, Self::Error> {
        let (bonus_attack_damage, ability_power) = value;
        Self::try_infer(bonus_attack_damage, ability_power).ok_or(())
    }
}
