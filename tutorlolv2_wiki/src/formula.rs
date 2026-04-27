use crate::{
    client::MayFail,
    parser::{Effect, LevelArm, Scaling},
};
use serde::{Deserialize, Serialize};
use tutorlolv2_types::{CtxVar, Key};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Formula {
    /// Already materialized per-slot.
    Vector(Vec<String>),

    /// Single expression driven by ctx vars.
    Expr { len: usize, expr: String },
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    Rank { len: usize, ctx_var: CtxVar },
    Level { len: usize },
    Unknown { len: usize },
}

impl Axis {
    const fn len(self) -> usize {
        match self {
            Self::Rank { len, .. } | Self::Level { len } | Self::Unknown { len } => len,
        }
    }

    const fn ctx_var(self) -> Option<CtxVar> {
        match self {
            Self::Rank { ctx_var, .. } => Some(ctx_var),
            Self::Level { .. } => Some(CtxVar::Level),
            Self::Unknown { .. } => None,
        }
    }
}

pub fn build_effect_formula(key: Key, effect: &Effect) -> MayFail<Formula> {
    let axis = infer_axis(key, effect);

    let mut formula = build_base_formula(axis, effect)?;

    for scaling in &effect.scalings {
        apply_scaling(axis, &mut formula, scaling)?;
    }

    Ok(formula)
}

fn infer_axis(key: Key, effect: &Effect) -> Axis {
    let explicit_len = effect
        .use_values
        .as_ref()
        .map(Vec::len)
        .or_else(|| effect.base.as_ref().map(Vec::len))
        .or_else(|| effect.scalings.iter().find_map(scaling_len));

    let default_len = match key {
        Key::Q | Key::W | Key::E | Key::R => 5,
        Key::P => 0,
    };

    let len = explicit_len.unwrap_or(default_len);

    if let Some(values) = &effect.use_values {
        if matches!(values.len(), 18 | 20) {
            return Axis::Level { len: values.len() };
        }
    }

    if effect
        .scalings
        .iter()
        .any(|v| matches!(v, Scaling::BasedOnLevel { .. }))
    {
        return Axis::Level {
            len: explicit_len.unwrap_or(20),
        };
    }

    match key {
        Key::Q => Axis::Rank {
            len: len.max(5),
            ctx_var: CtxVar::QLevel,
        },
        Key::W => Axis::Rank {
            len: len.max(5),
            ctx_var: CtxVar::WLevel,
        },
        Key::E => Axis::Rank {
            len: len.max(5),
            ctx_var: CtxVar::ELevel,
        },
        Key::R => {
            if matches!(len, 18 | 20) {
                Axis::Level { len }
            } else {
                Axis::Rank {
                    len: len.max(5),
                    ctx_var: CtxVar::RLevel,
                }
            }
        }
        Key::P => {
            if matches!(len, 18 | 20) {
                Axis::Level { len }
            } else {
                Axis::Unknown { len }
            }
        }
    }
}

fn build_base_formula(axis: Axis, effect: &Effect) -> MayFail<Formula> {
    if let Some(use_formula) = &effect.use_formula {
        if let Some(axis_var) = axis.ctx_var() {
            let expr = substitute_x(use_formula, &format!("({axis_var})"));
            return Ok(Formula::Expr {
                len: axis.len().max(1),
                expr,
            });
        }

        if let Some(values) = &effect.use_values {
            return Ok(Formula::Vector(
                values.iter().map(|v| normalize_num_str(v)).collect(),
            ));
        }

        if let Some(values) = &effect.base {
            return Ok(Formula::Vector(
                values.iter().map(|v| normalize_num_str(v)).collect(),
            ));
        }

        return Err(
            "[formula] use_formula exists but no usable axis/use_values/base was found".into(),
        );
    }

    if let Some(values) = &effect.use_values {
        return Ok(Formula::Vector(
            values.iter().map(|v| normalize_num_str(v)).collect(),
        ));
    }

    if let Some(values) = &effect.base {
        return Ok(Formula::Vector(
            values.iter().map(|v| normalize_num_str(v)).collect(),
        ));
    }

    if axis.len() > 0 {
        return Ok(Formula::Vector(vec!["0.0".to_string(); axis.len()]));
    }

    Ok(Formula::Vector(Vec::new()))
}

fn apply_scaling(axis: Axis, formula: &mut Formula, scaling: &Scaling) -> MayFail {
    match formula {
        Formula::Vector(values) => apply_scaling_to_vector(axis, values, scaling),
        Formula::Expr { expr, .. } => apply_scaling_to_expr(axis, expr, scaling),
    }
}

fn apply_scaling_to_vector(axis: Axis, values: &mut Vec<String>, scaling: &Scaling) -> MayFail {
    match scaling {
        Scaling::Simple { .. }
        | Scaling::Per100 { .. }
        | Scaling::PercentAttr { .. }
        | Scaling::BasedOnLevel { .. } => {
            let term = render_scaling_term(axis, scaling)?;
            ensure_len(values, axis.len().max(1));
            for slot in values.iter_mut() {
                *slot = add_term(slot, &term);
            }
            Ok(())
        }
        Scaling::Ranked { values: ranked, .. } | Scaling::RankedPer100 { values: ranked, .. } => {
            let terms = ranked
                .iter()
                .map(|v| render_ranked_value_term(scaling, *v))
                .collect::<MayFail<Vec<_>>>()?;

            if values.is_empty() {
                *values = terms;
                return Ok(());
            }

            if values.len() != terms.len() {
                return Err(format!(
                    "[formula] ranked scaling length mismatch: formula={}, scaling={}",
                    values.len(),
                    terms.len()
                )
                .into());
            }

            for (slot, term) in values.iter_mut().zip(terms) {
                *slot = add_term(slot, &term);
            }

            Ok(())
        }
        Scaling::Flat { values: flat } => {
            let terms = flat.iter().map(|v| render_f32(*v)).collect::<Vec<_>>();

            if values.is_empty() {
                *values = terms;
                return Ok(());
            }

            if values.len() != terms.len() {
                return Err(format!(
                    "[formula] flat scaling length mismatch: formula={}, scaling={}",
                    values.len(),
                    terms.len()
                )
                .into());
            }

            for (slot, term) in values.iter_mut().zip(terms) {
                *slot = add_term(slot, &term);
            }

            Ok(())
        }
        Scaling::Nested { raw, .. } => {
            Err(format!("[formula] Nested scaling not implemented yet: {raw}").into())
        }
        Scaling::Other { raw } => {
            Err(format!("[formula] Other scaling not implemented yet: {raw}").into())
        }
    }
}

fn apply_scaling_to_expr(axis: Axis, expr: &mut String, scaling: &Scaling) -> MayFail {
    let term = match scaling {
        Scaling::Simple { .. }
        | Scaling::Per100 { .. }
        | Scaling::PercentAttr { .. }
        | Scaling::BasedOnLevel { .. } => render_scaling_term(axis, scaling)?,

        Scaling::Ranked { values, .. } | Scaling::RankedPer100 { values, .. } => {
            let axis_var = axis
                .ctx_var()
                .ok_or("[formula] ranked scaling needs a known axis var")?;

            let terms = values
                .iter()
                .map(|v| render_ranked_value_term(scaling, *v))
                .collect::<MayFail<Vec<_>>>()?;

            render_exact_match(axis_var, &terms)
        }

        Scaling::Flat { values } => {
            let axis_var = axis
                .ctx_var()
                .ok_or("[formula] flat scaling needs a known axis var")?;

            let terms = values.iter().map(|v| render_f32(*v)).collect::<Vec<_>>();
            render_exact_match(axis_var, &terms)
        }

        Scaling::Nested { raw, .. } => {
            return Err(format!("[formula] Nested scaling not implemented yet: {raw}").into());
        }

        Scaling::Other { raw } => {
            return Err(format!("[formula] Other scaling not implemented yet: {raw}").into());
        }
    };

    *expr = add_term(expr, &term);
    Ok(())
}

fn render_scaling_term(_axis: Axis, scaling: &Scaling) -> MayFail<String> {
    match scaling {
        Scaling::Simple { value, ctx_var } => Ok(render_mul(*value, *ctx_var)),
        Scaling::Per100 { value, ctx_var } => Ok(render_mul(*value, *ctx_var)),
        Scaling::PercentAttr { value, ctx_var, .. } => Ok(render_mul(*value, *ctx_var)),
        Scaling::BasedOnLevel {
            level_var,
            arms,
            ctx_var,
            ..
        } => {
            let match_expr = render_level_arms(*level_var, arms);
            if *ctx_var == CtxVar::SteelcapsEffect {
                Ok(match_expr)
            } else {
                Ok(format!("({match_expr}) * {ctx_var}"))
            }
        }
        _ => Err("[formula] render_scaling_term called with non-scalar scaling".into()),
    }
}

fn render_ranked_value_term(scaling: &Scaling, value: f32) -> MayFail<String> {
    match scaling {
        Scaling::Ranked { ctx_var, .. } => Ok(render_mul(value, *ctx_var)),
        Scaling::RankedPer100 { ctx_var, .. } => Ok(render_mul(value, *ctx_var)),
        _ => Err("[formula] render_ranked_value_term called with invalid scaling".into()),
    }
}

fn render_exact_match(axis_var: CtxVar, terms: &[String]) -> String {
    let mut arms = Vec::new();

    for (i, term) in terms.iter().enumerate() {
        let rank = i + 1;
        arms.push(format!("{rank} => {term}"));
    }

    arms.push("_ => 0.0".to_string());

    format!("match {axis_var} as u8 {{ {} }}", arms.join(", "))
}

fn render_level_arms(level_var: CtxVar, arms: &[LevelArm]) -> String {
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

    format!("match {level_var} as u8 {{ {} }}", rendered.join(", "))
}

fn substitute_x(formula: &str, replacement: &str) -> String {
    formula.replace('x', replacement)
}

fn add_term(base: &str, term: &str) -> String {
    let base = base.trim();
    let term = term.trim();

    if base.is_empty() || base == "0" || base == "0.0" {
        term.to_string()
    } else {
        format!("({base}) + ({term})")
    }
}

fn scaling_len(scaling: &Scaling) -> Option<usize> {
    match scaling {
        Scaling::Ranked { values, .. }
        | Scaling::RankedPer100 { values, .. }
        | Scaling::Flat { values } => Some(values.len()),
        Scaling::BasedOnLevel { arms, .. } => Some(arms.len()),
        _ => None,
    }
}

fn ensure_len(values: &mut Vec<String>, len: usize) {
    if values.is_empty() && len > 0 {
        *values = vec!["0.0".to_string(); len];
    }
}

fn render_mul(value: f32, ctx_var: CtxVar) -> String {
    format!("{} * {ctx_var}", render_f32(value))
}

fn render_f32(value: f32) -> String {
    let s = value.to_string();
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        format!("{s}.0")
    }
}

fn normalize_num_str(s: &str) -> String {
    match s.trim().parse::<f32>() {
        Ok(v) => render_f32(v),
        Err(_) => s.trim().to_string(),
    }
}
