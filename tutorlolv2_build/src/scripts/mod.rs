use crate::scripts::utils::{add_f32_postfix, clean_math_expr, clear_suffixes};
use tutorlolv2_fmt::{highlight_rust, invoke_rustfmt};

pub mod champions;
pub mod items;
pub mod model;
pub mod runes;
pub mod utils;

pub static USE_SUPER: &str = "use super::*;";

pub static TOWER_DAMAGE: &str = r#"intrinsic const TOWER_DAMAGE {
    damage_type: RiotFormulas::adaptative_type(
        bonus_stats.attack_damage,
        current_stats.ability_power,
    ),
    damage: intrinsic |plates, percent, flat| {
        base_stats.attack_damage
            + bonus_stats.attack_damage
            + current_stats.ability_power
                * 0.6
                * (100 / (140 + (-25 + 50 * plates) * percent - flat))
    }
}"#;

pub static ONHIT_EFFECT: &str = r#"intrinsic const ONHIT_EFFECT {
    damage_type: DamageType::Mixed,
    definition: |damage, [min, max], attr| match attr {
        Attrs::OnhitMin | Attrs::AreaOnhitMin => *min += damage,
        Attrs::OnhitMax | Attrs::AreaOnhitMax => *max += damage,
        Attrs::Onhit | Attrs::AreaOnhit => {
            *min += damage;
            *max += damage;
        }
    }
};"#;

pub static CRITICAL_STRIKE: &str = r#"intrinsic const CRITICAL_STRIKE {
    attributes: Attrs::OnhitMax,
    damage_type: DamageType::Physical,
    damage: |ctx| ctx.ad * ctx.crit_damage / 100.0,
};"#;

pub static BASIC_ATTACK: &str = r#"intrinsic const BASIC_ATTACK {
    attributes: Attrs::OnhitMin,
    damage_type: DamageType::Physical,
    damage: |ctx| ctx.ad,
};"#;

pub trait StringExt {
    fn highlight_rust(&self) -> String;
    fn clear_suffixes(&self) -> String;
    fn invoke_rustfmt(&self, width: usize) -> String;
    fn replace_const(&self) -> String;
    fn to_screaming_snake_case(&self) -> String;
    fn remove_special_chars(&self) -> String;
    fn clean_math_expr(&self) -> String;
    fn transform_expr(&self) -> (String, bool);
}

impl StringExt for str {
    fn transform_expr(&self) -> (String, bool) {
        add_f32_postfix(self)
    }
    fn clean_math_expr(&self) -> String {
        clean_math_expr(self)
    }
    fn highlight_rust(&self) -> String {
        highlight_rust(self)
    }
    fn clear_suffixes(&self) -> String {
        clear_suffixes(self)
    }
    fn invoke_rustfmt(&self, width: usize) -> String {
        invoke_rustfmt(self, width)
    }
    fn replace_const(&self) -> String {
        self.replacen("class=\"type\"", "class=\"constant\"", 1)
    }
    fn remove_special_chars(&self) -> String {
        tutorlolv2_fmt::remove_special_chars(self)
    }
    fn to_screaming_snake_case(&self) -> String {
        fn is_boundary(c: char) -> bool {
            c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\'')
        }
        fn is_ignorable(c: char) -> bool {
            c == '\''
        }
        let mut out = String::with_capacity(self.len() * 2);
        let mut chars = self.chars().peekable();
        let mut prev_was_alpha = false;
        let mut prev_was_upper = false;
        let mut prev_was_digit = false;
        while let Some(c) = chars.next() {
            if is_ignorable(c) {
                continue;
            }
            if is_boundary(c) {
                if !out.is_empty() && !out.ends_with('_') {
                    out.push('_');
                }
                prev_was_alpha = false;
                prev_was_upper = false;
                prev_was_digit = false;
                continue;
            }
            let is_upper = c.is_ascii_uppercase();
            let is_alpha = c.is_ascii_alphabetic();
            let is_digit = c.is_ascii_digit();
            let next_is_lower = chars
                .peek()
                .map(|n| n.is_ascii_lowercase())
                .unwrap_or(false);
            let need_us = if out.is_empty() || out.ends_with('_') {
                false
            } else if is_upper {
                (prev_was_alpha && !prev_was_upper)
                    || prev_was_digit
                    || (prev_was_upper && next_is_lower)
            } else if is_alpha {
                prev_was_digit
            } else if is_digit {
                prev_was_alpha
            } else {
                false
            };
            if need_us && !out.ends_with('_') {
                out.push('_');
            }
            out.push(c.to_ascii_uppercase());
            prev_was_alpha = is_alpha;
            prev_was_upper = is_upper;
            prev_was_digit = is_digit;
        }
        if out.ends_with('_') {
            out.pop();
        }
        out
    }
}
