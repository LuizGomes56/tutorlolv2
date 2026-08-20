use {
    crate::scripts::batch::FmtOutput,
    derive_more::{Display, FromStr},
    serde::{Deserialize, Serialize},
    std::{collections::BTreeMap, ops::Range, sync::LazyLock},
    strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr},
    synoptic::{Highlighter, TokOpt},
};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumIter,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "lowercase")]
#[display(rename_all = "lowercase")]
pub enum Keyword {
    As,
    Void,
    Pub,
    Use,
    Crate,
    Mut,
    Static,
    Ref,
    Dyn,
    Unsafe,
    Extern,
    Type,
    Super,
    Mod,
    Struct,
    Const,
    Enum,
    Fn,
    Let,
    Impl,
    Trait,
    Where,
    #[strum(serialize = "Self")]
    SelfUpper,
    #[strum(serialize = "self")]
    SelfLower,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "lowercase")]
#[display(rename_all = "lowercase")]
pub enum Primitive {
    Bool,
    Usize,
    U8,
    U16,
    U32,
    U64,
    Isize,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Str,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "lowercase")]
#[display(rename_all = "lowercase")]
pub enum Control {
    Break,
    Continue,
    Intrinsic,
    Loop,
    Match,
    Return,
    Yield,
    For,
    While,
    If,
    Else,
    In,
    Impossible,
    Unrecognized,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
pub enum Constant {
    Some,
    None,
    Melee,
    Ranged,
    Physical,
    Undefined,
    Unknown,
    Unspecified,
    Mixed,
    True,
    Adaptive,
    Onhit,
    OnhitMin,
    OnhitMax,
    Area,
    AreaOnhit,
    AreaOnhitMin,
    AreaOnhitMax,
    Magic,
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
    AbilityPower,
    AdaptiveDamage,
    Armor,
    ArmorPenetrationFlat,
    ArmorPenetrationPercent,
    AttackDamage,
    AttackSpeed,
    BaseAd,
    BaseArmor,
    BaseAttackSpeed,
    BaseHealth,
    BaseMagicResist,
    BaseMana,
    BonusAd,
    BonusArmor,
    BonusAttackSpeed,
    BonusHealth,
    BonusMagicResist,
    BonusMana,
    BonusMoveSpeed,
    CritChance,
    CritDamage,
    CurrentHealth,
    CurrentMana,
    Level,
    QLevel,
    WLevel,
    ELevel,
    RLevel,
    MagicMultiplier,
    MagicPenetrationFlat,
    MagicPenetrationPercent,
    MagicResist,
    MaxHealth,
    MaxMana,
    MissingHealth,
    PhysicalMultiplier,
    RanduinEffect,
    RocksolidEffect,
    Stacks,
    SteelcapsEffect,
    LifeSteal,
    EnemyArmor,
    EnemyBonusArmor,
    EnemyBonusHealth,
    EnemyBonusMagicResist,
    EnemyCurrentHealth,
    EnemyMagicResist,
    EnemyMaxHealth,
    EnemyMissingHealth,
    Top,
    Middle,
    Jungle,
    Bottom,
    Support,
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
    UnknownMap33,
    UnknownMap35,
    OneForAll,
    UnsealedSpellbook,
    SwiftPlay,
    AbilityHaste,
    AdaptiveForce,
    ArmorPenetration,
    BaseHealthRegen,
    BaseManaRegen,
    GoldPer10Seconds,
    HealAndShieldPower,
    Health,
    Lethality,
    MagicPenetration,
    Mana,
    MoveSpeed,
    MoveSpeedPercent,
    Omnivamp,
    Tenacity,
    Ability,
    Attack,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
#[display(rename_all = "snake_case")]
pub enum Variable {
    Purchasable,
    Tier,
    Maps,
    Stats,
    Price,
    Positions,
    Name,
    UnstableFeatures,
    AdaptiveType,
    AttackType,
    Damage,
    DamageType,
    Attributes,
    Comment,
    MinDmg,
    MaxDmg,
    MeleeMinDmg,
    MeleeMaxDmg,
    RangedMinDmg,
    RangedMaxDmg,
    AbilityPower,
    AdaptiveDamage,
    Armor,
    ArmorPenetrationFlat,
    ArmorPenetrationPercent,
    AttackDamage,
    AttackSpeed,
    BaseAd,
    BaseArmor,
    BaseAttackSpeed,
    BaseHealth,
    BaseMagicResist,
    BaseMana,
    BonusAd,
    BonusArmor,
    BonusAttackSpeed,
    BonusHealth,
    BonusMagicResist,
    BonusMana,
    BonusMoveSpeed,
    CritChance,
    CritDamage,
    CurrentHealth,
    CurrentMana,
    Level,
    QLevel,
    WLevel,
    ELevel,
    RLevel,
    MagicMultiplier,
    MagicPenetrationFlat,
    MagicPenetrationPercent,
    MagicResist,
    MaxHealth,
    MaxMana,
    MissingHealth,
    PhysicalMultiplier,
    RanduinEffect,
    RocksolidEffect,
    Stacks,
    SteelcapsEffect,
    LifeSteal,
    EnemyArmor,
    EnemyBonusArmor,
    EnemyBonusHealth,
    EnemyBonusMagicResist,
    EnemyCurrentHealth,
    EnemyMagicResist,
    EnemyMaxHealth,
    EnemyMissingHealth,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "PascalCase")]
#[display(rename_all = "PascalCase")]
pub enum Type {
    Generator,
    Key,
    MayFail,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
#[display(rename_all = "snake_case")]
pub enum Function {
    Generate,
    Warn,
    Ability,
    AbilityNth,
    End,
    Combo,
    CloneTo,
    CloneWith,
    MergeDamage,
    MergeSum,
    Times,
    Formula,
    Plus,
    Modify,
    Insert,
    Delete,
    Replace,
    AsVar,
    IntoIter,
    Map,
    Render,
    Collect,
    Join,
    ToString,
    Clone,
    DamageOf,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Serialize,
)]
pub enum Macro {
    #[strum(serialize = "concat!")]
    Concat,
}

#[derive(Clone, Copy, Debug, Deserialize, Display, IntoStaticStr, PartialEq, Serialize)]
pub enum KnownToken {
    Keyword(Keyword),
    Primitive(Primitive),
    Control(Control),
    Constant(Constant),
    Variable(Variable),
    Type(Type),
    Function(Function),
    Macro(Macro),
}

impl KnownToken {
    pub fn check(text: &str) -> Option<Op> {
        fn parse<T: core::str::FromStr>(
            text: &str,
            constructor: fn(T) -> KnownToken,
        ) -> Option<KnownToken> {
            text.parse::<T>().ok().map(constructor)
        }

        None.or_else(|| parse(text, Self::Keyword))
            .or_else(|| parse(text, Self::Primitive))
            .or_else(|| parse(text, Self::Control))
            .or_else(|| parse(text, Self::Constant))
            .or_else(|| parse(text, Self::Variable))
            .or_else(|| parse(text, Self::Type))
            .or_else(|| parse(text, Self::Function))
            .or_else(|| parse(text, Self::Macro))
            .map(Op::Known)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, EnumString, IntoStaticStr, PartialEq, Serialize)]
pub enum Bracket {
    #[strum(serialize = "(")]
    RParen,
    #[strum(serialize = ")")]
    LParen,
    #[strum(serialize = "{")]
    RCurly,
    #[strum(serialize = "}")]
    LCurly,
    #[strum(serialize = "[")]
    RBracket,
    #[strum(serialize = "]")]
    LBracket,
}

#[derive(Clone, Copy, Debug, Deserialize, IntoStaticStr, PartialEq, Serialize)]
pub enum Op {
    Span { class: Class, len: u8 },
    Raw(u8),
    Bracket { class: Class, this: Bracket },
    Known(KnownToken),
    Space(u8),
}

#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, FromStr, IntoStaticStr, PartialEq, Serialize,
)]
pub enum Class {
    Comment,
    String,
    Lifetime,
    Keyword,
    Control,
    Constant,
    Type,
    Primitive,
    Number,
    Boolean,
    Macro,
    Function,
    Variable,
    Bracket1,
    Bracket2,
    Bracket3,
}

impl Class {
    pub const fn bracket(len: usize) -> Self {
        match len % 3 + 1 {
            1 => Class::Bracket1,
            2 => Class::Bracket2,
            3 => Class::Bracket3,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Builder {
    pub source: String,
    pub ops: Vec<Op>,
    bracket_stack: Vec<Class>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            ops: Vec::new(),
            bracket_stack: Vec::new(),
        }
    }

    pub fn batch(&mut self, batch: &mut BTreeMap<String, BTreeMap<String, Vec<FmtOutput>>>) {
        for value in batch.values_mut() {
            for data in value.values_mut() {
                for output in data.iter_mut() {
                    if !output.json.default {
                        output.range = self.merge(&output.builder);
                    }
                }
            }
        }
    }

    pub fn merge(&mut self, ir: &Self) -> Range<usize> {
        let start = self.ops.len();

        self.source.push_str(&ir.source);
        self.ops.extend(ir.ops.clone());

        start..self.ops.len()
    }

    pub fn bracket(&mut self, this: Bracket) {
        let class = match this {
            Bracket::RParen | Bracket::RCurly | Bracket::RBracket => {
                let class = Class::bracket(self.bracket_stack.len());
                self.bracket_stack.push(class);
                class
            }
            Bracket::LParen | Bracket::LCurly | Bracket::LBracket => {
                self.bracket_stack.pop().unwrap_or(Class::Bracket1)
            }
        };
        self.ops.push(Op::Bracket { class, this });
    }

    pub fn raw(&mut self, text: &str) {
        let length = text.len() as _;

        self.source.push_str(text);
        if let Some(op) = self.ops.last_mut()
            && let Op::Raw(len) = op
        {
            *len += length;
            return;
        }

        self.ops.push(Op::Raw(length));
    }

    pub fn span(&mut self, class: Class, text: &str) {
        if let Some(op) = KnownToken::check(text) {
            return self.ops.push(op);
        }

        let start = self.source.len();

        self.source.push_str(text);
        self.ops.push(Op::Span {
            class,
            len: (self.source.len() - start) as _,
        });
    }

    pub fn space(&mut self, n: u8) {
        if n < size_of::<Op>() as _ {
            self.raw(&" ".repeat(n as _));
        } else {
            self.ops.push(Op::Space(n));
        }
    }
}

static RUST_HIGHLIGHTER: LazyLock<Highlighter> = LazyLock::new(|| {
    let mut h = Highlighter::new(4);

    h.bounded("Comment", r"/\*", r"\*/", false);
    h.keyword("Comment", r"//.*$");
    h.bounded_interp("String", "\"", "\"", "\\{", "\\}", true);
    h.keyword("Lifetime", r"'\w+");

    trait RegexAdd: IntoEnumIterator + ToString {
        fn regadd(postfix: Option<&str>) -> String {
            format!(
                r"\b({})\b",
                Self::iter()
                    .map(|a| {
                        let mut s = a.to_string();
                        if let Some(postfix) = postfix {
                            s.push_str(postfix);
                        }
                        s
                    })
                    .collect::<Vec<_>>()
                    .join("|")
            )
        }
    }

    impl<T: IntoEnumIterator + ToString> RegexAdd for T {}

    h.keyword("Keyword", &Keyword::regadd(None));
    h.keyword("Control", &Control::regadd(None));
    h.keyword("Constant", r"::[A-Z_][A-Za-z0-9_]*\b");
    h.keyword("Constant", r"\b[A-Z][A-Z0-9_]*\b");
    h.keyword("Constant", &Constant::regadd(None));
    h.keyword("Type", r"\b[A-Z][a-zA-Z0-9]*\b");
    h.keyword("Type", &Type::regadd(None));
    h.keyword("Primitive", &Primitive::regadd(None));
    h.keyword("Number", r"\b(?:0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?\b");
    h.keyword("Boolean", r"\b(true|false)\b");
    h.keyword("Macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("Function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("Function", r"\b(zero)\b");
    h.keyword("Function", r"\b([a-z][a-zA-Z0-9_]*)\s*\(");
    h.keyword("Function", &Function::regadd(Some("\\(")));
    h.keyword("Variable", r"\b[a-z][a-zA-Z0-9_]*\b");
    h.keyword("Variable", &Variable::regadd(None));
    h
});

pub fn rust_html(input: &str) -> Builder {
    let code = input.lines().map(str::to_string).collect::<Vec<_>>();

    let mut h = RUST_HIGHLIGHTER.clone();
    h.run(&code);

    let mut b = Builder::new();

    for (i, line) in code.iter().enumerate() {
        for token in h.line(i, line) {
            match token {
                TokOpt::Some(text, k) if let Ok(kind) = k.parse() => match kind {
                    Class::Variable if text.ends_with("__fn__") => {
                        b.span(Class::Function, &text[..text.len() - "__fn__".len()]);
                    }
                    Class::Function if text.ends_with('(') => {
                        let name = &text[..text.len() - 1];

                        b.span(kind, name);
                        b.bracket(Bracket::RParen);
                    }
                    Class::String => {
                        let mut start = 0;

                        for (i, ch) in text.char_indices() {
                            if ch == '{' || ch == '}' {
                                if start != i {
                                    b.span(kind, &text[start..i]);
                                }

                                b.span(Class::Keyword, &text[i..i + ch.len_utf8()]);
                                start = i + ch.len_utf8();
                            }
                        }

                        if start != text.len() {
                            b.span(kind, &text[start..]);
                        }
                    }
                    Class::Constant if text.starts_with("::") => {
                        b.raw("::");
                        b.span(Class::Constant, &text[2..]);
                    }
                    Class::Comment => {
                        let txt = text.trim_matches(|c| c == '*' || c == '/').trim();
                        b.span(kind, txt);
                    }
                    _ => b.span(kind, &text),
                },
                TokOpt::Some(text, _) | TokOpt::None(text) => {
                    let mut buf = String::new();
                    let mut spaces = 0u8;

                    macro_rules! flush_buf {
                        () => {
                            if !buf.is_empty() {
                                b.raw(&buf);
                                buf.clear();
                            }
                        };
                    }
                    macro_rules! flush_spaces {
                        () => {
                            if spaces > 0 {
                                b.space(spaces);
                                spaces = 0;
                            }
                        };
                    }

                    for ch in text.chars() {
                        if let Some(bracket) = match ch {
                            '(' => Some(Bracket::RParen),
                            ')' => Some(Bracket::LParen),
                            '{' => Some(Bracket::RCurly),
                            '}' => Some(Bracket::LCurly),
                            '[' => Some(Bracket::RBracket),
                            ']' => Some(Bracket::LBracket),
                            _ => None,
                        } {
                            flush_spaces!();
                            flush_buf!();
                            b.bracket(bracket);
                        } else if ch == ' ' {
                            flush_buf!();
                            spaces += 1;
                        } else {
                            flush_spaces!();
                            buf.push(ch);
                        }
                    }
                    flush_spaces!();
                    flush_buf!();
                }
            }
        }

        b.raw("\n");
    }

    b
}
