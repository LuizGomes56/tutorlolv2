use crate::{
    client::MayFail,
    formula::SequencePattern,
    parser::{Effect, LevelArm, Scaling},
};
use regex::Regex;
use std::sync::LazyLock;
use tutorlolv2_types::{CtxVar, Key};

#[derive(Clone, Debug)]
pub enum TargetMode {
    None,
    Direct(CtxVar),
    Per100(CtxVar),
}

#[derive(Clone, Debug)]
pub struct Piece {
    coeff: String,
    target: TargetMode,
}

#[derive(Clone, Debug)]
pub enum RenderAction {
    Add(String),
    Mul(String),
}

impl Piece {
    fn to_expr(self) -> String {
        let Self { coeff, target } = self;

        match target {
            TargetMode::None => coeff,
            TargetMode::Direct(ctx_var) => format!("({coeff}) * {ctx_var}"),
            TargetMode::Per100(ctx_var) => format!("({coeff}) * ({ctx_var} * 0.01)"),
        }
    }
}

impl Scaling {
    fn render(&self, axis: CtxVar) -> MayFail<String> {
        match self {
            Self::Simple { value, ctx_var } => Ok(Piece {
                coeff: render_num(*value),
                target: TargetMode::Direct(*ctx_var),
            }
            .to_expr()),

            Self::PercentAttr { value, ctx_var, .. } => Ok(Piece {
                coeff: render_num(*value),
                target: TargetMode::Direct(*ctx_var),
            }
            .to_expr()),

            Self::Per100 { value, ctx_var } => Ok(Piece {
                coeff: render_num(*value),
                target: TargetMode::Per100(*ctx_var),
            }
            .to_expr()),

            Self::Flat { values } => {
                if values.len() == 1 {
                    return Ok(render_num(values[0]));
                }

                Ok(simplify_series(values, axis)?
                    .ok_or("[formula] Flat scaling could not be rendered")?)
            }

            Self::Ranked { values, ctx_var } => Ok(Piece {
                coeff: simplify_series(values, axis)?
                    .ok_or("[formula] Ranked scaling could not be rendered")?,
                target: TargetMode::Direct(*ctx_var),
            }
            .to_expr()),

            Self::RankedPer100 { values, ctx_var } => Ok((Piece {
                coeff: simplify_series(values, axis)?
                    .ok_or("[formula] RankedPer100 scaling could not be rendered")?,
                target: TargetMode::Per100(*ctx_var),
            })
            .to_expr()),

            Self::BasedOnLevel {
                level_var,
                arms,
                ctx_var,
                debug,
            } => {
                let coeff = render_level_coeff(*level_var, arms);

                // Most BasedOnLevel coefficients below or equal to ~2 are ratios
                // that should multiply their ctx_var. Larger values are usually
                // absolute values/caps split out by the parser (for example monster caps),
                // and should be rendered as their own formula.
                let target = match based_on_level_is_ratio(arms, debug) {
                    true => TargetMode::Direct(*ctx_var),
                    false => TargetMode::None,
                };

                Ok(Piece { coeff, target }.to_expr())
            }

            Self::RangePercentAttr {
                min,
                max,
                debug,
                ctx_var,
            } => Ok(render_range_percent_attr(axis, *min, *max, debug, *ctx_var)),

            Self::Multiplier { raw, base, inner } => {
                Ok(render_multiplier(axis, raw, *base, inner)?)
            }

            Self::Nested { outer, inner, .. } => render_nested(axis, outer, inner),

            Self::Other { raw } => {
                Err(format!("[formula] Unsupported Scaling::Other: {raw}").into())
            }
        }
    }

    fn render_scaling_action(&self, axis: CtxVar) -> MayFail<RenderAction> {
        match self {
            Scaling::Multiplier { raw, base, inner } => Ok(RenderAction::Mul(render_multiplier(
                axis, raw, *base, inner,
            )?)),
            _ => Ok(RenderAction::Add(self.render(axis)?)),
        }
    }

    fn render_piece(&self, axis: CtxVar) -> MayFail<Piece> {
        match self {
            Self::Simple { value, ctx_var } => Ok(Piece {
                coeff: render_num(*value),
                target: TargetMode::Direct(*ctx_var),
            }),

            Self::PercentAttr { value, ctx_var, .. } => Ok(Piece {
                coeff: render_num(*value),
                target: TargetMode::Direct(*ctx_var),
            }),

            Self::Per100 { value, ctx_var } => Ok(Piece {
                coeff: render_num(*value),
                target: TargetMode::Per100(*ctx_var),
            }),

            Self::Flat { values } => Ok(Piece {
                coeff: if values.len() == 1 {
                    render_num(values[0])
                } else {
                    simplify_series(values, axis)?
                        .ok_or("[formula] Flat scaling could not be rendered")?
                },
                target: TargetMode::None,
            }),

            Self::Ranked { values, ctx_var } => Ok(Piece {
                coeff: simplify_series(values, axis)?
                    .ok_or("[formula] Ranked scaling could not be rendered")?,
                target: TargetMode::Direct(*ctx_var),
            }),

            Self::RankedPer100 { values, ctx_var } => Ok(Piece {
                coeff: simplify_series(values, axis)?
                    .ok_or("[formula] RankedPer100 scaling could not be rendered")?,
                target: TargetMode::Per100(*ctx_var),
            }),

            Self::BasedOnLevel {
                level_var,
                arms,
                ctx_var,
                debug,
            } => Ok(Piece {
                coeff: render_level_coeff(*level_var, arms),
                target: if based_on_level_is_ratio(arms, debug) {
                    TargetMode::Direct(*ctx_var)
                } else {
                    TargetMode::None
                },
            }),

            Self::RangePercentAttr {
                min,
                max,
                debug,
                ctx_var,
            } => Ok(Piece {
                coeff: render_range_percent_attr(axis, *min, *max, debug, *ctx_var),
                target: TargetMode::None,
            }),

            Self::Multiplier { raw, base, inner } => Ok(Piece {
                coeff: render_multiplier(axis, raw, *base, inner)?,
                target: TargetMode::None,
            }),

            Self::Nested { outer, inner, .. } => Ok(Piece {
                coeff: render_nested(axis, outer, inner)?,
                target: TargetMode::None,
            }),

            Self::Other { raw } => {
                Err(format!("[formula] Unsupported Scaling::Other: {raw}").into())
            }
        }
    }

    fn try_get_len(scaling: &Scaling) -> Option<usize> {
        match scaling {
            Scaling::Ranked { values, .. }
            | Scaling::RankedPer100 { values, .. }
            | Scaling::Flat { values } => Some(values.len()),
            Scaling::BasedOnLevel { arms, .. } => level_arm_values(arms).map(|v| v.len()),
            _ => None,
        }
    }
}

impl Effect {
    pub fn load_formula(&mut self, champion_id: &str, key: Key) -> MayFail {
        self.formula = match champion_id {
            // "Nidalee" if self.index > 0 && matches!(key, Key::Q) => {
            //     self.simplify_formula_with_axis(key, Some(CtxVar::QLevel))
            // }
            _ => self.simplify_formula(key),
        }?;

        Ok(())
    }

    fn infer_axis(&self, key: Key) -> Option<CtxVar> {
        let len = self
            .base
            .as_ref()
            .map(Vec::len)
            .or_else(|| self.use_values.as_ref().map(Vec::len))
            .or_else(|| self.scalings.iter().find_map(Scaling::try_get_len))
            .unwrap_or(0);

        if matches!(len, 18 | 20) {
            return Some(CtxVar::Level);
        }

        match key {
            Key::P => Some(CtxVar::Level),
            Key::Q => Some(CtxVar::QLevel),
            Key::W => Some(CtxVar::WLevel),
            Key::E => Some(CtxVar::ELevel),
            Key::R => Some(CtxVar::RLevel),
        }
    }

    fn use_values_belongs_to_level_scaling(&self, key: Key) -> bool {
        let leveling = self.inner.leveling.to_ascii_lowercase();

        let has_based_on_level_text = leveling.contains("based on level");
        let has_rank_progression = leveling.contains('/');

        let has_based_on_level_scaling = self
            .scalings
            .iter()
            .any(|s| matches!(s, Scaling::BasedOnLevel { .. }));

        has_based_on_level_text
            && has_rank_progression
            && has_based_on_level_scaling
            && matches!(key, Key::Q | Key::W | Key::E | Key::R)
    }

    pub fn simplify_formula(&self, key: Key) -> MayFail<Option<String>> {
        self.simplify_formula_with_axis(key, None)
    }

    /// Use this for champions/effects whose progression axis is known to be special
    /// (for example Nidalee cougar abilities that scale with R rank).
    pub fn simplify_formula_with_axis(
        &self,
        key: Key,
        axis_override: Option<CtxVar>,
    ) -> MayFail<Option<String>> {
        let axis = match axis_override.or_else(|| self.infer_axis(key)) {
            Some(v) => v,
            None => return Ok(None),
        };

        let mut expr = match build_base_expr(key, axis, self)? {
            Some(v) => v,
            None => "0.0".to_string(),
        };

        for scaling in &self.scalings {
            match scaling.render_scaling_action(axis)? {
                RenderAction::Add(term) => {
                    expr = add_expr(&expr, &term);
                }
                RenderAction::Mul(multiplier) => {
                    expr = mul_expr(&expr, &multiplier);
                }
            }
        }

        Ok(Some(expr))
    }
}

fn build_base_expr(key: Key, axis: CtxVar, effect: &Effect) -> MayFail<Option<String>> {
    if let Some(use_formula) = &effect.use_formula {
        return Ok(Some(use_formula.replace('x', &format!("({axis} as f32)"))));
    }

    if let Some(base) = &effect.base {
        return simplify_series(base, axis);
    }

    if let Some(use_values) = &effect.use_values {
        if effect.use_values_belongs_to_level_scaling(key) {
            return Ok(None);
        }

        return simplify_series(use_values, axis);
    }

    Ok(None)
}

fn render_nested(axis: CtxVar, outer: &Scaling, inner: &[Scaling]) -> MayFail<String> {
    let Piece { mut coeff, target } = outer.render_piece(axis)?;

    for child in inner {
        let child_piece = child.render_piece(axis)?;
        coeff = add_expr(&coeff, &child_piece.to_expr());
    }

    Ok(Piece { coeff, target }.to_expr())
}

fn render_multiplier(axis: CtxVar, raw: &str, base: f64, inner: &[Scaling]) -> MayFail<String> {
    // Parser currently stores "0.3 per 100% bonus attack speed" as Per100(0.003).
    // For multipliers this number is not a percent coefficient; it is a direct
    // multiplier delta per 100%. Prefer the raw text when it matches this form.
    if let Some(expr) = render_multiplier_from_raw(raw, inner) {
        return Ok(expr);
    }

    let mut expr = render_num(base);

    for scaling in inner {
        let child = scaling.render_piece(axis)?;
        expr = add_expr(&expr, &child.to_expr());
    }

    Ok(expr)
}

fn render_multiplier_from_raw(raw: &str, inner: &[Scaling]) -> Option<String> {
    static MULT_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)^\s*1\s*\+\s*(-?\d+(?:\.\d+)?)\s+per\s+100%?\s+").unwrap()
    });

    let caps = MULT_RE.captures(raw)?;
    let value = caps.get(1)?.as_str().parse::<f64>().ok()?;
    let ctx_var = first_ctx_var(inner)?;

    Some(add_expr(
        "1.0",
        &Piece {
            coeff: render_num(value),
            target: TargetMode::Per100(ctx_var),
        }
        .to_expr(),
    ))
}

fn first_ctx_var(list: &[Scaling]) -> Option<CtxVar> {
    for scaling in list {
        match scaling {
            Scaling::Simple { ctx_var, .. }
            | Scaling::Ranked { ctx_var, .. }
            | Scaling::RankedPer100 { ctx_var, .. }
            | Scaling::Per100 { ctx_var, .. }
            | Scaling::PercentAttr { ctx_var, .. }
            | Scaling::BasedOnLevel { ctx_var, .. }
            | Scaling::RangePercentAttr { ctx_var, .. } => return Some(*ctx_var),
            Scaling::Nested { outer, inner, .. } => {
                if let Some(v) = first_ctx_var(std::slice::from_ref(outer)) {
                    return Some(v);
                }
                if let Some(v) = first_ctx_var(inner) {
                    return Some(v);
                }
            }
            Scaling::Multiplier { inner, .. } => {
                if let Some(v) = first_ctx_var(inner) {
                    return Some(v);
                }
            }
            Scaling::Flat { .. } | Scaling::Other { .. } => {}
        }
    }

    None
}

fn render_range_percent_attr(
    axis: CtxVar,
    min: f64,
    max: f64,
    debug: &str,
    ctx_var: CtxVar,
) -> String {
    let lower = debug.to_ascii_lowercase();

    if lower.contains("level") {
        let level_axis = if lower.contains("rank") {
            axis
        } else {
            CtxVar::Level
        };

        return render_range_linear(min, max, level_axis, range_denominator_for_axis(level_axis));
    }

    let driver = match ctx_var {
        // These are naturally 0.0..1.0 drivers in the formula system.
        CtxVar::CritChance => format!("{ctx_var}"),
        // These are best-effort: the parser marks some stack/chime/fury-like values
        // as SteelcapsEffect when no dedicated CtxVar exists.
        _ => format!("{ctx_var}"),
    };

    add_expr(
        &render_num(min),
        &format!("{} * ({driver})", render_num(max - min)),
    )
}

fn render_range_linear(min: f64, max: f64, axis: CtxVar, denominator: f64) -> String {
    let span = max - min;

    if approx_zero(span) {
        return render_num(min);
    }

    add_expr(
        &render_num(min),
        &format!(
            "{} * {} / {}",
            render_num(span),
            axis_offset(axis),
            render_num(denominator)
        ),
    )
}

fn range_denominator_for_axis(axis: CtxVar) -> f64 {
    match axis {
        CtxVar::QLevel | CtxVar::WLevel | CtxVar::ELevel | CtxVar::RLevel => 4.0,
        _ => 17.0,
    }
}

fn simplify_series(values: &[f64], axis: CtxVar) -> MayFail<Option<String>> {
    if values.is_empty() {
        return Ok(None);
    }

    if values.len() == 1 {
        return Ok(Some(render_num(values[0])));
    }

    let values = maybe_extend_series(values, axis);
    let pat = SequencePattern::new(&values);

    let expr = match pat {
        SequencePattern::Linear => render_linear(&values, axis),
        SequencePattern::Quadratic => render_quadratic(&values, axis),
        SequencePattern::Cubic => render_cubic(&values, axis),
        SequencePattern::Unknown | SequencePattern::Inconclusive => {
            render_exact_match(&values, axis)
        }
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
        Some(render_num(v0)),
        match approx_zero(d1) {
            true => None,
            false => Some(format!("{} * {n}", render_num(d1))),
        },
    ])
}

fn render_quadratic(values: &[f64], axis: CtxVar) -> String {
    let &[v0, v1, v2, ..] = values else {
        panic!("render_quadratic requires 3 values: {values:?}");
    };

    let d1 = v1 - v0;
    let d2 = v2 - 2.0 * v1 + v0;
    let n = axis_offset(axis);

    combine_terms([
        Some(render_num(v0)),
        match approx_zero(d1) {
            true => None,
            false => Some(format!("{} * {n}", render_num(d1))),
        },
        match approx_zero(d2) {
            true => None,
            false => Some(format!("{} * {n} * ({n} - 1.0) * 0.5", render_num(d2))),
        },
    ])
}

fn render_cubic(values: &[f64], axis: CtxVar) -> String {
    let &[v0, v1, v2, v3, ..] = values else {
        panic!("render_cubic requires 4 values: {values:?}");
    };

    let d1 = v1 - v0;
    let d2 = v2 - 2.0 * v1 + v0;
    let d3 = v3 - 3.0 * v2 + 3.0 * v1 - v0;
    let n = axis_offset(axis);

    combine_terms([
        Some(render_num(v0)),
        match approx_zero(d1) {
            true => None,
            false => Some(format!("{} * {n}", render_num(d1))),
        },
        match approx_zero(d2) {
            true => None,
            false => Some(format!("{} * {n} * ({n} - 1.0) * 0.5", render_num(d2))),
        },
        match approx_zero(d3) {
            true => None,
            false => Some(format!(
                "{} * {n} * ({n} - 1.0) * ({n} - 2.0) / 6.0",
                render_num(d3)
            )),
        },
    ])
}

fn render_level_coeff(level_var: CtxVar, arms: &[LevelArm]) -> String {
    if let Some(values) = level_arm_values(arms) {
        if values.len() > 1 {
            let pat = SequencePattern::new(&values);

            if matches!(
                pat,
                SequencePattern::Linear | SequencePattern::Quadratic | SequencePattern::Cubic
            ) {
                return simplify_series(&values, level_var)
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| render_level_match(level_var, arms));
            }
        }
    }

    render_level_match(level_var, arms)
}

fn based_on_level_is_ratio(arms: &[LevelArm], debug: &str) -> bool {
    if debug.to_ascii_lowercase().contains("capped at") {
        return false;
    }

    let max = arms
        .iter()
        .map(LevelArm::value)
        .fold(f64::NEG_INFINITY, f64::max);

    max <= 2.0
}

fn render_level_match(level_var: CtxVar, arms: &[LevelArm]) -> String {
    let mut rendered = Vec::new();

    for arm in arms {
        match arm {
            LevelArm::To { range, value, .. } => {
                rendered.push(format!("{range:?} => {}", render_num(*value)))
            }
            LevelArm::Range { range, value, .. } => {
                let arm = match range.len() == 1 {
                    true => range.start.to_string(),
                    false => format!("{range:?}"),
                };
                rendered.push(format!("{arm} => {}", render_num(*value)))
            }
            LevelArm::From { range, value, .. } => {
                rendered.push(format!("{range:?} => {}", render_num(*value)))
            }
        }
    }

    if !rendered.iter().any(|v| v.trim_start().starts_with("_")) {
        let fallback = arms.last().map(LevelArm::value).unwrap_or(0.0);
        rendered.push(format!("_ => {}", render_num(fallback)));
    }

    format!("match {level_var} as u8 {{ {} }}", rendered.join(", "))
}

fn level_arm_values(arms: &[LevelArm]) -> Option<Vec<f64>> {
    if arms.is_empty() {
        return None;
    }

    let mut values = Vec::new();

    for arm in arms {
        match arm {
            LevelArm::To { range, value, .. } => {
                for _ in 1..range.end {
                    values.push(*value);
                }
            }
            LevelArm::Range { range, value, .. } => {
                for _ in range.start..range.end {
                    values.push(*value);
                }
            }
            LevelArm::From { range, value, .. } => {
                for _ in range.start..=20 {
                    values.push(*value);
                }
            }
        }
    }

    match values.is_empty() {
        true => None,
        false => Some(values),
    }
}

fn add_expr(lhs: &str, rhs: &str) -> String {
    let lhs = lhs.trim();
    let rhs = rhs.trim();

    if lhs.is_empty() || lhs == "0.0" || lhs == "0" {
        rhs.to_string()
    } else if rhs.is_empty() || rhs == "0.0" || rhs == "0" {
        lhs.to_string()
    } else {
        format!("({lhs}) + ({rhs})")
    }
}

fn mul_expr(lhs: &str, rhs: &str) -> String {
    let lhs = lhs.trim();
    let rhs = rhs.trim();

    if lhs.is_empty() || lhs == "0.0" || lhs == "0" {
        rhs.to_string()
    } else if rhs.is_empty() || rhs == "1.0" || rhs == "1" {
        lhs.to_string()
    } else {
        format!("({lhs}) * ({rhs})")
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
    v.abs() < 1e-9
}

fn render_exact_match(values: &[f64], axis: CtxVar) -> String {
    let mut arms = Vec::new();

    for (i, value) in values.iter().enumerate() {
        let rank = i + 1;
        arms.push(format!("{rank} => {}", render_num(*value)));
    }

    let fallback = values.last().copied().unwrap_or(0.0);

    format!(
        "match {axis} as u8 {{ {}, _ => {} }}",
        arms.join(", "),
        render_num(fallback),
    )
}

fn maybe_extend_series(values: &[f64], axis: CtxVar) -> Vec<f64> {
    if matches!(
        axis,
        CtxVar::QLevel | CtxVar::WLevel | CtxVar::ELevel | CtxVar::RLevel
    ) && values.len() == 4
    {
        let pat = SequencePattern::new(values);

        if matches!(
            pat,
            SequencePattern::Linear | SequencePattern::Quadratic | SequencePattern::Cubic
        ) {
            let mut out = values.to_vec();
            out.push(pat.predict_next(values));
            return out;
        }
    }

    values.to_vec()
}

fn render_num(v: f64) -> String {
    if approx_zero(v) {
        return "0.0".to_string();
    }

    let marker = 1_000_000_000.0;

    let rounded = if (v - v.round()).abs() < 1e-9 {
        v.round()
    } else {
        (v * marker).round() / marker
    };

    let mut s = format!("{rounded:.9}");
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }

    if s.ends_with('.') {
        s.push('0');
    }

    if !s.contains('.') {
        s.push_str(".0");
    }

    s
}
