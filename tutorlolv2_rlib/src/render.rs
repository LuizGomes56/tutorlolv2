use heck::ToShoutySnakeCase;
use std::fmt::{Debug, Display};
use tutorlolv2::{ChampionId, TypeMetadata};

use crate::packer::render_ch_formula;

#[derive(Clone, Copy)]
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
    Any,
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

pub enum Bracket {
    /// Symbol: '`(`'
    RParen,
    /// Symbol: '`)`'
    LParen,
    /// Symbol: '`{`'
    RCurly,
    /// Symbol: '`}`'
    LCurly,
    /// Symbol: '`[`'
    RBracket,
    /// Symbol: '`]`'
    LBracket,
}

impl Display for Bracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Bracket::RParen => '(',
            Bracket::LParen => ')',
            Bracket::RCurly => '{',
            Bracket::LCurly => '}',
            Bracket::RBracket => '[',
            Bracket::LBracket => ']',
        };
        write!(f, "{s}")
    }
}

struct Highlighter {
    inner: String,
    bracket_stack: Vec<Class>,
}

impl Highlighter {
    pub fn into_inner(mut self) -> String {
        self.inner
            .insert_str(0, r#"<pre style="white-space: pre-wrap; tab-size: 4">"#);
        self.inner.push_str("<pre>");
        self.inner
    }

    pub fn push(&mut self, class: Class, value: impl Display) -> &mut Self {
        let span = Self::span(class, value);
        self.add(&span)
    }

    pub fn add(&mut self, value: &str) -> &mut Self {
        self.inner.push_str(value);
        self
    }

    pub fn span(class: Class, value: impl Display) -> String {
        let n = class as u8;
        format!(r#"<span class="C{n}">{value}</span>"#)
    }

    pub fn string(value: impl Display) -> String {
        Self::span(Class::String, format!("\"{value}\""))
    }

    pub fn array_field<T: Display>(&mut self, name: &str, class: Class, array: &[T]) -> &mut Self {
        self.push(Class::Variable, name)
            .add(": ")
            .bracket(Bracket::RBracket);

        for (i, value) in array.into_iter().enumerate() {
            self.push(class, value);
            if i < array.len() - 1 {
                self.add(", ");
            }
        }

        self.bracket(Bracket::LBracket).new_line()
    }

    pub fn bracket(&mut self, bracket: Bracket) -> &mut Self {
        let class = match bracket {
            Bracket::RParen | Bracket::RCurly | Bracket::RBracket => {
                let class = Class::bracket(self.bracket_stack.len());
                self.bracket_stack.push(class);
                class
            }
            Bracket::LParen | Bracket::LCurly | Bracket::LBracket => {
                self.bracket_stack.pop().unwrap_or(Class::Bracket1)
            }
        };
        self.push(class, bracket)
    }

    pub fn field(&mut self, name: &str, class: Class, value: impl Display) -> &mut Self {
        let v = match class {
            Class::String => Self::string(value),
            _ => Self::span(class, value),
        };

        self.push(Class::Variable, name)
            .add(": ")
            .add(&v)
            .new_line()
    }

    pub fn new_line(&mut self) -> &mut Self {
        self.add(",\n\t")
    }

    pub fn global_const(&mut self, name: &str) -> &mut Self {
        self.push(Class::Keyword, "const ")
            .push(Class::Constant, name.to_shouty_snake_case())
            .add(" = ")
            .bracket(Bracket::RCurly)
            .add("\n\t")
    }

    pub fn new() -> Self {
        Self {
            inner: String::with_capacity(1 << 12),
            bracket_stack: Vec::new(),
        }
    }
}

pub fn render_champion_global(id: ChampionId) -> String {
    let mut h = Highlighter::new();

    h.global_const(id.debug())
        .field("name", Class::String, id.name())
        .field("adaptive_type", Class::Constant, id.adaptive_type())
        .field("attack_type", Class::Constant, id.attack_type())
        .array_field("positions", Class::Type, id.positions());

    for (i, TypeMetadata { kind, .. }) in id.abilities().iter().enumerate() {
        h.push(Class::Variable, kind.discriminant().to_lowercase())
            .add(": ");

        let damage = render_ch_formula(id, *kind).unwrap();

        h.add(&damage);

        if i < id.number_of_abilities() - 1 {
            h.new_line();
        } else {
            h.add(",\n");
        }
    }

    h.bracket(Bracket::LCurly);
    h.into_inner()
}

pub fn render_item_global() -> String {
    // static {var_name}: X = X {{
    //     name: {name},
    //     price: {price},
    //     stats: {stats:?},
    //     maps: {maps:?},
    //     tier: {tier},
    //     purchasable: {purchasable},
    //     {damage}
    // }};
    todo!()
}

pub fn render_rune_global() -> String {
    // static {upper_id}: X = X {{
    //     name: {name:?}{damage}
    // }};
    todo!()
}
