use lazy_regex::regex;
use regex::Captures;
use syn::{BinOp, Expr};

/// Precision of the numeric literals written back into a formatted formula.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Precision {
    /// Shortest literal that parses back to the same `f32`, which is what the generated code
    /// evaluates. This removes digits that `f32` cannot represent (`clippy::excessive_precision`).
    F32,
    /// Shortest literal that parses back to the same `f64`. Used when the result is evaluated
    /// numerically and must not lose precision, as in `probe_ratio`.
    F64,
}

/// Formats a formula for the generated code: literals keep only the precision an `f32`
/// holds, and parentheses its evaluation order doesn't need are removed. Formulas are not
/// simplified, so they keep the shape they are written in, term by term
pub fn format_formula(f: &str) -> String {
    format_formula_with(f, Precision::F32)
}

/// Same as [`format_formula`], but keeps every digit an `f64` can hold.
pub fn format_formula_exact(f: &str) -> String {
    format_formula_with(f, Precision::F64)
}

pub fn format_formula_with(f: &str, precision: Precision) -> String {
    let formula = regex!(r"\bzero\b").replace_all(f.trim(), "0");

    // Formulas without variables are folded into their value
    if let Ok(expr) = syn::parse_str::<Expr>(&formula)
        && let Some(value) = evaluate(&expr)
    {
        return format_number(value, precision);
    }

    // Otherwise, only the parentheses around the whole formula are removed
    let normalized = normalize_around_matches(&formula)
        .unwrap_or_else(|| strip_outer_parentheses(&formula).to_owned());
    parenthesize_matches(&format_numbers(&normalized, precision))
}

/// [`normalize`] of a formula with `match` blocks, which it can't print: each block is
/// an identifier while the arithmetic around it is normalized, and is put back as it is
fn normalize_around_matches(formula: &str) -> Option<String> {
    let mut blocks = Vec::new();
    let mut swapped = String::new();
    let mut rest = formula;

    while let Some(keyword) = regex!(r"\bmatch\b").find(rest) {
        let open = rest[keyword.end()..].find('{')? + keyword.end();
        let close = closing_brace(rest, open)?;

        swapped.push_str(&rest[..keyword.start()]);
        swapped.push_str(&format!("__match_{}__", blocks.len()));
        blocks.push(&rest[keyword.start()..=close]);

        rest = &rest[close + 1..];
    }

    swapped.push_str(rest);

    let normalized = normalize(&swapped)?;

    Some(
        regex!(r"__match_(\d+)__")
            .replace_all(&normalized, |caps: &Captures| {
                caps[1]
                    .parse::<usize>()
                    .map_or("", |i| blocks[i])
                    .to_owned()
            })
            .into_owned(),
    )
}

/// `formula` without the parentheses that wrap all of it, as `((a) + (b))`
fn strip_outer_parentheses(formula: &str) -> &str {
    let mut formula = formula.trim();

    while formula.starts_with('(') && closing(formula, 0, b'(', b')') == Some(formula.len() - 1) {
        formula = formula[1..formula.len() - 1].trim();
    }

    formula
}

/// Wraps each `match` that is an operand in parentheses: a `match` that starts a sum
/// is a statement in Rust, not an expression. A `match` alone, already in parentheses,
/// or as the value of an arm, is kept as it is. Arms are processed too
fn parenthesize_matches(formula: &str) -> String {
    let mut out = String::new();
    let mut rest = formula;

    while let Some(keyword) = regex!(r"\bmatch\b").find(rest) {
        let Some(open) = rest[keyword.end()..].find('{').map(|i| i + keyword.end()) else {
            break;
        };

        let Some(close) = closing_brace(rest, open) else {
            break;
        };

        let block = format!(
            "{}{{{}}}",
            &rest[keyword.start()..open],
            parenthesize_matches(&rest[open + 1..close])
        );

        out.push_str(&rest[..keyword.start()]);

        let before = out.trim_end();
        let after = rest[close + 1..].trim_start();

        // After `;`, as `cold_path();` in a map arm, the match is the block's value
        let standalone = (before.is_empty() || before.ends_with(['(', ',', '>', '{', ';']))
            && (after.is_empty() || after.starts_with([')', ',', '}']));

        match standalone {
            true => out.push_str(&block),
            false => out.push_str(&format!("({block})")),
        }

        rest = &rest[close + 1..];
    }

    out.push_str(rest);
    out
}

/// Index of the `}` that closes the `{` at `open`
fn closing_brace(s: &str, open: usize) -> Option<usize> {
    closing(s, open, b'{', b'}')
}

/// Index of the `right` that closes the `left` at `open`
fn closing(s: &str, open: usize, left: u8, right: u8) -> Option<usize> {
    let mut depth = 0;

    for (i, c) in s.bytes().enumerate().skip(open) {
        if c == left {
            depth += 1;
        } else if c == right {
            depth -= 1;

            if depth == 0 {
                return Some(i);
            }
        }
    }

    None
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

/// Rewrites an arithmetic expression keeping only the parentheses its evaluation order needs.
/// Same-precedence groups on the right side are kept: floating point operations are not
/// associative, and `a * (b * c)` lets the compiler fold `b * c` when both are literals.
pub fn normalize(formula: &str) -> Option<String> {
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

fn format_numbers(formula: &str, precision: Precision) -> String {
    regex!(r"\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?\b")
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
