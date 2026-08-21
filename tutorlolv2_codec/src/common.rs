use std::fmt;

pub const MAGIC: [u8; 4] = *b"FBC1";
pub const VERSION: u8 = 2;
pub(crate) const HEADER_LEN: usize = 24;

// Number opcodes.
pub(crate) const OP_NUM_U8: u8 = 0x00;
pub(crate) const OP_NUM_I8: u8 = 0x01;
pub(crate) const OP_NUM_CONST: u8 = 0x02;
pub(crate) const OP_NUM_F32: u8 = 0x03;

// Leaf/structural opcodes.
pub(crate) const OP_CTX: u8 = 0x10;
pub(crate) const OP_REF_LOCAL: u8 = 0x11;
pub(crate) const OP_MATCH_INLINE: u8 = 0x12;
pub(crate) const OP_GROUP: u8 = 0x13;
pub(crate) const OP_NEG: u8 = 0x14;
pub(crate) const OP_MATCH_BLOCK: u8 = 0x15;

// 0x20..=0x2b are binary operators.
// low 2 bits: operator  (0 +, 1 -, 2 *, 3 /)
// bits 2..3: break kind (0 inline, 1 before op, 2 after op)
pub(crate) const OP_BIN_BASE: u8 = 0x20;
pub(crate) const OP_BIN_MAX: u8 = 0x2b;

#[derive(Debug, Clone)]
pub enum Error {
    Parse {
        line: u32,
        column: u32,
        message: String,
    },
    UnknownCtxVar(String),
    UnknownFunction(String),
    OwnerOutOfRange {
        kind: EntityKind,
        owner: u16,
        count: u16,
    },
    DuplicateOwner {
        kind: EntityKind,
        owner: u16,
    },
    DuplicateLocal(u8),
    ChampionLocalsMustBeDense,
    InvalidSlotCombination(&'static str),
    TooManyMatches,
    TooManyMatchArms,
    TooManyFormulas,
    TooManyOwners,
    InvalidIndent {
        line: u32,
        column: u32,
    },
    Corrupt(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse {
                line,
                column,
                message,
            } => write!(f, "parse error at {line}:{column}: {message}"),
            Self::UnknownCtxVar(v) => write!(f, "unknown ctx variable: {v}"),
            Self::UnknownFunction(v) => write!(f, "unknown local function reference: {v}"),
            Self::OwnerOutOfRange { kind, owner, count } => {
                write!(
                    f,
                    "{kind:?} owner {owner} is outside table of {count} owners"
                )
            }
            Self::DuplicateOwner { kind, owner } => {
                write!(f, "{kind:?} owner {owner} was inserted twice")
            }
            Self::DuplicateLocal(local) => write!(f, "duplicate local formula index {local}"),
            Self::ChampionLocalsMustBeDense => {
                write!(
                    f,
                    "champion local formula indices must be 0..len with no gaps"
                )
            }
            Self::InvalidSlotCombination(msg) => write!(f, "invalid damage slots: {msg}"),
            Self::TooManyMatches => write!(f, "more than 256 unique matches"),
            Self::TooManyMatchArms => write!(f, "a match has more than 255 arms"),
            Self::TooManyFormulas => write!(f, "more than 65535 formulas"),
            Self::TooManyOwners => write!(f, "an owner table exceeds u16::MAX"),
            Self::InvalidIndent { line, column } => write!(
                f,
                "line-break indentation at {line}:{column} is not an even number of spaces or exceeds 510 spaces"
            ),
            Self::Corrupt(msg) => write!(f, "corrupt formula database: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::FromRepr)]
#[repr(u8)]
pub enum EntityKind {
    Champion,
    Item,
    Rune,
}

/// Logical formula slot used by both items and runes.
/// Physical storage is sparse; this is never a physical formula index.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageSlot {
    MeleeMin = 0,
    MeleeMax = 1,
    RangedMin = 2,
    RangedMax = 3,
}

impl DamageSlot {
    pub const ALL: [Self; 4] = [
        Self::MeleeMin,
        Self::MeleeMax,
        Self::RangedMin,
        Self::RangedMax,
    ];

    #[inline]
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }

    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::MeleeMin),
            1 => Some(Self::MeleeMax),
            2 => Some(Self::RangedMin),
            3 => Some(Self::RangedMax),
            _ => None,
        }
    }

    pub(crate) fn validate_mask(mask: u8) -> Result<(), Error> {
        let melee_min = mask & Self::MeleeMin.bit() != 0;
        let melee_max = mask & Self::MeleeMax.bit() != 0;
        let ranged_min = mask & Self::RangedMin.bit() != 0;
        let ranged_max = mask & Self::RangedMax.bit() != 0;

        if melee_max && !melee_min {
            return Err(Error::InvalidSlotCombination(
                "MeleeMax cannot exist without MeleeMin",
            ));
        }
        if ranged_max && !ranged_min {
            return Err(Error::InvalidSlotCombination(
                "RangedMax cannot exist without RangedMin",
            ));
        }
        if (melee_min || melee_max) && !ranged_min {
            return Err(Error::InvalidSlotCombination(
                "melee damage requires ranged damage for context",
            ));
        }
        Ok(())
    }
}
