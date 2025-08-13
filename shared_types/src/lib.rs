use serde::Serialize;

#[derive(Copy, Clone, Serialize)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
    A,
    C,
    O,
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
                        concat!(stringify!([<$field:upper>]), "MEGA") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Mega)"),
                        concat!(stringify!([<$field:upper>]), "MAX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Max)"),
                        concat!(stringify!([<$field:upper>]), "MIN") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Min)"),
                        concat!(stringify!([<$field:upper>]), "MNX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion)"),
                        concat!(stringify!([<$field:upper>]), "MN1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion1)"),
                        concat!(stringify!([<$field:upper>]), "MN2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion2)"),
                        concat!(stringify!([<$field:upper>]), "MN3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Minion3)"),
                        concat!(stringify!([<$field:upper>]), "MMNX") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::MinionMax)"),
                        concat!(stringify!([<$field:upper>]), "MSTR") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster)"),
                        concat!(stringify!([<$field:upper>]), "MST1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster1)"),
                        concat!(stringify!([<$field:upper>]), "MST2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster2)"),
                        concat!(stringify!([<$field:upper>]), "MST3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster3)"),
                        concat!(stringify!([<$field:upper>]), "MST4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Monster4)"),
                        concat!(stringify!([<$field:upper>]), "MMST") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::MonsterMax)"),
                        stringify!([<$field:upper>]) => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::Void)"),
                        concat!(stringify!([<$field:upper>]), "MAX1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7Max)"),
                        concat!(stringify!([<$field:upper>]), "MAX8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8Max)"),
                        concat!(stringify!([<$field:upper>]), "MIN1") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_1Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN2") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_2Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN3") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_3Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN4") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_4Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN5") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_5Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN6") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_6Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN7") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_7Min)"),
                        concat!(stringify!([<$field:upper>]), "MIN8") => concat!("AbilityLike::", stringify!([<$field:upper>]), "(AbilityName::_8Min)"),
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
                                AbilityName::Mega => concat!(stringify!([<$field:upper>]), "MEGA"),
                                AbilityName::Max => concat!(stringify!([<$field:upper>]), "MAX"),
                                AbilityName::Min => concat!(stringify!([<$field:upper>]), "MIN"),
                                AbilityName::Minion => concat!(stringify!([<$field:upper>]), "MNX"),
                                AbilityName::Minion1 => concat!(stringify!([<$field:upper>]), "MN1"),
                                AbilityName::Minion2 => concat!(stringify!([<$field:upper>]), "MN2"),
                                AbilityName::Minion3 => concat!(stringify!([<$field:upper>]), "MN3"),
                                AbilityName::MinionMax => concat!(stringify!([<$field:upper>]), "MMNX"),
                                AbilityName::Monster => concat!(stringify!([<$field:upper>]), "MSTR"),
                                AbilityName::Monster1 => concat!(stringify!([<$field:upper>]), "MST1"),
                                AbilityName::Monster2 => concat!(stringify!([<$field:upper>]), "MST2"),
                                AbilityName::Monster3 => concat!(stringify!([<$field:upper>]), "MST3"),
                                AbilityName::Monster4 => concat!(stringify!([<$field:upper>]), "MST4"),
                                AbilityName::MonsterMax => concat!(stringify!([<$field:upper>]), "MMST"),
                                AbilityName::Void => stringify!([<$field:upper>]),
                                AbilityName::_1Max => concat!(stringify!([<$field:upper>]), "MAX1"),
                                AbilityName::_2Max => concat!(stringify!([<$field:upper>]), "MAX2"),
                                AbilityName::_3Max => concat!(stringify!([<$field:upper>]), "MAX3"),
                                AbilityName::_4Max => concat!(stringify!([<$field:upper>]), "MAX4"),
                                AbilityName::_5Max => concat!(stringify!([<$field:upper>]), "MAX5"),
                                AbilityName::_6Max => concat!(stringify!([<$field:upper>]), "MAX6"),
                                AbilityName::_7Max => concat!(stringify!([<$field:upper>]), "MAX7"),
                                AbilityName::_8Max => concat!(stringify!([<$field:upper>]), "MAX8"),
                                AbilityName::_1Min => concat!(stringify!([<$field:upper>]), "MIN1"),
                                AbilityName::_2Min => concat!(stringify!([<$field:upper>]), "MIN2"),
                                AbilityName::_3Min => concat!(stringify!([<$field:upper>]), "MIN3"),
                                AbilityName::_4Min => concat!(stringify!([<$field:upper>]), "MIN4"),
                                AbilityName::_5Min => concat!(stringify!([<$field:upper>]), "MIN5"),
                                AbilityName::_6Min => concat!(stringify!([<$field:upper>]), "MIN6"),
                                AbilityName::_7Min => concat!(stringify!([<$field:upper>]), "MIN7"),
                                AbilityName::_8Min => concat!(stringify!([<$field:upper>]), "MIN8"),
                            }
                        },
                        _ => {
                            "UNDF"
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

#[derive(Clone, Copy, Serialize)]
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
