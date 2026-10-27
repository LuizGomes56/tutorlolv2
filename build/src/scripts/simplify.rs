use regex::{Captures, Regex};
use std::sync::LazyLock;
use syn::{BinOp, Expr};

/// Relative cost of a division compared to an addition or multiplication.
const DIV_WEIGHT: u32 = 4;

/// Precision of the numeric literals written back into a simplified formula.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Precision {
    /// Shortest literal that parses back to the same `f32`, which is what the generated code
    /// evaluates. This removes digits that `f32` cannot represent (`clippy::excessive_precision`).
    F32,
    /// Shortest literal that parses back to the same `f64`. Used when the result is evaluated
    /// numerically and must not lose precision, as in `probe_ratio`.
    F64,
}

/// Simplifies a formula, keeping the symbolic result only when it is cheaper to evaluate.
pub fn simplify(f: &str) -> String {
    simplify_with(f, Precision::F32)
}

/// Same as [`simplify`], but keeps every digit an `f64` can hold.
pub fn simplify_exact(f: &str) -> String {
    simplify_with(f, Precision::F64)
}

pub fn simplify_with(f: &str, precision: Precision) -> String {
    static ZERO_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bzero\b").unwrap());

    let formula = ZERO_RE.replace_all(f.trim(), "0");
    let (protected, matches) = protect_matches(&formula, precision);

    let original = finish(&protected, &matches, precision);
    let simplified = symbolic(&protected).and_then(|s| finish(&s, &matches, precision));

    match (original, simplified) {
        (Some((original, cost)), Some((simplified, simplified_cost))) => {
            if simplified_cost < cost {
                simplified
            } else {
                original
            }
        }
        (Some((original, _)), None) => original,
        (None, Some((simplified, _))) => simplified,
        (None, None) => restore_matches(&format_numbers(&protected, precision), &matches),
    }
}

/// Formats `value` as the shortest decimal literal that round-trips at the given precision.
pub fn format_number(value: f64, precision: Precision) -> String {
    // Also turns `-0` into `0`.
    let value = if value == 0.0 { 0.0 } else { value };

    match precision {
        Precision::F32 => (value as f32).to_string(),
        Precision::F64 => value.to_string(),
    }
}

/// Returns the `core::f32::consts` constant that `literal` approximates, using the same rule as
/// `clippy::approx_constant`, which denies these literals by default.
pub fn approx_constant(literal: &str) -> Option<&'static str> {
    use std::f64::consts::*;

    const KNOWN: [(f64, &str, usize); 19] = [
        (E, "E", 4),
        (FRAC_1_PI, "FRAC_1_PI", 4),
        (FRAC_1_SQRT_2, "FRAC_1_SQRT_2", 5),
        (FRAC_2_PI, "FRAC_2_PI", 5),
        (FRAC_2_SQRT_PI, "FRAC_2_SQRT_PI", 5),
        (FRAC_PI_2, "FRAC_PI_2", 5),
        (FRAC_PI_3, "FRAC_PI_3", 5),
        (FRAC_PI_4, "FRAC_PI_4", 5),
        (FRAC_PI_6, "FRAC_PI_6", 5),
        (FRAC_PI_8, "FRAC_PI_8", 5),
        (LN_2, "LN_2", 5),
        (LN_10, "LN_10", 5),
        (LOG2_10, "LOG2_10", 5),
        (LOG2_E, "LOG2_E", 5),
        (LOG10_2, "LOG10_2", 5),
        (LOG10_E, "LOG10_E", 5),
        (PI, "PI", 3),
        (SQRT_2, "SQRT_2", 5),
        (TAU, "TAU", 3),
    ];

    let literal = literal.replace('_', "");

    literal.parse::<f64>().ok()?;

    KNOWN
        .iter()
        .find(|&&(constant, _, min_digits)| {
            literal.len() > min_digits
                && (constant.to_string().starts_with(&literal)
                    || format!("{constant:.*}", literal.len() - 2) == literal)
        })
        .map(|&(_, name, _)| name)
}

fn symbolic(formula: &str) -> Option<String> {
    symb_anafis::simplify(&formula.replace("ctx.", "ctx_"), &[], None)
        .ok()
        .map(|r| r.replace("ctx_", "ctx."))
}

/// Turns a candidate into valid Rust and returns it with its evaluation cost.
fn finish(formula: &str, matches: &[String], precision: Precision) -> Option<(String, Cost)> {
    let expanded = expand_powers(formula)?;
    let normalized = normalize(&expanded).unwrap_or(expanded);
    let result = restore_matches(&format_numbers(&normalized, precision), matches);
    let expr = syn::parse_str::<Expr>(&result).ok()?;

    match Cost::of(&expr) {
        (_, true) => {
            let value = format_number(evaluate(&expr)?, precision);
            Some((value, Cost::default()))
        }
        (cost, false) => Some((result, cost)),
    }
}

/// Rewrites an arithmetic expression keeping only the parentheses its evaluation order needs.
/// Same-precedence groups on the right side are kept: floating point operations are not
/// associative, and `a * (b * c)` lets the compiler fold `b * c` when both are literals.
fn normalize(formula: &str) -> Option<String> {
    fn precedence(expr: &Expr) -> u8 {
        match expr {
            Expr::Paren(e) => precedence(&e.expr),
            Expr::Group(e) => precedence(&e.expr),
            Expr::Binary(e) => match e.op {
                BinOp::Add(_) | BinOp::Sub(_) => 1,
                BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => 2,
                _ => 0,
            },
            Expr::Unary(_) => 3,
            _ => 4,
        }
    }

    fn operand(expr: &Expr, min_precedence: u8) -> Option<String> {
        let printed = print(expr)?;

        Some(if precedence(expr) < min_precedence {
            format!("({printed})")
        } else {
            printed
        })
    }

    fn print(expr: &Expr) -> Option<String> {
        use quote::ToTokens;

        Some(match expr {
            Expr::Lit(e) => e.lit.to_token_stream().to_string(),
            Expr::Path(e) => e.path.get_ident()?.to_string(),
            Expr::Field(e) => format!("{}.{}", print(&e.base)?, e.member.to_token_stream()),
            Expr::Paren(e) => print(&e.expr)?,
            Expr::Group(e) => print(&e.expr)?,
            Expr::Call(e) => format!(
                "{}({})",
                print(&e.func)?,
                e.args
                    .iter()
                    .map(print)
                    .collect::<Option<Vec<_>>>()?
                    .join(", ")
            ),
            Expr::Unary(e) if matches!(e.op, syn::UnOp::Neg(_)) => {
                format!("-{}", operand(&e.expr, 3)?)
            }
            Expr::Binary(e) => {
                let precedence = precedence(expr);

                if precedence == 0 {
                    return None;
                }

                let left = operand(&e.left, precedence)?;
                let right = operand(&e.right, precedence + 1)?;

                format!("{left} {} {right}", e.op.to_token_stream())
            }
            _ => return None,
        })
    }

    print(&syn::parse_str(formula).ok()?)
}

/// Evaluates an expression made only of literals and arithmetic.
fn evaluate(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Lit(e) => match &e.lit {
            syn::Lit::Int(n) => n.base10_digits().parse().ok(),
            syn::Lit::Float(n) => n.base10_digits().parse().ok(),
            _ => None,
        },
        Expr::Paren(e) => evaluate(&e.expr),
        Expr::Group(e) => evaluate(&e.expr),
        Expr::Unary(e) if matches!(e.op, syn::UnOp::Neg(_)) => Some(-evaluate(&e.expr)?),
        Expr::Binary(e) => {
            let (left, right) = (evaluate(&e.left)?, evaluate(&e.right)?);

            match e.op {
                BinOp::Add(_) => Some(left + right),
                BinOp::Sub(_) => Some(left - right),
                BinOp::Mul(_) => Some(left * right),
                BinOp::Div(_) => Some(left / right),
                BinOp::Rem(_) => Some(left % right),
                _ => None,
            }
        }
        _ => None,
    }
    .filter(|value| value.is_finite())
}

fn placeholder(i: usize) -> String {
    format!("__MATCH_{i}__")
}

/// Replaces every top-level `match` block with a placeholder symbol the symbolic engine can
/// handle, simplifying the body of each arm on the way.
fn protect_matches(formula: &str, precision: Precision) -> (String, Vec<String>) {
    static MATCH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bmatch\b").unwrap());

    let mut protected = String::new();
    let mut matches = Vec::new();
    let mut rest = formula;

    while let Some(keyword) = MATCH_RE.find(rest) {
        let Some(open) = rest[keyword.end()..].find('{').map(|i| i + keyword.end()) else {
            break;
        };

        let Some(close) = closing_brace(rest, open) else {
            break;
        };

        let scrutinee = rest[keyword.end()..open].trim();
        let arms = split_top_level(&rest[open + 1..close], ',')
            .into_iter()
            .filter(|arm| !arm.trim().is_empty())
            .map(|arm| match arm.split_once("=>") {
                Some((pattern, body)) => {
                    format!("{} => {}", pattern.trim(), simplify_with(body, precision))
                }
                None => arm.trim().to_owned(),
            })
            .collect::<Vec<_>>()
            .join(", ");

        protected.push_str(&rest[..keyword.start()]);
        protected.push_str(&placeholder(matches.len()));
        matches.push(format!("match {scrutinee} {{ {arms} }}"));

        rest = &rest[close + 1..];
    }

    protected.push_str(rest);

    (protected, matches)
}

fn restore_matches(formula: &str, matches: &[String]) -> String {
    static PLACEHOLDER_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"__MATCH_(\d+)__").unwrap());

    PLACEHOLDER_RE
        .replace_all(formula, |caps: &Captures| {
            let whole = caps.get(0).unwrap();
            let block = &matches[caps[1].parse::<usize>().unwrap()];

            let before = formula[..whole.start()].trim_end();
            let after = formula[whole.end()..].trim_start();

            let standalone = (before.is_empty() || before.ends_with(['(', ',']))
                && (after.is_empty() || after.starts_with([')', ',']));

            if standalone {
                block.clone()
            } else {
                format!("({block})")
            }
        })
        .into_owned()
}

fn format_numbers(formula: &str, precision: Precision) -> String {
    static NUMBER_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?\b").unwrap());

    NUMBER_RE
        .replace_all(formula, |caps: &Captures| {
            let original = &caps[0];

            // Integers are left alone so range patterns stay intact.
            if !original.contains(['.', 'e', 'E']) {
                return original.to_owned();
            }

            match original.parse::<f64>() {
                Ok(value) if value.is_finite() => format_number(value, precision),
                _ => original.to_owned(),
            }
        })
        .into_owned()
}

/// Rewrites `base^n` as repeated multiplication. Returns `None` for non-integer exponents,
/// which have no `const fn` equivalent.
fn expand_powers(formula: &str) -> Option<String> {
    let mut s = formula.to_owned();

    while let Some(caret) = s.find('^') {
        let start = base_start(&s[..caret])?;
        let base = s[start..caret].trim().to_owned();

        let tail = &s[caret + 1..];
        let skipped = tail.len() - tail.trim_start().len();
        let tail = tail.trim_start();

        let (raw, len) = if tail.starts_with('(') {
            let close = closing_paren(tail)?;

            (&tail[1..close], close + 1)
        } else {
            let len = tail
                .char_indices()
                .find(|&(i, c)| !(c.is_ascii_digit() || c == '.' || (i == 0 && c == '-')))
                .map_or(tail.len(), |(i, _)| i);

            (&tail[..len], len)
        };

        let exponent = raw.trim().parse::<f64>().ok()?;

        if exponent.fract() != 0.0 || exponent.abs() > 16.0 {
            return None;
        }

        let product = || vec![base.as_str(); exponent.abs() as usize].join(" * ");

        let expanded = match exponent as i32 {
            0 => "1".to_owned(),
            1 => base.clone(),
            n if n > 0 => format!("({})", product()),
            _ => format!("(1 / ({}))", product()),
        };

        s.replace_range(start..caret + 1 + skipped + len, &expanded);
    }

    Some(s)
}

/// Finds where the operand that ends at `head` starts: a parenthesized group (with an optional
/// function name), an identifier, a field access or a number.
fn base_start(head: &str) -> Option<usize> {
    let bytes = head.trim_end().as_bytes();
    let mut i = bytes.len();

    if bytes.last() == Some(&b')') {
        let mut depth = 0;

        loop {
            i = i.checked_sub(1)?;

            match bytes[i] {
                b')' => depth += 1,
                b'(' => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                break;
            }
        }
    }

    while i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || matches!(bytes[i - 1], b'_' | b'.')) {
        i -= 1;
    }

    (i < bytes.len()).then_some(i)
}

fn closing_brace(s: &str, open: usize) -> Option<usize> {
    closing(s, open, b'{', b'}')
}

fn closing_paren(s: &str) -> Option<usize> {
    closing(s, 0, b'(', b')')
}

fn closing(s: &str, open: usize, left: u8, right: u8) -> Option<usize> {
    let mut depth = 0usize;

    for (i, &b) in s.as_bytes().iter().enumerate().skip(open) {
        if b == left {
            depth += 1;
        } else if b == right {
            depth -= 1;

            if depth == 0 {
                return Some(i);
            }
        }
    }

    None
}

fn split_top_level(s: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            c if c == separator && depth == 0 => {
                parts.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }

    parts.push(&s[start..]);
    parts
}

/// Evaluation cost of an expression. Operations whose operands are all literals are folded by
/// the compiler, so they only count towards `total`, which breaks ties in favor of the
/// expression with fewer operations overall.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Cost {
    runtime: u32,
    total: u32,
}

impl Cost {
    fn add(self, other: Self) -> Self {
        Self {
            runtime: self.runtime + other.runtime,
            total: self.total + other.total,
        }
    }

    fn op(self, weight: u32, constant: bool) -> Self {
        Self {
            runtime: self.runtime + if constant { 0 } else { weight },
            total: self.total + weight,
        }
    }

    /// Returns the cost of `expr` and whether it is a compile-time constant.
    fn of(expr: &Expr) -> (Self, bool) {
        let sum = |exprs: &mut dyn Iterator<Item = &Expr>| {
            exprs.fold(Self::default(), |acc, e| acc.add(Self::of(e).0))
        };

        match expr {
            Expr::Lit(_) => (Self::default(), true),
            Expr::Paren(e) => Self::of(&e.expr),
            Expr::Group(e) => Self::of(&e.expr),
            Expr::Cast(e) => Self::of(&e.expr),
            Expr::Unary(e) => {
                let (cost, constant) = Self::of(&e.expr);
                (cost.op(1, constant), constant)
            }
            Expr::Binary(e) => {
                let weight = match e.op {
                    BinOp::Div(_) | BinOp::Rem(_) => DIV_WEIGHT,
                    _ => 1,
                };

                let (left, left_constant) = Self::of(&e.left);
                let (right, right_constant) = Self::of(&e.right);
                let constant = left_constant && right_constant;

                (left.add(right).op(weight, constant), constant)
            }
            Expr::Match(e) => {
                let arms = e.arms.iter().map(|arm| Self::of(&arm.body).0).max();
                let cost = Self::of(&e.expr).0.add(arms.unwrap_or_default());

                (cost.op(1, false), false)
            }
            Expr::Call(e) => (sum(&mut e.args.iter()).op(1, false), false),
            Expr::MethodCall(e) => {
                let receiver = std::iter::once(e.receiver.as_ref());
                (sum(&mut receiver.chain(&e.args)).op(1, false), false)
            }
            _ => (Self::default(), false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_keep_only_the_precision_f32_can_represent() {
        assert_eq!(simplify("0.333333333333 * ctx.ap"), "0.33333334 * ctx.ap");
        assert_eq!(simplify("0.016666666667 * ctx.ap"), "0.016666668 * ctx.ap");
        assert_eq!(simplify("1.4285714286 * ctx.ap"), "1.4285715 * ctx.ap");
        assert_eq!(simplify_exact("0.1 + 0.2"), "0.30000000000000004");
        assert_eq!(simplify("0.1 + 0.2"), "0.3");
        assert_eq!(simplify("2.0000000000001 * ctx.ap"), "2 * ctx.ap");
    }

    #[test]
    fn constant_expressions_are_folded() {
        assert_eq!(simplify("1 / 3"), "0.33333334");
        assert_eq!(simplify("1 - 1"), "0");
        assert_eq!(simplify_exact("2 * 3 + 0.5"), "6.5");
        assert_eq!(simplify("(2 * ctx.ap) / (ctx.ap)"), "2");
    }

    #[test]
    fn original_is_kept_unless_the_simplification_is_cheaper() {
        // symb_anafis rewrites this as `(7 + 0.33*ctx.ap)/21`, which adds a runtime division.
        assert_eq!(
            simplify("0.11 / 7 * ctx.ap + 1 / 3"),
            "0.11 / 7 * ctx.ap + 1 / 3"
        );
        assert_eq!(simplify("ctx.ap * 0.5 * 2"), "ctx.ap");
        assert_eq!(simplify("ctx.ap + zero"), "ctx.ap");
        assert_eq!(simplify("10 + 5 * ctx.q_level"), "10 + 5 * ctx.q_level");
    }

    #[test]
    fn redundant_parentheses_are_removed() {
        assert_eq!(
            simplify("((0.45) * ctx.max_health)"),
            "0.45 * ctx.max_health"
        );
        assert_eq!(
            simplify("(((0.1) * ctx.ap) + ((0.03) * ctx.hp)) + ((0.75) * ctx.ad)"),
            "0.1 * ctx.ap + 0.03 * ctx.hp + 0.75 * ctx.ad"
        );
        assert_eq!(normalize("a - (b - c)").unwrap(), "a - (b - c)");
        assert_eq!(
            normalize("ctx.ap * (0.5 * 2)").unwrap(),
            "ctx.ap * (0.5 * 2)"
        );
        assert_eq!(normalize("(a * b) / c").unwrap(), "a * b / c");
        assert_eq!(normalize("-(a + b) * 2").unwrap(), "-(a + b) * 2");
    }

    #[test]
    fn powers_become_multiplications() {
        assert_eq!(expand_powers("ctx.a^2").unwrap(), "(ctx.a * ctx.a)");
        assert_eq!(
            expand_powers("c*(a*(1 + b))^2").unwrap(),
            "c*((a*(1 + b)) * (a*(1 + b)))"
        );
        assert_eq!(expand_powers("a/b^-1").unwrap(), "a/(1 / (b))");
        assert_eq!(expand_powers("x^0.5"), None);

        let squared = simplify("(ctx.ap + 1) * (ctx.ap + 1)");
        assert!(!squared.contains('^'), "{squared}");
    }

    #[test]
    fn match_arms_are_simplified_and_parenthesized() {
        assert_eq!(
            simplify("match ctx.level { ..=1 => 0.1 + 0.2, 2..5 => 1 / 3, _ => 20 }"),
            "match ctx.level { ..=1 => 0.3, 2..5 => 0.33333334, _ => 20 }"
        );

        let product = simplify("ctx.ap * match ctx.level { 1 => 2, _ => 3 }");
        assert!(
            product.contains("(match ctx.level { 1 => 2, _ => 3 })"),
            "{product}"
        );
        assert!(syn::parse_str::<Expr>(&product).is_ok(), "{product}");

        let nested =
            simplify("match ctx.level { 1 => match ctx.q_level { 1 => 1 / 4, _ => 2 }, _ => 3 }");
        assert_eq!(
            nested,
            "match ctx.level { 1 => match ctx.q_level { 1 => 0.25, _ => 2 }, _ => 3 }"
        );
    }

    #[test]
    fn approximate_constants_follow_clippy() {
        assert_eq!(approx_constant("3.14"), Some("PI"));
        assert_eq!(approx_constant("3.1416"), Some("PI"));
        assert_eq!(approx_constant("3.1415927"), Some("PI"));
        assert_eq!(approx_constant("6.28"), Some("TAU"));
        assert_eq!(approx_constant("3.1"), None);
        assert_eq!(approx_constant("0.785"), None);
        assert_eq!(approx_constant("31.4"), None);
    }
}
