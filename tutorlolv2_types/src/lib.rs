use bincode::Encode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use paste::paste;

#[derive(Copy, Clone, Encode, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl Serialize for AbilityLike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AbilityLike {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(AbilityLike::from_str(&s))
    }
}

impl ToString for AbilityLike {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl AbilityLike {
    pub fn as_char(&self) -> char {
        match self {
            AbilityLike::P(_) => 'P',
            AbilityLike::Q(_) => 'Q',
            AbilityLike::W(_) => 'W',
            AbilityLike::E(_) => 'E',
            AbilityLike::R(_) => 'R',
        }
    }

    pub fn from_str(value: &str) -> Self {
        let first_char = value.chars().next().unwrap();
        let ability_name = &value[2..value.len() - 1];

        macro_rules! match_ability_name {
            ($field:ident) => {
                AbilityLike::$field(AbilityName::from_str(ability_name).unwrap())
            };
        }
        match first_char {
            'P' => match_ability_name!(P),
            'Q' => match_ability_name!(Q),
            'W' => match_ability_name!(W),
            'E' => match_ability_name!(E),
            'R' => match_ability_name!(R),
            _ => panic!("Invalid AbilityLike"),
        }
    }

    fn get_first_and_last_paren(self_string: &str) -> (usize, usize) {
        let mut first_paren = 0;
        let mut last_paren = 0;
        for (i, ch) in self_string.chars().enumerate() {
            if ch == '(' {
                first_paren = i;
            }
            if ch == ')' {
                last_paren = i;
            }
        }
        (first_paren, last_paren)
    }

    pub fn get_ability_name(self_string: &str, parens: (usize, usize)) -> &str {
        let (first_paren, last_paren) = parens;
        self_string.get(first_paren + 1..last_paren).unwrap()
    }

    pub fn get_ability_like(self_string: &str, parens: (usize, usize)) -> &str {
        let (first_paren, _) = parens;
        self_string.get(0..first_paren).unwrap()
    }

    pub fn ability_name(&self) -> String {
        let self_string = self.to_string();
        let parens = Self::get_first_and_last_paren(&self_string);
        let ability_name = Self::get_ability_name(&self_string, parens);
        ability_name.to_string()
    }

    pub fn ability_like(&self) -> String {
        let self_string = self.to_string();
        let parens = Self::get_first_and_last_paren(&self_string);
        let ability_like = Self::get_ability_like(&self_string, parens);
        ability_like.to_string()
    }

    pub fn as_literal(&self) -> String {
        let self_string = self.to_string();
        let parens = Self::get_first_and_last_paren(&self_string);
        let ability_name = Self::get_ability_name(&self_string, parens);
        let ability_like = Self::get_ability_like(&self_string, parens);
        format!(
            "AbilityLike::{}(AbilityName::{})",
            ability_like, ability_name
        )
    }
}

impl ToString for AbilityName {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

macro_rules! enum_from_str {
    ($(#[$meta:meta])* $vis:vis enum $E:ident { $($V:ident),* $(,)? }) => {
        $(#[$meta])*
        $vis enum $E {
            $($V),*
        }
        impl ::core::str::FromStr for $E {
            type Err = String;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    $( ::core::stringify!($V) => Ok(Self::$V), )*
                    _ => unreachable!("Invalid string for enum {}: {}", stringify!($E), s),
                }
            }
        }
    };
}

enum_from_str!(
    #[derive(
        Clone, Copy, Serialize, Deserialize, Encode, Debug, Eq, PartialEq, Hash, PartialOrd, Ord,
    )]
    #[repr(u8)]
    pub enum AbilityName {
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        Mega,
        Max,
        Min,
        Void,
        _1Max,
        _2Max,
        _3Max,
        _4Max,
        _5Max,
        _6Max,
        _7Max,
        _8Max,
        _1Min,
        _2Min,
        _3Min,
        _4Min,
        _5Min,
        _6Min,
        _7Min,
        _8Min,
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
);
