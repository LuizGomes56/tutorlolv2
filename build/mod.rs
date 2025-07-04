use regex::{Captures, Regex};

pub(super) fn transform_expr(expr: &str) -> (String, bool) {
    let re_num = Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap();
    let with_f64 = re_num.replace_all(expr, |caps: &Captures| format!("{}f64", &caps[1]));
    let re_up = Regex::new(r"\b([A-Z][A-Z0-9_]*)\b").unwrap();
    let count_ctx = re_up.find_iter(with_f64.as_ref()).count();
    let result = re_up.replace_all(&with_f64, |caps: &Captures| format!("ctx.{}", &caps[1]));
    (result.into_owned(), count_ctx > 0)
}

pub mod phf_champions;
pub mod phf_items;
pub mod phf_meta;
pub mod phf_names;
pub mod phf_runes;
pub mod wr_formulas;
