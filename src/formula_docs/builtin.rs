use super::FormulaRenderer;
use alloc::string::String;
use tutorlolv2_codec::{Bracket, Class, GenericHighlighter, Html, Markup};
use tutorlolv2_types::{AdaptiveType, CtxVar};

impl FormulaRenderer {
    pub fn ignite_fn_markup<M: Markup>() -> M {
        let damage = crate::libgen::IGNITE_DAMAGE_BY_LEVEL;
        let last = damage.len() - 1;
        let step = damage[last] - damage[last - 1];
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Keyword, "fn")
            .text(" ")
            .push(Class::Function, "ignite")
            .bracket(Bracket::LParen)
            .push(Class::Variable, "level")
            .text(": ")
            .push(Class::Type, "NonZeroU8")
            .bracket(Bracket::RParen)
            .text(" -> ")
            .push(Class::Primitive, "i32")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Keyword, "let")
            .text(" ")
            .push(Class::Variable, "n")
            .text(" = ")
            .push(Class::Variable, "level")
            .text(".")
            .push(Class::Function, "get")
            .bracket(Bracket::LParen)
            .bracket(Bracket::RParen)
            .text(" ")
            .push(Class::Keyword, "as")
            .text(" ")
            .push(Class::Primitive, "i32")
            .text(";\n\t")
            .push(Class::Control, "match")
            .text(" ")
            .push(Class::Variable, "n")
            .text(" ")
            .bracket(Bracket::LCurly);

        for (index, value) in damage[..last].iter().enumerate() {
            h.text("\n\t\t")
                .push(Class::Number, index + 1)
                .text(" => ")
                .push(Class::Number, value)
                .text(",");
        }

        h.text("\n\t\t_ => ")
            .push(Class::Number, damage[last])
            .text(" + ")
            .push(Class::Number, step)
            .text(" * ")
            .bracket(Bracket::LParen)
            .push(Class::Variable, "n")
            .text(" - ")
            .push(Class::Number, damage.len())
            .bracket(Bracket::RParen)
            .text(",\n\t")
            .bracket(Bracket::RCurly)
            .text("\n")
            .bracket(Bracket::RCurly);
        h.into_markup()
    }

    pub fn onhit_effect_global_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Control, "intrinsic")
            .text(" ")
            .push(Class::Type, "Onhit")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Variable, "damage_type")
            .text(": ")
            .push(Class::Constant, "Mixed")
            .text(",\n\t")
            .push(Class::Variable, "definition")
            .text(": ")
            .push(Class::Keyword, "fn")
            .text(" ")
            .push(Class::Function, "onhit")
            .bracket(Bracket::LParen)
            .text("...")
            .bracket(Bracket::RParen)
            .text(" -> ")
            .push(Class::Type, "Attacks")
            .text("\n")
            .bracket(Bracket::RCurly)
            .text(";");

        h.into_markup()
    }

    pub fn onhit_effect_fn_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Keyword, "fn")
            .text(" ")
            .push(Class::Function, "onhit")
            .bracket(Bracket::LParen)
            .bracket(Bracket::RParen)
            .text(" -> ")
            .push(Class::Type, "Attacks")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Control, "intrinsic")
            .text("\n")
            .bracket(Bracket::RCurly);

        h.into_markup()
    }

    pub fn critical_strike_global_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Control, "intrinsic")
            .text(" ")
            .push(Class::Type, "CritStrike")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Variable, "attributes")
            .text(": ")
            .push(Class::Constant, "OnhitMax")
            .text(",\n\t")
            .push(Class::Variable, "damage_type")
            .text(": ")
            .push(Class::Constant, "Physical")
            .text(",\n\t")
            .push(Class::Variable, "damage")
            .text(": ")
            .push(Class::Variable, "attack_damage")
            .text(" * ")
            .push(Class::Variable, "crit_damage")
            .text(" / ")
            .push(Class::Number, 100)
            .text("\n")
            .bracket(Bracket::RCurly)
            .text(";");

        h.into_markup()
    }

    pub fn critical_strike_fn_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Keyword, "fn")
            .text(" ")
            .push(Class::Function, "critical_strike")
            .bracket(Bracket::LParen)
            .bracket(Bracket::RParen)
            .text(" -> ")
            .push(Class::Primitive, "f32")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Variable, "attack_damage")
            .text(" * ")
            .push(Class::Variable, "crit_damage")
            .text("\n\t  / ")
            .push(Class::Number, 100)
            .text(" ")
            .push(Class::Comment, "* physical_multiplier")
            .text("\n")
            .bracket(Bracket::RCurly);

        h.into_markup()
    }

    pub fn basic_attack_global_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Control, "intrinsic")
            .text(" ")
            .push(Class::Type, "BasicAttack")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Variable, "attributes")
            .text(": ")
            .push(Class::Constant, "OnhitMin")
            .text(",\n\t")
            .push(Class::Variable, "damage_type")
            .text(": ")
            .push(Class::Constant, "Physical")
            .text(",\n\t")
            .push(Class::Variable, "damage")
            .text(": ")
            .push(Class::Variable, "attack_damage")
            .text(" ")
            .push(Class::Comment, "* physical_multiplier")
            .text(",\n")
            .bracket(Bracket::RCurly)
            .text(";");

        h.into_markup()
    }

    pub fn basic_attack_fn_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Keyword, "fn")
            .text(" ")
            .push(Class::Function, "basic_attack")
            .bracket(Bracket::LParen)
            .bracket(Bracket::RParen)
            .text(" -> ")
            .push(Class::Primitive, "f32")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Variable, "attack_damage")
            .text(" ")
            .push(Class::Comment, "* physical_multiplier")
            .text("\n")
            .bracket(Bracket::RCurly);

        h.into_markup()
    }

    pub fn tower_global_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.global_struct("Tower")
            .fragment_field("damage_type", &{
                let mut h = GenericHighlighter::<M>::new();

                h.push(Class::Control, "match")
                    .raw(" ")
                    .push(Class::Number, 0.35)
                    .raw(" * ")
                    .push(Class::Variable, CtxVar::BonusAd.to_snake_case())
                    .raw("\n\t\t")
                    .raw(" | ")
                    .push(Class::Number, 0.2)
                    .raw(" * ")
                    .push(Class::Variable, CtxVar::AbilityPower.to_snake_case())
                    .raw(" ")
                    .bracket(Bracket::LCurly)
                    .raw("\n\t\t")
                    .push(Class::Variable, "lhs")
                    .raw(" > ")
                    .push(Class::Variable, "rhs")
                    .raw(" => ")
                    .push(Class::Constant, AdaptiveType::Physical)
                    .raw(",\n\t\t")
                    .push(Class::Variable, "lhs")
                    .raw(" < ")
                    .push(Class::Variable, "rhs")
                    .raw(" => ")
                    .push(Class::Constant, AdaptiveType::Magic)
                    .raw(",\n\t\t")
                    .push(Class::Control, "else")
                    .raw(" => ")
                    .push(Class::Type, "ChampionId")
                    .raw("::")
                    .push(Class::Function, "adaptive_type")
                    .raw(",\n\t")
                    .bracket(Bracket::RCurly);

                h.into_markup()
            })
            .finish_struct();

        h.into_markup()
    }

    pub fn tower_fn_markup<M: Markup>() -> M {
        let mut h = GenericHighlighter::<M>::new();

        h.push(Class::Keyword, "fn")
            .raw(" ")
            .push(Class::Function, "tower_damage")
            .bracket(Bracket::LParen)
            .raw("...")
            .bracket(Bracket::RParen)
            .raw(" ")
            .bracket(Bracket::LCurly)
            .raw("\n\t")
            .push(Class::Keyword, "let")
            .raw(" ")
            .push(Class::Variable, "base")
            .raw(" = ")
            .push(Class::Variable, CtxVar::BaseAd.to_snake_case())
            .raw("\n\t\t+ ")
            .push(Class::Variable, CtxVar::BonusAd.to_snake_case())
            .raw("\n\t\t+ ")
            .push(Class::Variable, CtxVar::AbilityPower.to_snake_case())
            .raw(" * ")
            .push(Class::Number, 0.6)
            .raw(";\n\n\t")
            .push(Class::Keyword, "let")
            .raw(" ")
            .push(Class::Variable, "bonus_resist")
            .raw(" = ")
            .push(Class::Number, 15)
            .raw(" + ")
            .push(Class::Number, 50)
            .raw(" * ")
            .bracket(Bracket::LParen)
            .push(Class::Variable, "plates")
            .raw(" - ")
            .push(Class::Number, 1)
            .bracket(Bracket::RParen)
            .raw(";\n\n\t")
            .push(Class::Control, "if")
            .raw(" ")
            .push(Class::Variable, "plates")
            .raw(" == ")
            .push(Class::Number, 0)
            .raw(" ")
            .bracket(Bracket::LCurly)
            .raw("\n\t\t")
            .push(Class::Variable, "bonus_resist")
            .raw(" = ")
            .push(Class::Number, 0)
            .raw(";\n\t")
            .bracket(Bracket::RCurly)
            .raw("\n\n\t")
            .push(Class::Keyword, "let")
            .raw(" ")
            .push(Class::Variable, "raw_resist")
            .raw(" = ")
            .push(Class::Number, 40)
            .raw(" + ")
            .push(Class::Variable, "bonus_resist")
            .raw(";\n\t")
            .push(Class::Keyword, "let")
            .raw(" ")
            .push(Class::Variable, "resist")
            .raw(" = ")
            .push(Class::Variable, "raw_resist")
            .raw("\n\t\t* ")
            .bracket(Bracket::LParen)
            .push(Class::Number, 1)
            .raw(" - ")
            .push(Class::Variable, "pen_percent")
            .raw(" / ")
            .push(Class::Number, 100)
            .bracket(Bracket::RParen)
            .raw("\n\t\t- ")
            .push(Class::Variable, "pen_flat")
            .raw(";\n\n\t")
            .push(Class::Keyword, "let")
            .raw(" ")
            .push(Class::Variable, "mult")
            .raw(" = ")
            .push(Class::Number, 100)
            .raw(" / ")
            .bracket(Bracket::LParen)
            .push(Class::Number, 100)
            .raw(" + ")
            .push(Class::Variable, "resist")
            .bracket(Bracket::RParen)
            .raw(";\n\t")
            .push(Class::Variable, "base")
            .raw(" * ")
            .push(Class::Variable, "mult")
            .raw("\n")
            .bracket(Bracket::RCurly);

        h.into_markup()
    }

    // HTML strings, as used by the web frontend.

    pub fn ignite_fn() -> String {
        Self::ignite_fn_markup::<Html>().into_string()
    }

    pub fn onhit_effect_global() -> String {
        Self::onhit_effect_global_markup::<Html>().into_string()
    }

    pub fn onhit_effect_fn() -> String {
        Self::onhit_effect_fn_markup::<Html>().into_string()
    }

    pub fn critical_strike_global() -> String {
        Self::critical_strike_global_markup::<Html>().into_string()
    }

    pub fn critical_strike_fn() -> String {
        Self::critical_strike_fn_markup::<Html>().into_string()
    }

    pub fn basic_attack_global() -> String {
        Self::basic_attack_global_markup::<Html>().into_string()
    }

    pub fn basic_attack_fn() -> String {
        Self::basic_attack_fn_markup::<Html>().into_string()
    }

    pub fn tower_global() -> String {
        Self::tower_global_markup::<Html>().into_string()
    }

    pub fn tower_fn() -> String {
        Self::tower_fn_markup::<Html>().into_string()
    }
}
