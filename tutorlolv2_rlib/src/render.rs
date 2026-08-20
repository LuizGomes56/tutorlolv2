use crate::packer::{EntityKind, render_ch_formula, render_items_or_runes_formula};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use std::fmt::Display;
use strum::EnumString;
use tutorlolv2::{AttackType, ChampionId, DamageIndex, ItemId, RuneId, TypeMetadata};

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

#[derive(EnumString)]
pub enum Bracket {
    /// Symbol: `(`
    RParen,
    /// Symbol: `)`
    LParen,
    /// Symbol: `{`
    RCurly,
    /// Symbol: `}`
    LCurly,
    /// Symbol: `[`
    RBracket,
    /// Symbol: `]`
    LBracket,
}

impl Display for Bracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::RParen => "(",
                Self::LParen => ")",
                Self::RCurly => "{",
                Self::LCurly => "}",
                Self::RBracket => "[",
                Self::LBracket => "]",
            }
        )
    }
}

struct Highlighter {
    inner: String,
    bracket_stack: Vec<Class>,
}

impl Highlighter {
    pub fn into_inner(mut self) -> String {
        let css = include_str!("style.css");
        let head = format!(
            "<html>
                <head>
                    <title>Packer Check</title>
                    <style>
                        {css}
                    </style>
                </head>
                <body>
                    <pre>"
        );
        self.inner.insert_str(0, &head);
        self.inner.push_str("</pre></body></html>");
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

    pub fn struct_field<T: Display, U: Display>(
        &mut self,
        name: &str,
        class: Class,
        array: &[(T, U)],
    ) -> &mut Self {
        self.push(Class::Variable, name)
            .add(": ")
            .bracket(Bracket::RCurly)
            .add("\n\t\t");

        for (i, (field, value)) in array.iter().enumerate() {
            self.push(Class::Variable, field.to_string().to_snake_case())
                .add(": ")
                .add(&Self::span(class, value))
                .new_line();

            if i < array.len() - 1 {
                self.add("\t");
            }
        }

        self.bracket(Bracket::LCurly).new_line()
    }

    pub fn new_line(&mut self) -> &mut Self {
        self.add(",\n\t")
    }

    pub fn global_const(&mut self, name: &str) -> &mut Self {
        self.push(Class::Keyword, "const")
            .add(" ")
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

pub fn render_champion_global(id: ChampionId) -> Result<String, Box<dyn core::error::Error>> {
    let mut h = Highlighter::new();

    h.global_const(id.debug())
        .field("name", Class::String, id.name())
        .field("adaptive_type", Class::Constant, id.adaptive_type())
        .field("attack_type", Class::Constant, id.attack_type())
        .array_field("positions", Class::Type, id.positions());

    for (i, TypeMetadata { kind, .. }) in id.abilities().iter().enumerate() {
        h.push(Class::Variable, kind.discriminant().to_lowercase())
            .add(": ");

        let damage = render_ch_formula(id, *kind)?;

        h.add(&damage);

        if i < id.number_of_abilities() - 1 {
            h.new_line();
        } else {
            h.add(",\n");
        }
    }

    h.bracket(Bracket::LCurly);

    Ok(h.into_inner())
}

pub fn render_item_global(id: ItemId) -> Result<String, Box<dyn core::error::Error>> {
    let mut h = Highlighter::new();

    h.global_const(id.debug())
        .field("name", Class::String, id.name())
        .field("price", Class::Number, id.price())
        .field("tier", Class::Number, id.tier())
        .field("purchasable", Class::Boolean, id.purchasable())
        .array_field("maps", Class::Type, id.maps())
        .struct_field("stats", Class::Number, id.stats());

    if id.deals_damage() {
        for (attack_type, damage_index) in [
            (AttackType::Melee, DamageIndex::Min),
            (AttackType::Melee, DamageIndex::Max),
            (AttackType::Ranged, DamageIndex::Min),
            (AttackType::Ranged, DamageIndex::Max),
        ] {
            if !id.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            if !matches!(attack_type, AttackType::Melee) && matches!(damage_index, DamageIndex::Min)
            {
                h.new_line();
            }

            h.push(
                Class::Variable,
                format!("{attack_type:?}_{damage_index:?}").to_lowercase(),
            )
            .add(": ");

            let slot = 2 * attack_type as u8 + damage_index as u8;

            let damage = render_items_or_runes_formula(EntityKind::Item, id, slot)?;

            h.add(&damage);
        }
    }

    h.add("\n");
    h.bracket(Bracket::LCurly);

    Ok(h.into_inner())
}

pub fn render_rune_global(id: RuneId) -> Result<String, Box<dyn core::error::Error>> {
    let mut h = Highlighter::new();

    h.global_const(id.debug())
        .field("name", Class::String, id.name());

    if id.deals_damage() {
        for (attack_type, damage_index) in [
            (AttackType::Melee, DamageIndex::Min),
            (AttackType::Melee, DamageIndex::Max),
            (AttackType::Ranged, DamageIndex::Min),
            (AttackType::Ranged, DamageIndex::Max),
        ] {
            if !id.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            if !matches!(attack_type, AttackType::Melee) && matches!(damage_index, DamageIndex::Min)
            {
                h.new_line();
            }

            h.push(
                Class::Variable,
                format!("{attack_type:?}_{damage_index:?}").to_lowercase(),
            )
            .add(": ");

            let slot = 2 * attack_type as u8 + damage_index as u8;

            let damage = render_items_or_runes_formula(EntityKind::Rune, id, slot)?;

            h.add(&damage);
        }
    }

    h.add("\n");
    h.bracket(Bracket::LCurly);

    Ok(h.into_inner())
}
