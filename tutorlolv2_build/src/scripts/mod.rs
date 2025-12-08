pub mod export_champions;
pub mod export_items;
pub mod export_runes;
pub mod utils;

pub mod champions;
pub mod items;

pub use crate::*;
pub use export_champions::*;
pub use export_items::*;
pub use export_runes::*;
pub use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use serde::Deserialize;
pub use std::{
    collections::{BTreeMap, HashMap},
    fs,
};
pub use tutorlolv2_fmt::*;
pub use utils::*;

pub static CRITICAL_STRIKE: &str = r#"pub static CRITICAL_STRIKE: DamageExpression = DamageExpression {
    attributes: Attrs::OnhitMax,
    damage_type: DamageType::Physical,
    minimum_damage: |ctx| {
        ctx.ad * ctx.crit_damage / 100.0
    },
    maximum_damage: zero,
};"#;

pub static BASIC_ATTACK: &str = r#"pub static BASIC_ATTACK: DamageExpression = DamageExpression {
    attributes: Attrs::OnhitMin,
    damage_type: DamageType::Physical,
    minimum_damage: |ctx| ctx.ad,
    maximum_damage: zero,
};"#;

pub trait StringExt {
    fn highlight_rust(&self) -> String;
    fn clear_suffixes(&self) -> String;
    fn invoke_rustfmt(&self, width: usize) -> String;
    fn replace_const(&self) -> String;
    fn read_as_path(&self) -> String;
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
    fn read_as_path(&self) -> String {
        std::fs::read_to_string(cwd!(self)).unwrap()
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
