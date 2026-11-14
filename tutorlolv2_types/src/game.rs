//! Positions and maps.
use super::*;

/// Represents each playable position or `lane` that a champion can
/// play in the standard gamemode `SummonersRift`, whose definition
/// is [`GameMap::SummonersRift`]. If we don't know a champion's position,
/// it is set to [`Position::Top`].
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
    strum::VariantArray,
    strum::EnumCount,
    strum::FromRepr,
    strum::EnumString,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
#[strum(parse_err_ty = ParseError, parse_err_fn = Position::parse_error)]
pub enum Position {
    #[default]
    #[strum(to_string = "Top", serialize = "TOP")]
    Top,
    #[strum(to_string = "Jungle", serialize = "JUNGLE")]
    Jungle,
    #[strum(to_string = "Middle", serialize = "MIDDLE")]
    Middle,
    #[strum(to_string = "Bottom", serialize = "BOTTOM")]
    Bottom,
    #[strum(to_string = "Support", serialize = "SUPPORT", serialize = "UTILITY")]
    Support,
}

impl Position {
    const fn parse_error(_: &str) -> &'static str {
        "No matches when calling Position::from_str"
    }

    pub const fn index(&self) -> usize {
        *self as _
    }

    pub const fn role(&self) -> &'static str {
        match self {
            Position::Top => "top",
            Position::Jungle => "jungle",
            Position::Middle => "mid",
            Position::Bottom => "adc",
            Position::Support => "support",
        }
    }
}

/// All possible maps and codes that can be played. Most of them are
/// event maps that may never return to the game, and don't have a
/// deterministic code. [`GameMap::SummonersRift`] is the default map.
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
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub enum GameMap {
    Aram,
    Arena,
    DarkStar,
    Dominion,
    Invasion,
    NexusBlitz,
    Odyssey,
    Project,
    StarGuardian,
    SummonersRift,
    Tft,
    Tutorial,
    TwistedTreeline,
    Urf,
    #[default]
    Unknown,
    UnknownMap33,
    UnknownMap35,
    OneForAll,
    UnsealedSpellbook,
    SwiftPlay,
    /// ARAM: Mayhem, played on the ARAM map with its own code. Appended so
    /// the encoding of every other variant is kept
    AramMayhem,
}

/// Set of game mode changes that closures apply, as stored in [`Ctx`]. Several
/// maps share one, as ARAM and ARAM: Mayhem. The first variant is zero, so a
/// zeroed context, as the rows of an output table before they are written,
/// behaves as Summoner's Rift
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
    strum::EnumString,
    strum::FromRepr,
    strum::IntoStaticStr,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub enum MapMode {
    /// No changes, as on Summoner's Rift
    #[default]
    Default,
    /// `Module:MapChanges/data/aram`
    Aram,
    /// `Module:MapChanges/data/ar`
    Arena,
}

impl GameMap {
    /// Changes that closures apply on this map
    pub const fn mode(self) -> MapMode {
        match self {
            Self::Aram | Self::AramMayhem => MapMode::Aram,
            Self::Arena => MapMode::Arena,
            _ => MapMode::Default,
        }
    }

    /// Value that grows linearly from `min` at level 1 to `max` at level 18
    const fn by_level(min: f32, max: f32, level: u8) -> f32 {
        let level = match level {
            0 => 1,
            19.. => 18,
            level => level,
        };

        min + (max - min) * (level - 1) as f32 / 17.0
    }

    /// Base health that this map grants to every champion
    pub const fn base_health(self, attack_type: AttackType, level: u8) -> f32 {
        match (self, attack_type) {
            (Self::AramMayhem, AttackType::Melee) => Self::by_level(20.0, 600.0, level),
            (Self::AramMayhem, AttackType::Ranged) => Self::by_level(20.0, 400.0, level),
            _ => 0.0,
        }
    }

    /// Bonus armor and magic resistance that this map grants to every champion
    pub const fn bonus_resists(self, attack_type: AttackType, level: u8) -> (f32, f32) {
        match (self, attack_type) {
            (Self::Aram, AttackType::Melee) => (0.0, 15.0),
            (Self::AramMayhem, AttackType::Melee) => (15.0, 15.0),
            (Self::AramMayhem, AttackType::Ranged) => {
                let value = Self::by_level(2.0, 25.0, level);
                (value, value)
            }
            _ => (0.0, 0.0),
        }
    }

    /// Constant conversion of a [`u8`] into a [`GameMap`] enum,
    /// where the byte represents the code of the current map
    pub const fn from_code(value: u8) -> Self {
        match value {
            3 => GameMap::Tutorial,
            4 | 10 => GameMap::TwistedTreeline,
            8 => GameMap::Dominion,
            11 => GameMap::SummonersRift,
            12 => GameMap::AramMayhem,
            14 => GameMap::Aram,
            13 => GameMap::Invasion,
            16 => GameMap::DarkStar,
            18 => GameMap::StarGuardian,
            19 => GameMap::Project,
            20 => GameMap::Odyssey,
            21 => GameMap::NexusBlitz,
            22 => GameMap::Tft,
            30 => GameMap::Arena,
            // Unknown
            33 => GameMap::UnknownMap33,
            35 => GameMap::UnknownMap35,
            0xFC => GameMap::OneForAll,
            0xFD => GameMap::UnsealedSpellbook,
            0xFE => GameMap::SwiftPlay,
            0xFF => GameMap::Urf,
            _ => GameMap::Unknown,
        }
    }
}
