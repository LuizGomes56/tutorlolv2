use super::FormulaRenderer;
use alloc::string::String;
use tutorlolv2_codec::{Bracket, Class, Highlighter};
use tutorlolv2_types::{AdaptiveType, CtxVar};

impl FormulaRenderer {
    pub fn ignite_fn() -> String {
        let mut h = Highlighter::new();

        h.push(Class::Keyword, "fn")
            .text(" ")
            .push(Class::Function, "ignite")
            .bracket(Bracket::LParen)
            .push(Class::Variable, "level")
            .text(": ")
            .push(Class::Primitive, "i32")
            .bracket(Bracket::RParen)
            .text(" -> ")
            .push(Class::Primitive, "i32")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text("\n\t")
            .push(Class::Number, 70)
            .text(" + ")
            .push(Class::Number, 20)
            .text(" * ")
            .push(Class::Variable, "level")
            .text(" + ")
            .push(Class::Number, 5)
            .text("\n\t  * ")
            .push(Class::Control, "if")
            .text(" ")
            .push(Class::Variable, "level")
            .text(" > ")
            .push(Class::Number, 4)
            .text(" ")
            .bracket(Bracket::LCurly)
            .text(" ")
            .push(Class::Variable, "level")
            .text(" - ")
            .push(Class::Number, 4)
            .text(" ")
            .bracket(Bracket::RCurly)
            .text("\n\t\t")
            .push(Class::Control, "else")
            .text(" ")
            .bracket(Bracket::LCurly)
            .text(" ")
            .push(Class::Number, 0)
            .text(" ")
            .bracket(Bracket::RCurly)
            .text("\n")
            .bracket(Bracket::RCurly);

        h.into_fragment()
    }

    pub fn onhit_effect_global() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }

    pub fn onhit_effect_fn() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }

    pub fn critical_strike_global() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }

    pub fn critical_strike_fn() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }

    pub fn basic_attack_global() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }

    pub fn basic_attack_fn() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }

    pub fn tower_global() -> String {
        let mut h = Highlighter::new();

        h.global_struct("Tower")
            .html_field("damage_type", &{
                let mut h = Highlighter::new();

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

                h.into_fragment()
            })
            .finish_struct();

        h.into_fragment()
    }

    pub fn tower_fn() -> String {
        let mut h = Highlighter::new();

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

        h.into_fragment()
    }
}
