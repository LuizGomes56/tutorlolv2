use crate::{
    client::MayFail,
    formula::SequencePattern,
    parser::{Effect, LevelArm, Scaling},
};
use tutorlolv2_types::{CtxVar, Key};

#[derive(Clone, Debug)]
enum TargetMode {
    None,
    Direct(CtxVar),
    Per100(CtxVar),
}

#[derive(Clone, Debug)]
struct Piece {
    coeff: String,
    target: TargetMode,
}

pub fn simplify_formula(key: Key, effect: &Effect) -> MayFail<Option<String>> {
    simplify_formula_with_axis(key, effect, None)
}

/// Nidalee and Tahm Kench should use this variant
pub fn simplify_formula_with_axis(
    key: Key,
    effect: &Effect,
    axis_override: Option<CtxVar>,
) -> MayFail<Option<String>> {
    let axis = match axis_override.or_else(|| infer_axis_from_effect(key, effect)) {
        Some(v) => v,
        None => return Ok(None),
    };

    let mut expr = match build_base_expr(axis, effect)? {
        Some(v) => v,
        None => "0.0".to_string(),
    };

    for scaling in &effect.scalings {
        let term = render_scaling(axis, scaling)?;
        expr = add_expr(&expr, &term);
    }

    Ok(Some(expr))
}

fn build_base_expr(axis: CtxVar, effect: &Effect) -> MayFail<Option<String>> {
    if let Some(use_formula) = &effect.use_formula {
        return Ok(Some(use_formula.replace('x', &format!("({axis} as f32)"))));
    }

    effect
        .base
        .as_ref()
        .or(effect.use_values.as_ref())
        .map(|series| simplify_series(series, axis))
        .transpose()
        .map(Option::flatten)
}

fn render_scaling(axis: CtxVar, scaling: &Scaling) -> MayFail<String> {
    match scaling {
        Scaling::Simple { value, ctx_var } => Ok(piece_to_expr(Piece {
            coeff: value.to_string(),
            target: TargetMode::Direct(*ctx_var),
        })),

        Scaling::PercentAttr { value, ctx_var, .. } => Ok(piece_to_expr(Piece {
            coeff: value.to_string(),
            target: TargetMode::Direct(*ctx_var),
        })),

        Scaling::Per100 { value, ctx_var } => Ok(piece_to_expr(Piece {
            coeff: value.to_string(),
            target: TargetMode::Per100(*ctx_var),
        })),

        Scaling::Flat { values } => {
            if values.len() == 1 {
                return Ok(values[0].to_string());
            }

            Ok(simplify_series(values, axis)?
                .ok_or("[formula] Flat scaling is Unknown/Inconclusive")?)
        }

        Scaling::Ranked { values, ctx_var } => Ok(piece_to_expr(Piece {
            coeff: simplify_series(&values, axis)?
                .ok_or("[formula] Ranked scaling is Unknown/Inconclusive")?,
            target: TargetMode::Direct(*ctx_var),
        })),

        Scaling::RankedPer100 { values, ctx_var } => Ok(piece_to_expr(Piece {
            coeff: simplify_series(&values, axis)?
                .ok_or("[formula] RankedPer100 scaling is Unknown/Inconclusive")?,
            target: TargetMode::Per100(*ctx_var),
        })),

        Scaling::BasedOnLevel {
            level_var,
            arms,
            ctx_var,
            ..
        } => Ok(piece_to_expr(Piece {
            coeff: render_level_match(*level_var, arms),
            target: TargetMode::Direct(*ctx_var),
        })),

        Scaling::Nested { outer, inner, .. } => render_nested(axis, &outer, &inner),

        Scaling::Other { raw } => {
            Err(format!("[formula] Unsupported Scaling::Other: {raw}").into())
        }
    }
}

fn render_nested(axis: CtxVar, outer: &Scaling, inner: &[Scaling]) -> MayFail<String> {
    let Piece { mut coeff, target } = render_piece(axis, outer)?;

    for child in inner {
        let child_piece = render_piece(axis, child)?;
        let child_expr = piece_to_expr(child_piece);
        coeff = add_expr(&coeff, &child_expr);
    }

    Ok(piece_to_expr(Piece { coeff, target }))
}

fn render_piece(axis: CtxVar, scaling: &Scaling) -> MayFail<Piece> {
    match scaling {
        Scaling::Simple { value, ctx_var } => Ok(Piece {
            coeff: value.to_string(),
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::PercentAttr { value, ctx_var, .. } => Ok(Piece {
            coeff: value.to_string(),
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::Per100 { value, ctx_var } => Ok(Piece {
            coeff: value.to_string(),
            target: TargetMode::Per100(*ctx_var),
        }),

        Scaling::Flat { values } => Ok(Piece {
            coeff: simplify_series(values, axis)?
                .ok_or("[formula] Flat scaling is Unknown/Inconclusive")?,
            target: TargetMode::None,
        }),

        Scaling::Ranked { values, ctx_var } => Ok(Piece {
            coeff: simplify_series(values, axis)?
                .ok_or("[formula] Ranked scaling is Unknown/Inconclusive")?,
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::RankedPer100 { values, ctx_var } => Ok(Piece {
            coeff: simplify_series(values, axis)?
                .ok_or("[formula] RankedPer100 scaling is Unknown/Inconclusive")?,
            target: TargetMode::Per100(*ctx_var),
        }),

        Scaling::BasedOnLevel {
            level_var,
            arms,
            ctx_var,
            ..
        } => Ok(Piece {
            coeff: render_level_match(*level_var, arms),
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::Nested { outer, inner, .. } => Ok(Piece {
            coeff: render_nested(axis, outer, inner)?,
            target: TargetMode::None,
        }),

        Scaling::Other { raw } => {
            Err(format!("[formula] Unsupported Scaling::Other: {raw}").into())
        }
    }
}

fn piece_to_expr(piece: Piece) -> String {
    let Piece { coeff, target } = piece;

    match target {
        TargetMode::None => coeff,
        TargetMode::Direct(ctx_var) => format!("({coeff}) * {ctx_var}"),
        TargetMode::Per100(ctx_var) => format!("({coeff}) * ({ctx_var} * 0.01)"),
    }
}

fn infer_axis_from_effect(key: Key, effect: &Effect) -> Option<CtxVar> {
    let len = effect
        .base
        .as_ref()
        .map(Vec::len)
        .or_else(|| effect.use_values.as_ref().map(Vec::len))
        .unwrap_or(0);

    if matches!(len, 18 | 20) {
        return Some(CtxVar::Level);
    }

    match key {
        Key::Q => Some(CtxVar::QLevel),
        Key::W => Some(CtxVar::WLevel),
        Key::E => Some(CtxVar::ELevel),
        Key::R => Some(CtxVar::RLevel),
        Key::P => match len == 0 {
            true => Some(CtxVar::Level),
            false => None,
        },
    }
}

fn simplify_series(values: &[f64], axis: CtxVar) -> MayFail<Option<String>> {
    let pat = SequencePattern::new(values);

    let expr = match pat {
        SequencePattern::Linear => render_linear(values, axis),
        SequencePattern::Quadratic => render_quadratic(values, axis),
        SequencePattern::Cubic => render_cubic(values, axis),
        SequencePattern::Unknown | SequencePattern::Inconclusive => return Ok(None),
    };

    Ok(Some(expr))
}

fn render_linear(values: &[f64], axis: CtxVar) -> String {
    let &[v0, v1, ..] = values else {
        panic!("render_linear requires 2 values: {values:?}");
    };

    let d1 = v1 - v0;
    let n = axis_offset(axis);

    combine_terms([
        Some(v0.to_string()),
        match approx_zero(d1) {
            true => None,
            false => Some(format!("{d1} * {n}")),
        },
    ])
}

fn render_quadratic(values: &[f64], axis: CtxVar) -> String {
    let &[v0, v1, v2, ..] = values else {
        panic!("render_quadratic requires 3 values");
    };

    let d1 = v1 - v0;
    let d2 = v2 - 2.0 * v1 + v0;
    let n = axis_offset(axis);

    combine_terms([
        Some(v0.to_string()),
        match approx_zero(d1) {
            true => None,
            false => Some(format!("{d1} * {n}")),
        },
        match approx_zero(d2) {
            true => None,
            false => Some(format!("{d2} * {n} * ({n} - 1.0) * 0.5")),
        },
    ])
}

fn render_cubic(values: &[f64], axis: CtxVar) -> String {
    let &[v0, v1, v2, v3, ..] = values else {
        panic!("render_cubic requires 4 values");
    };

    let d1 = v1 - v0;
    let d2 = v2 - 2.0 * v1 + v0;
    let d3 = v3 - 3.0 * v2 + 3.0 * v1 - v0;
    let n = axis_offset(axis);

    combine_terms([
        Some(v0.to_string()),
        match approx_zero(d1) {
            true => None,
            false => Some(format!("{d1} * {n}")),
        },
        match approx_zero(d2) {
            true => None,
            false => Some(format!("{d2} * {n} * ({n} - 1.0) * 0.5")),
        },
        match approx_zero(d3) {
            true => None,
            false => Some(format!("{d3} * {n} * ({n} - 1.0) * ({n} - 2.0) / 6.0")),
        },
    ])
}

fn render_level_match(level_var: CtxVar, arms: &[LevelArm]) -> String {
    let mut rendered = Vec::new();

    for arm in arms {
        match arm {
            LevelArm::To { range, value, .. } => rendered.push(format!("{range:?} => {value}")),
            LevelArm::Range { range, value, .. } => rendered.push(format!("{range:?} => {value}")),
            LevelArm::From { range, value, .. } => rendered.push(format!("{range:?} => {value}")),
        }
    }

    format!("match {level_var} as u8 {{ {} }}", rendered.join(", "))
}

fn add_expr(lhs: &str, rhs: &str) -> String {
    if lhs.trim().is_empty() || lhs.trim() == "0.0" {
        rhs.to_string()
    } else if rhs.trim().is_empty() || rhs.trim() == "0.0" {
        lhs.to_string()
    } else {
        format!("({lhs}) + ({rhs})")
    }
}

fn axis_offset(axis: CtxVar) -> String {
    format!("(({axis} as f32) - 1.0)")
}

fn combine_terms<const N: usize>(terms: [Option<String>; N]) -> String {
    let mut out = String::new();

    for term in terms.into_iter().flatten() {
        match out.is_empty() {
            true => out = term,
            false => out = format!("({out}) + ({term})"),
        }
    }

    match out.is_empty() {
        true => "0.0".to_string(),
        false => out,
    }
}

fn approx_zero(v: f64) -> bool {
    let c = v.abs() < 1e-9;
    if v != 0.0 && c {
        println!("Rounded-NonZero")
    }
    c
}
