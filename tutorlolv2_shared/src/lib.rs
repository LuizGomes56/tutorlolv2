use bincode::Encode;
use serde::Serialize;

#[derive(Copy, Clone, Serialize, Encode)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl AbilityLike {
    pub fn from_str(s: &str) -> String {
        match s.chars().next() {
            Some('P') => AbilityLike::from_str_p(&s).to_string(),
            Some('Q') => AbilityLike::from_str_q(&s).to_string(),
            Some('W') => AbilityLike::from_str_w(&s).to_string(),
            Some('E') => AbilityLike::from_str_e(&s).to_string(),
            Some('R') => AbilityLike::from_str_r(&s).to_string(),
            _ => s.to_string(),
        }
    }
}

macro_rules! impl_key {
    ($field:ident) => {
        paste::paste! {
            impl AbilityLike {
                pub fn [<from_str_ $field>](s: &str) -> &'static str {
                    match s {
                        concat!(stringify!([<$field:upper>]), "1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1)"),
                        concat!(stringify!([<$field:upper>]), "2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2)"),
                        concat!(stringify!([<$field:upper>]), "3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3)"),
                        concat!(stringify!([<$field:upper>]), "4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4)"),
                        concat!(stringify!([<$field:upper>]), "5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5)"),
                        concat!(stringify!([<$field:upper>]), "6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6)"),
                        concat!(stringify!([<$field:upper>]), "7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7)"),
                        concat!(stringify!([<$field:upper>]), "8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8)"),
                        concat!(stringify!([<$field:upper>]), "_MEGA") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Mega)"),
                        concat!(stringify!([<$field:upper>]), "_MAX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Max)"),
                        concat!(stringify!([<$field:upper>]), "_MIN") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Min)"),
                        concat!(stringify!([<$field:upper>]), "_MINION") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion)"),
                        concat!(stringify!([<$field:upper>]), "_MINION_1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion1)"),
                        concat!(stringify!([<$field:upper>]), "_MINION_2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion2)"),
                        concat!(stringify!([<$field:upper>]), "_MINION_3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion3)"),
                        concat!(stringify!([<$field:upper>]), "_MINION_MAX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::MinionMax)"),
                        concat!(stringify!([<$field:upper>]), "_MONSTER") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster)"),
                        concat!(stringify!([<$field:upper>]), "_MONSTER_1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster1)"),
                        concat!(stringify!([<$field:upper>]), "_MONSTER_2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster2)"),
                        concat!(stringify!([<$field:upper>]), "_MONSTER_3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster3)"),
                        concat!(stringify!([<$field:upper>]), "_MONSTER_4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster4)"),
                        concat!(stringify!([<$field:upper>]), "_MONSTER_MAX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::MonsterMax)"),
                        stringify!([<$field:upper>]) => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Void)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7Max)"),
                        concat!(stringify!([<$field:upper>]), "_MAX_8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8Max)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7Min)"),
                        concat!(stringify!([<$field:upper>]), "_MIN_8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8Min)"),
                        _ => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Void)")
                    }
                }
                pub fn [<to_str_ $field>](&self) -> &'static str {
                    match self {
                        AbilityLike::[<$field:upper>](v) => {
                            match v {
                                AbilityName::_1 => concat!(stringify!([<$field:upper>]), "1"),
                                AbilityName::_2 => concat!(stringify!([<$field:upper>]), "2"),
                                AbilityName::_3 => concat!(stringify!([<$field:upper>]), "3"),
                                AbilityName::_4 => concat!(stringify!([<$field:upper>]), "4"),
                                AbilityName::_5 => concat!(stringify!([<$field:upper>]), "5"),
                                AbilityName::_6 => concat!(stringify!([<$field:upper>]), "6"),
                                AbilityName::_7 => concat!(stringify!([<$field:upper>]), "7"),
                                AbilityName::_8 => concat!(stringify!([<$field:upper>]), "8"),
                                AbilityName::Mega => concat!(stringify!([<$field:upper>]), "_MEGA"),
                                AbilityName::Max => concat!(stringify!([<$field:upper>]), "_MAX"),
                                AbilityName::Min => concat!(stringify!([<$field:upper>]), "_MIN"),
                                AbilityName::Minion => concat!(stringify!([<$field:upper>]), "_MINION"),
                                AbilityName::Minion1 => concat!(stringify!([<$field:upper>]), "_MINION_1"),
                                AbilityName::Minion2 => concat!(stringify!([<$field:upper>]), "_MINION_2"),
                                AbilityName::Minion3 => concat!(stringify!([<$field:upper>]), "_MINION_3"),
                                AbilityName::MinionMax => concat!(stringify!([<$field:upper>]), "_MINION_MAX"),
                                AbilityName::Monster => concat!(stringify!([<$field:upper>]), "_MONSTER"),
                                AbilityName::Monster1 => concat!(stringify!([<$field:upper>]), "_MONSTER_1"),
                                AbilityName::Monster2 => concat!(stringify!([<$field:upper>]), "_MONSTER_2"),
                                AbilityName::Monster3 => concat!(stringify!([<$field:upper>]), "_MONSTER_3"),
                                AbilityName::Monster4 => concat!(stringify!([<$field:upper>]), "_MONSTER_4"),
                                AbilityName::MonsterMax => concat!(stringify!([<$field:upper>]), "_MONSTER_MAX"),
                                AbilityName::Void => stringify!([<$field:upper>]),
                                AbilityName::_1Max => concat!(stringify!([<$field:upper>]), "_MAX_1"),
                                AbilityName::_2Max => concat!(stringify!([<$field:upper>]), "_MAX_2"),
                                AbilityName::_3Max => concat!(stringify!([<$field:upper>]), "_MAX_3"),
                                AbilityName::_4Max => concat!(stringify!([<$field:upper>]), "_MAX_4"),
                                AbilityName::_5Max => concat!(stringify!([<$field:upper>]), "_MAX_5"),
                                AbilityName::_6Max => concat!(stringify!([<$field:upper>]), "_MAX_6"),
                                AbilityName::_7Max => concat!(stringify!([<$field:upper>]), "_MAX_7"),
                                AbilityName::_8Max => concat!(stringify!([<$field:upper>]), "_MAX_8"),
                                AbilityName::_1Min => concat!(stringify!([<$field:upper>]), "_MIN_1"),
                                AbilityName::_2Min => concat!(stringify!([<$field:upper>]), "_MIN_2"),
                                AbilityName::_3Min => concat!(stringify!([<$field:upper>]), "_MIN_3"),
                                AbilityName::_4Min => concat!(stringify!([<$field:upper>]), "_MIN_4"),
                                AbilityName::_5Min => concat!(stringify!([<$field:upper>]), "_MIN_5"),
                                AbilityName::_6Min => concat!(stringify!([<$field:upper>]), "_MIN_6"),
                                AbilityName::_7Min => concat!(stringify!([<$field:upper>]), "_MIN_7"),
                                AbilityName::_8Min => concat!(stringify!([<$field:upper>]), "_MIN_8"),
                            }
                        },
                        _ => {
                            "UNDEFINED"
                        }
                    }
                }
            }
        }
    };
}

impl_key!(p);
impl_key!(q);
impl_key!(w);
impl_key!(e);
impl_key!(r);

#[derive(Clone, Copy, Serialize, Encode)]
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
}
