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
        let x = axis_as_f32(axis);
        return Ok(Some(use_formula.replace('x', &x)));
    }

    if let Some(base) = &effect.base {
        if let Some(expr) = simplify_series_f64(base, axis)? {
            return Ok(Some(expr));
        }
        return Ok(None);
    }

    if let Some(use_values) = &effect.use_values {
        if let Some(expr) = simplify_series_f64(use_values, axis)? {
            return Ok(Some(expr));
        }
        return Ok(None);
    }

    Ok(None)
}

fn render_scaling(axis: CtxVar, scaling: &Scaling) -> MayFail<String> {
    match scaling {
        Scaling::Simple { value, ctx_var } => Ok(piece_to_expr(Piece {
            coeff: render_f32(*value),
            target: TargetMode::Direct(*ctx_var),
        })),

        Scaling::PercentAttr { value, ctx_var, .. } => Ok(piece_to_expr(Piece {
            coeff: render_f32(*value),
            target: TargetMode::Direct(*ctx_var),
        })),

        Scaling::Per100 { value, ctx_var } => Ok(piece_to_expr(Piece {
            coeff: render_f32(*value),
            target: TargetMode::Per100(*ctx_var),
        })),

        Scaling::Flat { values } => {
            let coeff = simplify_series_f32(values, axis)?
                .ok_or("[formula] Flat scaling is Unknown/Inconclusive")?;

            Ok(coeff)
        }

        Scaling::Ranked { values, ctx_var } => {
            let coeff = simplify_series_f32(values, axis)?
                .ok_or("[formula] Ranked scaling is Unknown/Inconclusive")?;

            Ok(piece_to_expr(Piece {
                coeff,
                target: TargetMode::Direct(*ctx_var),
            }))
        }

        Scaling::RankedPer100 { values, ctx_var } => {
            let coeff = simplify_series_f32(values, axis)?
                .ok_or("[formula] RankedPer100 scaling is Unknown/Inconclusive")?;

            Ok(piece_to_expr(Piece {
                coeff,
                target: TargetMode::Per100(*ctx_var),
            }))
        }

        Scaling::BasedOnLevel {
            level_var,
            arms,
            ctx_var,
            ..
        } => {
            let coeff = render_level_match(*level_var, arms);

            Ok(piece_to_expr(Piece {
                coeff,
                target: TargetMode::Direct(*ctx_var),
            }))
        }

        Scaling::Nested { outer, inner, .. } => render_nested(axis, outer, inner),

        Scaling::Other { raw } => {
            Err(format!("[formula] Unsupported Scaling::Other: {raw}").into())
        }
    }
}

fn render_nested(axis: CtxVar, outer: &Scaling, inner: &[Scaling]) -> MayFail<String> {
    let outer_piece = render_piece(axis, outer)?;
    let mut coeff = outer_piece.coeff.clone();

    for child in inner {
        let child_piece = render_piece(axis, child)?;
        let child_expr = piece_to_expr(child_piece);
        coeff = add_expr(&coeff, &child_expr);
    }

    Ok(piece_to_expr(Piece {
        coeff,
        target: outer_piece.target,
    }))
}

fn render_piece(axis: CtxVar, scaling: &Scaling) -> MayFail<Piece> {
    match scaling {
        Scaling::Simple { value, ctx_var } => Ok(Piece {
            coeff: render_f32(*value),
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::PercentAttr { value, ctx_var, .. } => Ok(Piece {
            coeff: render_f32(*value),
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::Per100 { value, ctx_var } => Ok(Piece {
            coeff: render_f32(*value),
            target: TargetMode::Per100(*ctx_var),
        }),

        Scaling::Flat { values } => {
            let coeff = simplify_series_f32(values, axis)?
                .ok_or("[formula] Flat scaling is Unknown/Inconclusive")?;

            Ok(Piece {
                coeff,
                target: TargetMode::None,
            })
        }

        Scaling::Ranked { values, ctx_var } => {
            let coeff = simplify_series_f32(values, axis)?
                .ok_or("[formula] Ranked scaling is Unknown/Inconclusive")?;

            Ok(Piece {
                coeff,
                target: TargetMode::Direct(*ctx_var),
            })
        }

        Scaling::RankedPer100 { values, ctx_var } => {
            let coeff = simplify_series_f32(values, axis)?
                .ok_or("[formula] RankedPer100 scaling is Unknown/Inconclusive")?;

            Ok(Piece {
                coeff,
                target: TargetMode::Per100(*ctx_var),
            })
        }

        Scaling::BasedOnLevel {
            level_var,
            arms,
            ctx_var,
            ..
        } => Ok(Piece {
            coeff: render_level_match(*level_var, arms),
            target: TargetMode::Direct(*ctx_var),
        }),

        Scaling::Nested { outer, inner, .. } => {
            let expr = render_nested(axis, outer, inner)?;
            Ok(Piece {
                coeff: expr,
                target: TargetMode::None,
            })
        }

        Scaling::Other { raw } => {
            Err(format!("[formula] Unsupported Scaling::Other: {raw}").into())
        }
    }
}

fn piece_to_expr(piece: Piece) -> String {
    match piece.target {
        TargetMode::None => piece.coeff,
        TargetMode::Direct(ctx_var) => format!("({}) * {}", piece.coeff, ctx_var),
        TargetMode::Per100(ctx_var) => format!("({}) * ({} * 0.01)", piece.coeff, ctx_var),
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
        Key::P => {
            if len == 0 {
                Some(CtxVar::Level)
            } else {
                None
            }
        }
    }
}

fn simplify_series_f32(values: &[f32], axis: CtxVar) -> MayFail<Option<String>> {
    let vec = values.iter().map(|v| *v as f64).collect::<Vec<_>>();
    simplify_series_f64(&vec, axis)
}

fn simplify_series_f64(values: &[f64], axis: CtxVar) -> MayFail<Option<String>> {
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
    let a = values[0];
    let d1 = values[1] - values[0];
    let n = axis_offset(axis);

    combine_terms([
        Some(render_f64(a)),
        if approx_zero(d1) {
            None
        } else {
            Some(format!("{} * {}", render_f64(d1), n))
        },
    ])
}

fn render_quadratic(values: &[f64], axis: CtxVar) -> String {
    let a = values[0];
    let d1 = values[1] - values[0];
    let d2 = values[2] - 2.0 * values[1] + values[0];
    let n = axis_offset(axis);

    combine_terms([
        Some(render_f64(a)),
        if approx_zero(d1) {
            None
        } else {
            Some(format!("{} * {}", render_f64(d1), n))
        },
        if approx_zero(d2) {
            None
        } else {
            Some(format!("{} * {} * ({} - 1.0) * 0.5", render_f64(d2), n, n))
        },
    ])
}

fn render_cubic(values: &[f64], axis: CtxVar) -> String {
    let a = values[0];
    let d1 = values[1] - values[0];
    let d2 = values[2] - 2.0 * values[1] + values[0];
    let d3 = values[3] - 3.0 * values[2] + 3.0 * values[1] - values[0];
    let n = axis_offset(axis);

    combine_terms([
        Some(render_f64(a)),
        if approx_zero(d1) {
            None
        } else {
            Some(format!("{} * {}", render_f64(d1), n))
        },
        if approx_zero(d2) {
            None
        } else {
            Some(format!("{} * {} * ({} - 1.0) * 0.5", render_f64(d2), n, n))
        },
        if approx_zero(d3) {
            None
        } else {
            Some(format!(
                "{} * {} * ({} - 1.0) * ({} - 2.0) / 6.0",
                render_f64(d3),
                n,
                n,
                n
            ))
        },
    ])
}

fn render_level_match(level_var: CtxVar, arms: &[LevelArm]) -> String {
    let mut rendered = Vec::new();

    for arm in arms {
        match arm {
            LevelArm::To {
                end_exclusive,
                value,
            } => rendered.push(format!("..{end_exclusive} => {}", render_f32(*value))),
            LevelArm::Range {
                start_inclusive,
                end_exclusive,
                value,
            } => rendered.push(format!(
                "{start_inclusive}..{end_exclusive} => {}",
                render_f32(*value)
            )),
            LevelArm::From {
                start_inclusive,
                value,
            } => rendered.push(format!("{start_inclusive}.. => {}", render_f32(*value))),
        }
    }

    rendered.push("_ => 0.0".to_string());

    format!("match {} as u8 {{ {} }}", level_var, rendered.join(", "))
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

fn axis_as_f32(axis: CtxVar) -> String {
    format!("({axis} as f32)")
}

fn axis_offset(axis: CtxVar) -> String {
    format!("(({} as f32) - 1.0)", axis)
}

fn combine_terms<const N: usize>(terms: [Option<String>; N]) -> String {
    let mut out = String::new();

    for term in terms.into_iter().flatten() {
        if out.is_empty() {
            out = term;
        } else {
            out = format!("({out}) + ({term})");
        }
    }

    if out.is_empty() {
        "0.0".to_string()
    } else {
        out
    }
}

fn render_f32(v: f32) -> String {
    render_f64(v as f64)
}

fn render_f64(v: f64) -> String {
    let s = v.to_string();
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        format!("{s}.0")
    }
}

fn approx_zero(v: f64) -> bool {
    v.abs() < 1e-9
}
