/// Entity category used by catalog IDs and packed database lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::FromRepr)]
#[repr(u8)]
pub enum EntityKind {
    Champion = 0,
    Item = 1,
    Rune = 2,
}

/// Catalog category and its generated module/type naming conventions.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Ord,
    PartialOrd,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::AsRefStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Tag {
    Champions,
    Items,
    Runes,
}

impl Tag {
    pub const fn entity_kind(self) -> EntityKind {
        match self {
            Self::Champions => EntityKind::Champion,
            Self::Items => EntityKind::Item,
            Self::Runes => EntityKind::Rune,
        }
    }

    pub const fn repr(&self) -> &'static str {
        match self {
            Self::Items => "u16",
            Self::Champions | Self::Runes => "u8",
        }
    }

    pub const fn enum_name(&self) -> &'static str {
        match self {
            Self::Items => "ItemId",
            Self::Champions => "ChampionId",
            Self::Runes => "RuneId",
        }
    }
}
