use bincode::Encode;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Serialize, Deserialize, Encode, Debug, Eq, PartialEq, PartialOrd, Hash, Ord,
)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl ToString for AbilityLike {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl AbilityLike {
    pub fn chars(&self) -> char {
        match self {
            AbilityLike::P(_) => 'P',
            AbilityLike::Q(_) => 'Q',
            AbilityLike::W(_) => 'W',
            AbilityLike::E(_) => 'E',
            AbilityLike::R(_) => 'R',
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
        self_string.get(first_paren..last_paren).unwrap()
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
