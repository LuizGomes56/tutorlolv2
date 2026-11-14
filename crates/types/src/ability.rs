//! Abilities: their keys, levels, identifiers, names and combos.
use super::*;

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Key {
    #[default]
    P,
    Q,
    W,
    E,
    R,
}

/// Holds the levels of the abilities of a champion
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

impl AbilityLevels {
    pub const ABILITIES: [Key; 4] = [Key::Q, Key::W, Key::E, Key::R];

    pub const fn default() -> Self {
        Self {
            q: 0,
            w: 0,
            e: 0,
            r: 0,
        }
    }
}

impl core::ops::Index<Key> for AbilityLevels {
    type Output = u8;

    fn index(&self, index: Key) -> &Self::Output {
        match index {
            Key::P => panic!("Can't use Key::P to index AbilityLevels"),
            Key::Q => &self.q,
            Key::W => &self.w,
            Key::E => &self.e,
            Key::R => &self.r,
        }
    }
}

impl core::ops::IndexMut<Key> for AbilityLevels {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        match index {
            Key::P => panic!("Can't use Key::P to index AbilityLevels"),
            Key::Q => &mut self.q,
            Key::W => &mut self.w,
            Key::E => &mut self.e,
            Key::R => &mut self.r,
        }
    }
}

impl Key {
    pub const fn as_char(&self) -> char {
        match self {
            Key::P => 'P',
            Key::Q => 'Q',
            Key::W => 'W',
            Key::E => 'E',
            Key::R => 'R',
        }
    }

    pub const fn as_ctx_var(&self) -> CtxVar {
        match self {
            Key::P => CtxVar::Level,
            Key::Q => CtxVar::QLevel,
            Key::W => CtxVar::WLevel,
            Key::E => CtxVar::ELevel,
            Key::R => CtxVar::RLevel,
        }
    }
}

impl TryFrom<char> for Key {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' | 'A' | 'P' => Ok(Key::P),
            'Q' => Ok(Key::Q),
            'W' => Ok(Key::W),
            'E' => Ok(Key::E),
            'R' => Ok(Key::R),
            _ => Err("Invalid char when calling Key::try_from"),
        }
    }
}

/// Enum that represents one ability of a champion, with a custom display name.
/// - [`AbilityId::P`] represents the passive of a champion
/// - Other variants correspond to the abilities `Q`, `W`, `E`, and `R` (ultimate)
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = types))]
pub enum AbilityId {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl From<Key> for AbilityId {
    fn from(value: Key) -> Self {
        AbilityId::from_key_fn(value)(AbilityName::Void)
    }
}

impl From<AbilityId> for Key {
    fn from(value: AbilityId) -> Self {
        value.as_key()
    }
}

impl AbilityId {
    pub const fn const_eq(self, other: AbilityId) -> bool {
        unsafe {
            let a: [u8; 2] = core::mem::transmute(self);
            let b: [u8; 2] = core::mem::transmute(other);
            a[0] == b[0] && a[1] == b[1]
        }
    }

    pub const fn setter(&self) -> fn(AbilityName) -> Self {
        match self {
            AbilityId::P(_) => AbilityId::P,
            AbilityId::Q(_) => AbilityId::Q,
            AbilityId::W(_) => AbilityId::W,
            AbilityId::E(_) => AbilityId::E,
            AbilityId::R(_) => AbilityId::R,
        }
    }

    pub const fn from_key_fn(key: Key) -> fn(AbilityName) -> Self {
        match key {
            Key::P => AbilityId::P,
            Key::Q => AbilityId::Q,
            Key::W => AbilityId::W,
            Key::E => AbilityId::E,
            Key::R => AbilityId::R,
        }
    }

    pub const fn as_key(&self) -> Key {
        match self {
            AbilityId::P(_) => Key::P,
            AbilityId::Q(_) => Key::Q,
            AbilityId::W(_) => Key::W,
            AbilityId::E(_) => Key::E,
            AbilityId::R(_) => Key::R,
        }
    }

    pub const fn as_char(&self) -> char {
        self.as_key().as_char()
    }

    pub const fn ability_name(&self) -> AbilityName {
        match self {
            AbilityId::P(v) => *v,
            AbilityId::Q(v) => *v,
            AbilityId::W(v) => *v,
            AbilityId::E(v) => *v,
            AbilityId::R(v) => *v,
        }
    }

    pub fn discriminant(&self) -> alloc::string::String {
        let letter = self.as_char();
        let ability_name = self.ability_name().short();
        alloc::format!("{letter}{ability_name}")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Encode, Hash, PartialEq, PartialOrd, Serialize)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = types))]
pub enum ComboElement {
    Ability(AbilityId),
    Attack,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    strum::EnumString,
    strum::Display,
    strum::VariantNames,
    strum::IntoStaticStr,
    strum::FromRepr,
)]
#[repr(u8)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = types))]
pub enum AbilityName {
    Void,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    Min,
    _1Min,
    _2Min,
    _3Min,
    _4Min,
    _5Min,
    _6Min,
    _7Min,
    _8Min,
    Max,
    _1Max,
    _2Max,
    _3Max,
    _4Max,
    _5Max,
    _6Max,
    _7Max,
    _8Max,
    Mega,
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
}

impl AbilityName {
    pub const JMP: u8 = Self::Min as u8;

    pub const fn short(&self) -> &'static str {
        match self {
            Self::Void => "",
            Self::_1 => "1",
            Self::_2 => "2",
            Self::_3 => "3",
            Self::_4 => "4",
            Self::_5 => "5",
            Self::_6 => "6",
            Self::_7 => "7",
            Self::_8 => "8",
            Self::Min => "min",
            Self::_1Min => "1min",
            Self::_2Min => "2min",
            Self::_3Min => "3min",
            Self::_4Min => "4min",
            Self::_5Min => "5min",
            Self::_6Min => "6min",
            Self::_7Min => "7min",
            Self::_8Min => "8min",
            Self::Max => "max",
            Self::_1Max => "1max",
            Self::_2Max => "2max",
            Self::_3Max => "3max",
            Self::_4Max => "4max",
            Self::_5Max => "5max",
            Self::_6Max => "6max",
            Self::_7Max => "7max",
            Self::_8Max => "8max",
            Self::Mega => "mega",
            Self::Minion => "c",
            Self::Minion1 => "c1",
            Self::Minion2 => "c2",
            Self::Minion3 => "c3",
            Self::MinionMax => "cmax",
            Self::Monster => "m",
            Self::Monster1 => "m1",
            Self::Monster2 => "m2",
            Self::Monster3 => "m3",
            Self::Monster4 => "m4",
            Self::MonsterMax => "mmax",
        }
    }

    pub const fn cast_max(&self) -> Self {
        let byte = *self as u8;
        assert!(byte < Self::Mega as u8 - 1);

        match byte >= Self::Max as u8 {
            true => *self,
            false if byte < Self::Min as u8 => Self::from_repr(byte + (Self::JMP << 1)).unwrap(),
            false => Self::from_repr(byte + Self::JMP).unwrap(),
        }
    }

    pub const fn cast_min(&self) -> Self {
        Self::from_repr(self.cast_max() as u8 - Self::JMP).unwrap()
    }
}
