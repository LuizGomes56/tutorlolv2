use crate::{
    MayFail,
    items::{ItemEffectRaw, ItemRaw, cache},
    parser::{Effect, EffectInner, Scaling, assign_ctx_var, vec_dedup},
};
use heck::ToPascalCase;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::LazyLock};
use tutorlolv2_types::{CtxVar, Key};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemEffect {
    pub name: Option<String>,
    pub unique: Option<bool>,
    pub raw_description: Option<String>,
    pub ast: Vec<MarkupNode>,
    /// Effect for melee champions, or for both when their values are the same
    pub effect: Effect,
    /// Effect for ranged champions when it differs, from `{{rd|melee|ranged}}`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranged: Option<Effect>,
    /// Effect for the alternate text of `{{ft|text|alternate}}` when it differs,
    /// such as the total damage of a burn that the text describes per tick
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flip: Option<Effect>,
    /// Effect described after "for a total of", which `effect` leaves out
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<Effect>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemEffects {
    pub pass: Option<ItemEffect>,
    /// Items may have up to 4 passives, and the damage is often in a later one,
    /// such as Statikk Shiv's Electroshock
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass2: Option<ItemEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass3: Option<ItemEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass4: Option<ItemEffect>,
    pub act: Option<ItemEffect>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WikiItem {
    pub id: u32,
    pub name: String,
    pub item_id: String,
    pub tier: Option<u8>,

    #[serde(default)]
    pub modes: BTreeMap<String, bool>,

    #[serde(default)]
    pub stats: BTreeMap<String, f64>,

    #[serde(default)]
    pub effects: ItemEffects,

    #[serde(default)]
    pub recipe: Vec<String>,

    pub buy: Option<u16>,
    pub purchasable: bool,
    pub custom: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MarkupNode {
    Text {
        text: String,
    },
    Link {
        target: String,
        label: Option<String>,
    },
    Template(TemplateNode),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "template", rename_all = "snake_case")]
pub enum TemplateNode {
    As {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Rd {
        melee: Vec<MarkupNode>,
        ranged: Vec<MarkupNode>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Pp {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Ap {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Fd {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Ft {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Tip {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Tt {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Sti {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Ii {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Ais {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Ui {
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
    Generic {
        name: String,
        positional: Vec<Vec<MarkupNode>>,
        named: BTreeMap<String, Vec<MarkupNode>>,
    },
}

impl Default for MarkupNode {
    fn default() -> Self {
        Self::Text {
            text: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct EffectAnalysis {
    scalings: Vec<Scaling>,
    base: Option<Vec<f64>>,
}

impl EffectAnalysis {
    fn merge(&mut self, mut other: EffectAnalysis) {
        self.scalings.append(&mut other.scalings);
        if self.base.is_none() {
            self.base = other.base.take();
        }
    }
}

pub fn parse_items() -> MayFail {
    let bytes = crate::read(cache().join("data").with_extension("json"))?;
    let data = serde_json::from_slice::<BTreeMap<String, ItemRaw>>(&bytes)?;

    let result = data
        .into_iter()
        .map(|(name, raw)| {
            let mut value = parse_item(raw);

            let assign_formula = |v: Option<&mut ItemEffect>| {
                if let Some(ie) = v
                    && let Ok(formula) = ie.effect.simplify_formula(Key::P)
                {
                    ie.effect.formula = formula;
                }
            };

            let assign_other_formulas = |v: Option<&mut ItemEffect>| {
                if let Some(ie) = v {
                    for effect in [&mut ie.ranged, &mut ie.flip, &mut ie.total]
                        .into_iter()
                        .flatten()
                    {
                        if let Ok(formula) = effect.simplify_formula(Key::P) {
                            effect.formula = formula;
                        }
                    }
                }
            };

            assign_other_formulas(value.effects.act.as_mut());
            assign_other_formulas(value.effects.pass.as_mut());
            assign_other_formulas(value.effects.pass2.as_mut());
            assign_other_formulas(value.effects.pass3.as_mut());
            assign_other_formulas(value.effects.pass4.as_mut());

            assign_formula(value.effects.act.as_mut());
            assign_formula(value.effects.pass.as_mut());
            assign_formula(value.effects.pass2.as_mut());
            assign_formula(value.effects.pass3.as_mut());
            assign_formula(value.effects.pass4.as_mut());

            let key = name.replace("'s", "s").to_pascal_case();
            value.name = name;
            value.item_id = key.clone();

            (key, value)
        })
        .collect::<BTreeMap<_, _>>();

    let json = serde_json::to_string_pretty(&result)?;
    crate::write(cache().join("full").with_extension("json"), &json)
}

fn parse_item(raw: ItemRaw) -> WikiItem {
    // Variables defined in one effect may be used by another, like Hollow Radiance's
    // eruption that deals 200% of the damage defined by its burn
    let mut vars = BTreeMap::new();

    for effect in [
        &raw.effects.pass,
        &raw.effects.pass2,
        &raw.effects.pass3,
        &raw.effects.pass4,
        &raw.effects.act,
    ]
    .into_iter()
    .flatten()
    {
        if let Some(description) = &effect.description {
            resolve_markup(description, Variant::default(), &mut vars, &mut Vec::new());
        }
    }

    let parse = |raw| parse_item_effect(raw, &vars);

    WikiItem {
        item_id: raw.name.replace("'s", "s").to_pascal_case(),
        name: raw.name,
        id: raw.id,
        tier: raw.tier,
        modes: raw.modes,
        stats: raw.stats,
        effects: ItemEffects {
            pass: raw.effects.pass.map(parse),
            pass2: raw.effects.pass2.map(parse),
            pass3: raw.effects.pass3.map(parse),
            pass4: raw.effects.pass4.map(parse),
            act: raw.effects.act.map(parse),
        },
        recipe: raw.recipe,
        buy: raw.buy,
        purchasable: false,
        custom: false,
    }
}

fn parse_item_effect(raw: ItemEffectRaw, vars: &BTreeMap<String, String>) -> ItemEffect {
    let description = raw.description.clone().unwrap_or_default();
    let ast = parse_markup(&description);

    let (effect, total) = analyze_effect(&description, Variant::default(), vars);

    let variant = |variant| {
        let (other, _) = analyze_effect(&description, variant, vars);
        (other != effect).then_some(other)
    };

    let ranged = variant(Variant {
        ranged: true,
        flip: false,
    });

    let flip = variant(Variant {
        ranged: false,
        flip: true,
    });

    ItemEffect {
        name: raw.name,
        unique: raw.unique,
        raw_description: raw.description,
        effect,
        ranged,
        flip,
        total,
        ast,
    }
}

/// Which text of the description is parsed
#[derive(Clone, Copy, Debug, Default)]
struct Variant {
    /// Values for ranged champions, from `{{rd|melee|ranged}}`
    ranged: bool,
    /// Alternate text of `{{ft|text|alternate}}`
    flip: bool,
}

/// Parses the description after resolving the templates that compute values.
/// The text after "for a total of" is parsed separately, as the second effect
fn analyze_effect(
    description: &str,
    variant: Variant,
    vars: &BTreeMap<String, String>,
) -> (Effect, Option<Effect>) {
    let mut series = Vec::new();
    let resolved = resolve_markup(description, variant, &mut vars.clone(), &mut series);

    let (main, total) = split_total(&resolved);
    let effect = build_effect(description, main, &series);
    let total = total
        .map(|total| build_effect(description, total, &series))
        .filter(Effect::should_keep);

    (effect, total)
}

/// Splits "deals 10 every second, for a total of 60." into the text before
/// "for a total of" and the text of the total, up to the end of its clause
fn split_total(resolved: &str) -> (&str, Option<&str>) {
    const MARKER: &str = "for a total of";

    let Some(start) = resolved.to_ascii_lowercase().find(MARKER) else {
        return (resolved, None);
    };

    let rest = &resolved[start + MARKER.len()..];
    let bytes = rest.as_bytes();
    let mut depth = 0i32;
    let mut end = rest.len();

    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b'{' | b'(' | b'[' => depth += 1,
            b'}' | b')' | b']' => depth -= 1,
            // A dot followed by a digit is a decimal point
            b'.' if bytes.get(i + 1).is_some_and(u8::is_ascii_digit) => {}
            b',' | b'.' if depth <= 0 => {
                end = i;
                break;
            }
            _ => {}
        }
    }

    (&resolved[..start], Some(&rest[..end]))
}

fn build_effect(description: &str, resolved: &str, series: &[LevelSeries]) -> Effect {
    let nodes = parse_markup(resolved);
    let mut analysis = analyze_sequence(&nodes, false);

    nest_per100_ratios(&flatten_nodes(&nodes), &mut analysis.scalings);
    vec_dedup(&mut analysis.scalings);

    let mut effect = Effect {
        index: 0,
        formula: None,
        inner: EffectInner {
            description: description.to_string(),
            leveling: String::new(),
        },
        scalings: analysis.scalings,
        use_formula: None,
        use_values: None,
        use_levels: None,
        base: analysis.base,
    };

    // A base that comes from a progression by level uses all of its values. The
    // resolved text has the first value rounded, so they are compared with a tolerance
    let level_series = match &effect.base {
        Some(base) => match base.as_slice() {
            [value] => series
                .iter()
                .find(|s| (s.values[0] - value).abs() < 1e-5 * value.abs().max(1.0)),
            _ => None,
        },
        // Damage written outside of `{{as}}`, like "dealing {{pp|50 to 100|color=physical damage}}"
        None => series.iter().find(|s| s.damage),
    };

    if let Some(level_series) = level_series {
        effect.base = None;
        effect.use_values = Some(level_series.values.clone());
        effect.use_levels = Some(level_series.levels.clone());
    }

    effect
}

/// "12% (+ 5% per 100 bonus AD) of target's maximum health" is a single ratio of
/// the target's health that increases with bonus AD, but each part is parsed as
/// its own scaling. This replaces those parts with one nested scaling. The ratio
/// may also increase per point of a stat or per stack, as in "0.7% (+ 0.1% per
/// target's Reap stack) of the target's maximum health"
fn nest_per100_ratios(text: &str, scalings: &mut Vec<Scaling>) {
    // `{{as|3%|hp}}` is flattened with its color as "3% hp", so each value may be
    // followed by a word, which may also be the "of" before the target
    static NESTED_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?i)(\d+(?:\.\d+)?)%(?:\s+[a-z]+)?\s*((?:\([^()]*\)(?:\s+[a-z]+)?\s*)+)((?:of )?(?:the )?target's (?:maximum|current) health)",
        )
        .unwrap()
    });

    static INNER_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\(\+\s*(\d+(?:\.\d+)?)%\s+per\s+(100\s+|1\s+)?([^)]+)\)").unwrap()
    });

    let text = text.replace("'''", "");

    let Some(caps) = NESTED_RE.captures(&text) else {
        return;
    };
    let Ok(value) = caps[1].parse::<f64>() else {
        return;
    };

    let inner = INNER_RE
        .captures_iter(&caps[2])
        .filter_map(|inner| {
            let value = inner[1].parse::<f64>().ok()? / 100.0;
            let ctx_var = assign_ctx_var(&inner[3]);

            Some(match inner.get(2).map(|per| per.as_str().trim()) {
                Some("100") => Scaling::Per100 { value, ctx_var },
                _ => Scaling::Simple { value, ctx_var },
            })
        })
        .collect::<Vec<_>>();

    if inner.is_empty() {
        return;
    }

    let debug = caps[3].to_string();
    let ctx_var = assign_ctx_var(&debug);
    let outer = value / 100.0;

    // The outer ratio is parsed without the target when it is written apart
    let is_outer = |scaling: &Scaling| match scaling {
        Scaling::Simple { value, ctx_var } | Scaling::PercentAttr { value, ctx_var, .. } => {
            (*value - outer).abs() < 1e-9
                && matches!(
                    ctx_var,
                    CtxVar::Unknown | CtxVar::EnemyMaxHealth | CtxVar::EnemyCurrentHealth
                )
        }
        _ => false,
    };

    // Values are compared with a tolerance, since they may be divided differently
    let is_inner = |scaling: &Scaling| {
        inner.iter().any(|part| match (part, scaling) {
            (
                Scaling::Simple {
                    value: a,
                    ctx_var: x,
                },
                Scaling::Simple {
                    value: b,
                    ctx_var: y,
                },
            )
            | (
                Scaling::Per100 {
                    value: a,
                    ctx_var: x,
                },
                Scaling::Per100 {
                    value: b,
                    ctx_var: y,
                },
            ) => x == y && (a - b).abs() < 1e-9,
            _ => false,
        })
    };

    let is_part = |scaling: &Scaling| is_outer(scaling) || is_inner(scaling);

    let position = scalings.iter().position(is_part).unwrap_or(scalings.len());

    scalings.retain(|scaling| !is_part(scaling));
    scalings.insert(
        position.min(scalings.len()),
        Scaling::Nested {
            raw: caps[0].to_string(),
            outer: Box::new(Scaling::PercentAttr {
                value: outer,
                debug,
                ctx_var,
            }),
            inner,
        },
    );
}

fn parse_markup(input: &str) -> Vec<MarkupNode> {
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < input.len() {
        let next_template = input[i..].find("{{").map(|v| i + v);
        let next_link = input[i..].find("[[").map(|v| i + v);

        let next = match (next_template, next_link) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        let Some(next_start) = next else {
            push_text(&mut out, &input[i..]);
            break;
        };

        if next_start > i {
            push_text(&mut out, &input[i..next_start]);
        }

        if input[next_start..].starts_with("{{")
            && let Some(end) = find_balanced(input, next_start, "{{", "}}")
        {
            let content = &input[next_start + 2..end - 2];
            out.push(MarkupNode::Template(parse_template(content)));
            i = end;

            continue;
        }

        if input[next_start..].starts_with("[[")
            && let Some(end) = find_balanced(input, next_start, "[[", "]]")
        {
            let content = &input[next_start + 2..end - 2];
            out.push(parse_link(content));
            i = end;

            continue;
        }

        push_text(&mut out, &input[next_start..next_start + 1]);
        i = next_start + 1;
    }

    merge_text_nodes(&mut out);
    out
}

fn parse_template(content: &str) -> TemplateNode {
    let parts = split_top_level(content, '|');
    let name = parts
        .first()
        .map(|v| v.trim().to_ascii_lowercase())
        .unwrap_or_default();

    let mut positional = Vec::new();
    let mut named = BTreeMap::new();

    for part in parts.into_iter().skip(1) {
        if let Some((k, v)) = split_named_arg(&part) {
            named.insert(k.trim().to_string(), parse_markup(v.trim()));
        } else {
            positional.push(parse_markup(part.trim()));
        }
    }

    match name.as_str() {
        "as" => TemplateNode::As { positional, named },
        "rd" => {
            let melee = positional.first().cloned().unwrap_or_default();
            let ranged = positional.get(1).cloned().unwrap_or_default();

            TemplateNode::Rd {
                melee,
                ranged,
                named,
            }
        }
        "pp" => TemplateNode::Pp { positional, named },
        "ap" => TemplateNode::Ap { positional, named },
        "fd" => TemplateNode::Fd { positional, named },
        "ft" => TemplateNode::Ft { positional, named },
        "tip" => TemplateNode::Tip { positional, named },
        "tt" => TemplateNode::Tt { positional, named },
        "sti" => TemplateNode::Sti { positional, named },
        "ii" => TemplateNode::Ii { positional, named },
        "ais" => TemplateNode::Ais { positional, named },
        "ui" => TemplateNode::Ui { positional, named },
        _ => TemplateNode::Generic {
            name,
            positional,
            named,
        },
    }
}

fn parse_link(content: &str) -> MarkupNode {
    let parts = split_top_level(content, '|');
    let target = parts
        .first()
        .map(|v| v.trim())
        .unwrap_or_default()
        .to_string();

    let label = if parts.len() > 1 {
        Some(parts[1..].join("|").trim().to_string())
    } else {
        None
    };

    MarkupNode::Link { target, label }
}

fn find_balanced(input: &str, start: usize, open: &str, close: &str) -> Option<usize> {
    let mut i = start;
    let mut depth = 0usize;

    while i < input.len() {
        if input[i..].starts_with(open) {
            depth += 1;
            i += open.len();
            continue;
        }

        if input[i..].starts_with(close) {
            depth = depth.saturating_sub(1);
            i += close.len();

            if depth == 0 {
                return Some(i);
            }

            continue;
        }

        i += 1;
    }

    None
}

fn split_top_level(input: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;

    let mut tpl_depth = 0usize;
    let mut link_depth = 0usize;

    while i < input.len() {
        if input[i..].starts_with("{{") {
            tpl_depth += 1;
            i += 2;
            continue;
        }
        if input[i..].starts_with("}}") {
            tpl_depth = tpl_depth.saturating_sub(1);
            i += 2;
            continue;
        }
        if input[i..].starts_with("[[") {
            link_depth += 1;
            i += 2;
            continue;
        }
        if input[i..].starts_with("]]") {
            link_depth = link_depth.saturating_sub(1);
            i += 2;
            continue;
        }

        let ch = input[i..].chars().next().unwrap();
        if ch == sep && tpl_depth == 0 && link_depth == 0 {
            out.push(input[start..i].to_string());
            i += ch.len_utf8();
            start = i;
            continue;
        }

        i += ch.len_utf8();
    }

    out.push(input[start..].to_string());
    out
}

fn split_named_arg(input: &str) -> Option<(&str, &str)> {
    let mut i = 0usize;
    let mut tpl_depth = 0usize;
    let mut link_depth = 0usize;

    while i < input.len() {
        if input[i..].starts_with("{{") {
            tpl_depth += 1;
            i += 2;
            continue;
        }
        if input[i..].starts_with("}}") {
            tpl_depth = tpl_depth.saturating_sub(1);
            i += 2;
            continue;
        }
        if input[i..].starts_with("[[") {
            link_depth += 1;
            i += 2;
            continue;
        }
        if input[i..].starts_with("]]") {
            link_depth = link_depth.saturating_sub(1);
            i += 2;
            continue;
        }

        let ch = input[i..].chars().next().unwrap();
        if ch == '=' && tpl_depth == 0 && link_depth == 0 {
            return Some((&input[..i], &input[i + 1..]));
        }

        i += ch.len_utf8();
    }

    None
}

fn push_text(out: &mut Vec<MarkupNode>, text: &str) {
    if !text.is_empty() {
        out.push(MarkupNode::Text {
            text: text.to_string(),
        });
    }
}

fn merge_text_nodes(nodes: &mut Vec<MarkupNode>) {
    let mut out = Vec::new();

    for node in std::mem::take(nodes) {
        match node {
            MarkupNode::Text { text } => {
                if let Some(MarkupNode::Text { text: prev }) = out.last_mut() {
                    prev.push_str(&text);
                } else {
                    out.push(MarkupNode::Text { text });
                }
            }
            other => out.push(other),
        }
    }

    *nodes = out;
}

fn flatten_nodes(nodes: &[MarkupNode]) -> String {
    let mut out = String::new();

    for node in nodes {
        match node {
            MarkupNode::Text { text } => out.push_str(text),
            MarkupNode::Link { label, target } => {
                out.push_str(label.as_deref().unwrap_or(target));
            }
            MarkupNode::Template(template) => out.push_str(&flatten_template(template)),
        }
    }

    normalize_whitespace(&out)
}

fn flatten_template(template: &TemplateNode) -> String {
    match template {
        TemplateNode::As { positional, .. }
        | TemplateNode::Pp { positional, .. }
        | TemplateNode::Ap { positional, .. }
        | TemplateNode::Fd { positional, .. }
        | TemplateNode::Ft { positional, .. }
        | TemplateNode::Tip { positional, .. }
        | TemplateNode::Tt { positional, .. }
        | TemplateNode::Sti { positional, .. }
        | TemplateNode::Ii { positional, .. }
        | TemplateNode::Ais { positional, .. }
        | TemplateNode::Ui { positional, .. } => positional
            .iter()
            .map(|v| flatten_nodes(v))
            .collect::<Vec<_>>()
            .join(" "),
        TemplateNode::Rd { melee, ranged, .. } => {
            format!("{} | {}", flatten_nodes(melee), flatten_nodes(ranged))
        }
        TemplateNode::Generic {
            name, positional, ..
        } => {
            let inner = positional
                .iter()
                .map(|v| flatten_nodes(v))
                .collect::<Vec<_>>()
                .join(" | ");
            format!("{name}({inner})")
        }
    }
}

fn analyze_sequence(nodes: &[MarkupNode], allow_base: bool) -> EffectAnalysis {
    let mut out = EffectAnalysis::default();

    if allow_base {
        out.base = parse_base_from_sequence(nodes);
    }

    // Parse direct "(+ ...)" style scalings from the visible text of this sequence.
    let visible = flatten_nodes(nodes);
    out.scalings
        .extend(parse_scalings_from_plain_text(&visible));

    for (idx, node) in nodes.iter().enumerate() {
        match node {
            MarkupNode::Text { .. } | MarkupNode::Link { .. } => {}
            MarkupNode::Template(template) => match template {
                TemplateNode::As { positional, named } => {
                    out.merge(analyze_as(positional, named));
                }
                TemplateNode::Rd {
                    melee,
                    ranged,
                    named,
                } => {
                    out.scalings
                        .extend(contextualize_range_diff(nodes, idx, melee, ranged));

                    for value in named.values() {
                        out.merge(analyze_sequence(value, false));
                    }
                }
                TemplateNode::Pp { positional, named } => {
                    out.scalings.extend(contextualize_progression(
                        "pp", nodes, idx, positional, named,
                    ));

                    // recurse only into named/positional children as plain sequences,
                    // but do not flatten the pp node itself into fake Flat/Ranked.
                    for value in positional {
                        out.merge(analyze_sequence(value, false));
                    }
                    for value in named.values() {
                        out.merge(analyze_sequence(value, false));
                    }
                }
                TemplateNode::Ap { positional, named } => {
                    out.scalings.extend(contextualize_progression(
                        "ap", nodes, idx, positional, named,
                    ));

                    for value in positional {
                        out.merge(analyze_sequence(value, false));
                    }
                    for value in named.values() {
                        out.merge(analyze_sequence(value, false));
                    }
                }
                TemplateNode::Fd { positional, named }
                | TemplateNode::Ft { positional, named }
                | TemplateNode::Tip { positional, named }
                | TemplateNode::Tt { positional, named }
                | TemplateNode::Sti { positional, named }
                | TemplateNode::Ii { positional, named }
                | TemplateNode::Ais { positional, named }
                | TemplateNode::Ui { positional, named }
                | TemplateNode::Generic {
                    positional, named, ..
                } => {
                    for value in positional {
                        out.merge(analyze_sequence(value, false));
                    }
                    for value in named.values() {
                        out.merge(analyze_sequence(value, false));
                    }
                }
            },
        }
    }

    vec_dedup(&mut out.scalings);
    out
}

fn analyze_as(
    positional: &[Vec<MarkupNode>],
    named: &BTreeMap<String, Vec<MarkupNode>>,
) -> EffectAnalysis {
    let mut out = EffectAnalysis::default();

    // Only damage has a base value, unlike "{{as|20 bonus movement speed}}"
    let is_damage = positional
        .iter()
        .any(|value| flatten_nodes(value).to_ascii_lowercase().contains("damage"));

    if let Some(first) = positional.first() {
        out.merge(analyze_sequence(first, is_damage));
    }

    for value in positional.iter().skip(1) {
        out.merge(analyze_sequence(value, false));
    }

    for value in named.values() {
        out.merge(analyze_sequence(value, false));
    }

    out
}

fn contextualize_range_diff(
    siblings: &[MarkupNode],
    idx: usize,
    melee: &[MarkupNode],
    ranged: &[MarkupNode],
) -> Vec<Scaling> {
    let suffix = contextual_suffix(&siblings[idx + 1..]);
    let melee_text = join_nonempty([flatten_nodes(melee), suffix.clone()]);
    let ranged_text = join_nonempty([flatten_nodes(ranged), suffix]);

    let mut out = Vec::new();

    if is_semantic_value_text(&melee_text) {
        out.push(Scaling::Other {
            raw: format!("melee: {}", normalize_whitespace(&melee_text)),
        });
    }

    if is_semantic_value_text(&ranged_text) {
        out.push(Scaling::Other {
            raw: format!("ranged: {}", normalize_whitespace(&ranged_text)),
        });
    }

    out
}

fn contextualize_progression(
    kind: &str,
    siblings: &[MarkupNode],
    idx: usize,
    positional: &[Vec<MarkupNode>],
    named: &BTreeMap<String, Vec<MarkupNode>>,
) -> Vec<Scaling> {
    let mut parts = Vec::new();

    let first = positional
        .first()
        .map(|v| flatten_nodes(v))
        .unwrap_or_default();
    let second = positional
        .get(1)
        .map(|v| flatten_nodes(v))
        .unwrap_or_default();
    let suffix = contextual_suffix(&siblings[idx + 1..]);

    if !first.is_empty() {
        parts.push(first);
    }
    if !second.is_empty() {
        parts.push(format!("axis: {second}"));
    }
    if let Some(v) = named.get("type") {
        parts.push(format!("type={}", flatten_nodes(v)));
    }
    if let Some(v) = named.get("levels") {
        parts.push(format!("levels={}", flatten_nodes(v)));
    }
    if let Some(v) = named.get("formula") {
        parts.push(format!("formula={}", flatten_nodes(v)));
    }
    if let Some(v) = named.get("key") {
        parts.push(format!("key={}", flatten_nodes(v)));
    }
    if let Some(v) = named.get("key1") {
        parts.push(format!("key1={}", flatten_nodes(v)));
    }
    if let Some(v) = named.get("pp") {
        parts.push(format!("pp={}", flatten_nodes(v)));
    }
    if !suffix.is_empty() {
        parts.push(suffix);
    }

    let raw = format!("{kind}: {}", parts.join(" | "));
    let raw = normalize_whitespace(&raw);

    if raw == format!("{kind}:") || raw.is_empty() {
        Vec::new()
    } else {
        vec![Scaling::Other { raw }]
    }
}

fn contextual_suffix(nodes: &[MarkupNode]) -> String {
    // only use the right-side context; this is what preserves
    // "of target's current health" for rd/pp/ap without dragging in the whole sentence.
    flatten_nodes(nodes)
}

fn parse_base_from_sequence(nodes: &[MarkupNode]) -> Option<Vec<f64>> {
    let text = normalize_whitespace(&flatten_base_text(nodes));
    if text.is_empty() || text.contains('%') {
        return None;
    }

    let mut buf = String::new();
    let mut started = false;

    for ch in text.chars() {
        let allowed = ch.is_ascii_digit() || ch == '.' || ch == '/' || ch.is_whitespace();

        if !started {
            if ch.is_ascii_digit() {
                started = true;
                buf.push(ch);
            }
            continue;
        }

        if allowed {
            buf.push(ch);
        } else {
            break;
        }
    }

    let buf = normalize_whitespace(&buf);
    if buf.is_empty() {
        return None;
    }

    let values = buf
        .split('/')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse().ok())
        .collect::<Vec<_>>();

    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn flatten_base_text(nodes: &[MarkupNode]) -> String {
    let mut out = String::new();

    for node in nodes {
        match node {
            MarkupNode::Text { text } => out.push_str(text),
            MarkupNode::Link { label, target } => out.push_str(label.as_deref().unwrap_or(target)),
            MarkupNode::Template(template) => match template {
                // ignore semantically-rich templates when extracting flat base
                TemplateNode::As { .. }
                | TemplateNode::Rd { .. }
                | TemplateNode::Pp { .. }
                | TemplateNode::Ap { .. } => {}
                // for simple wrappers, keep visible text
                TemplateNode::Fd { positional, .. }
                | TemplateNode::Ft { positional, .. }
                | TemplateNode::Tip { positional, .. }
                | TemplateNode::Tt { positional, .. }
                | TemplateNode::Sti { positional, .. }
                | TemplateNode::Ii { positional, .. }
                | TemplateNode::Ais { positional, .. }
                | TemplateNode::Ui { positional, .. }
                | TemplateNode::Generic { positional, .. } => {
                    out.push_str(
                        &positional
                            .iter()
                            .map(|v| flatten_nodes(v))
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                }
            },
        }
    }

    out
}

fn is_semantic_value_text(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.chars().any(|c| c.is_ascii_digit())
        && (text.contains('%')
            || text.contains("damage")
            || text.contains("health")
            || text.contains("mana")
            || text.contains("armor")
            || text.contains("magic resist")
            || text.contains("movement speed")
            || text.contains("attack speed")
            || text.contains("ad")
            || text.contains("ap")
            || text.contains("lethality")
            || text.contains("cooldown"))
}

fn parse_scalings_from_plain_text(input: &str) -> Vec<Scaling> {
    let text = normalize_whitespace(input);

    if text.is_empty() {
        return Vec::new();
    }

    let blocks = extract_plus_paren_blocks(&text);
    if !blocks.is_empty() {
        return blocks
            .iter()
            .map(|raw| Scaling::parse(raw))
            .collect::<Vec<_>>();
    }

    let mut out = Vec::new();

    if let Some(s) = parse_percent_attr(&text) {
        out.push(s);
    } else if let Some(s) = parse_ranked_per100(&text) {
        out.push(s);
    } else if let Some(s) = parse_per100(&text) {
        out.push(s);
    } else if let Some(s) = parse_simple_or_ranked(&text) {
        out.push(s);
    }

    out
}

fn parse_simple_or_ranked(input: &str) -> Option<Scaling> {
    let text = normalize_whitespace(input);
    let (left, right) = split_percent_tail(&text)?;
    let ctx_var = assign_ctx_var(right);

    if left.contains('/') {
        let values = parse_slash_f64s(left)
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        Some(Scaling::Ranked { values, ctx_var })
    } else {
        let value = left.parse::<f64>().ok()? / 100.0;
        Some(Scaling::Simple { value, ctx_var })
    }
}

fn parse_ranked_per100(input: &str) -> Option<Scaling> {
    let text = normalize_whitespace(input);
    let lower = text.to_ascii_lowercase();

    let idx = lower.find(" per 100")?;
    let left = text[..idx].trim();
    let right = text[idx + " per 100".len()..]
        .trim()
        .trim_start_matches('%')
        .trim();

    let (values_raw, _) = split_percent_tail(left)?;
    let values = parse_slash_f64s(values_raw)
        .into_iter()
        .map(|v| v / 100.0)
        .collect::<Vec<_>>();

    Some(Scaling::RankedPer100 {
        values,
        ctx_var: assign_ctx_var(right),
    })
}

fn parse_per100(input: &str) -> Option<Scaling> {
    let text = normalize_whitespace(input);
    let lower = text.to_ascii_lowercase();

    let idx = lower.find(" per 100")?;
    let left = text[..idx].trim();
    let right = text[idx + " per 100".len()..]
        .trim()
        .trim_start_matches('%')
        .trim();

    let (value_raw, _) = split_percent_tail(left)?;
    let value = value_raw.parse::<f64>().ok()? / 100.0;

    Some(Scaling::Per100 {
        value,
        ctx_var: assign_ctx_var(right),
    })
}

fn parse_percent_attr(input: &str) -> Option<Scaling> {
    let text = normalize_whitespace(input);
    let lower = text.to_ascii_lowercase();
    let idx = lower.find("% of ")?;
    let value = text[..idx].trim().parse::<f64>().ok()? / 100.0;
    let debug = text[idx + 2..].trim().to_string();

    Some(Scaling::PercentAttr {
        value,
        debug: debug.clone(),
        ctx_var: assign_ctx_var(&debug),
    })
}

fn split_percent_tail(input: &str) -> Option<(&str, &str)> {
    let idx = input.find('%')?;
    let left = input[..idx].trim();
    let right = input[idx + 1..].trim();
    Some((left, right))
}

fn parse_slash_f64s(input: &str) -> Vec<f64> {
    input
        .split('/')
        .map(normalize_whitespace)
        .filter_map(|v| v.parse().ok())
        .collect()
}

fn extract_plus_paren_blocks(input: &str) -> Vec<String> {
    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i + 1 < chars.len() {
        let (_, c1) = chars[i];
        let (_, c2) = chars[i + 1];

        if c1 == '(' && c2 == '+' {
            let start = chars[i].0;
            let mut depth = 1usize;
            let mut j = i + 1;

            while j + 1 < chars.len() {
                j += 1;
                let (byte, ch) = chars[j];

                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end = byte + ch.len_utf8();
                            out.push(input[start..end].trim().to_string());
                            break;
                        }
                    }
                    _ => {}
                }
            }

            i = j;
        } else {
            i += 1;
        }
    }

    out
}

fn normalize_whitespace(input: &str) -> String {
    input
        .replace('\u{00a0}', " ")
        .replace(['\u{2013}', '\u{2014}', '\u{2212}'], "-")
        .replace('\u{00D7}', "*")
        .replace("<br>", " ")
        .replace("<br/>", " ")
        .replace("<br />", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn join_nonempty(parts: [String; 2]) -> String {
    parts
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Values of a `{{pp}}` progression by level, which replaced the template with
/// its first value in the resolved markup
#[derive(Clone, Debug)]
struct LevelSeries {
    values: Vec<f64>,
    levels: Vec<u8>,
    /// Whether the progression is colored as damage
    damage: bool,
}

/// Replaces the templates that compute values with the values themselves, so the
/// rest of the parser only sees numbers. `{{rd|melee|ranged}}` and `{{ft|text|alternate}}`
/// are resolved for the given variant, `{{ap}}` and `{{fd}}` are evaluated, and
/// `{{pp}}` progressions by level are replaced by their first value, with every
/// value saved in `series`
fn resolve_markup(
    input: &str,
    variant: Variant,
    vars: &mut BTreeMap<String, String>,
    series: &mut Vec<LevelSeries>,
) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;

    while let Some(offset) = input[i..].find("{{") {
        let start = i + offset;
        out.push_str(&input[i..start]);

        let Some(end) = find_balanced(input, start, "{{", "}}") else {
            out.push_str(&input[start..]);
            return out;
        };

        out.push_str(&resolve_template(
            &input[start + 2..end - 2],
            variant,
            vars,
            series,
        ));
        i = end;
    }

    out.push_str(&input[i..]);
    out
}

fn resolve_template(
    content: &str,
    variant: Variant,
    vars: &mut BTreeMap<String, String>,
    series: &mut Vec<LevelSeries>,
) -> String {
    if let Some(rest) = content.strip_prefix("#vardefineecho:") {
        let (name, value) = rest.split_once('|').unwrap_or((rest, ""));
        let value = resolve_markup(value, variant, vars, series);

        vars.insert(name.trim().to_string(), value.clone());

        return value;
    }

    if let Some(name) = content.strip_prefix("#var:") {
        return vars.get(name.trim()).cloned().unwrap_or_default();
    }

    let parts = split_top_level(content, '|');
    let name = parts[0].trim().to_ascii_lowercase();

    let mut positional = Vec::new();
    let mut named = BTreeMap::new();

    for part in &parts[1..] {
        match split_named_arg(part) {
            Some((k, v)) => {
                named.insert(k.trim().to_string(), v.trim().to_string());
            }
            None => positional.push(part.trim().to_string()),
        }
    }

    let mut resolve = |s: &str| resolve_markup(s, variant, vars, series);

    match name.as_str() {
        "rd" => {
            let pick = match variant.ranged {
                true => positional.get(1).or(positional.first()),
                false => positional.first(),
            }
            .cloned()
            .unwrap_or_default();

            // With `pp=true`, both values are progressions sharing the named arguments
            match named.get("pp").map(String::as_str) {
                Some("true") => {
                    let pick = resolve(&pick);

                    resolve_pp(&[pick], &named, series)
                }
                _ => resolve(&pick),
            }
        }
        "ap" => {
            let args = positional.iter().map(|p| resolve(p)).collect::<Vec<_>>();

            resolve_ap(&args)
        }
        "pp" => {
            let args = positional.iter().map(|p| resolve(p)).collect::<Vec<_>>();
            let named = named.iter().map(|(k, v)| (k.clone(), resolve(v))).collect();

            resolve_pp(&args, &named, series)
        }
        "fd" => positional.first().map(|p| resolve(p)).unwrap_or_default(),
        "ft" => match variant.flip {
            true => positional.get(1).or(positional.first()),
            false => positional.first(),
        }
        .map(|p| resolve(p))
        .unwrap_or_default(),
        // `26.6{{recurring|6}}` is 26.666...
        "recurring" => positional.first().map(|p| p.repeat(6)).unwrap_or_default(),
        _ => {
            let parts = parts.iter().map(|p| resolve(p)).collect::<Vec<_>>();
            format!("{{{{{}}}}}", parts.join("|"))
        }
    }
}

fn format_value(value: f64) -> String {
    let rounded = (value * 1e6).round() / 1e6;
    let mut s = format!("{rounded}");

    if s == "-0" {
        s = "0".into();
    }

    s
}

/// Values of `count` steps from `from` to `to`
fn linear(from: f64, to: f64, count: usize) -> Vec<f64> {
    match count {
        0 => Vec::new(),
        1 => vec![from],
        _ => (0..count)
            .map(|i| from + (to - from) * i as f64 / (count - 1) as f64)
            .collect(),
    }
}

/// `{{ap|60/6}}` evaluates the expression, `{{ap|30 to 60 3}}` lists 3 values and
/// `{{ap|80|100|120}}` lists its arguments, as ranks separated by slashes
fn resolve_ap(args: &[String]) -> String {
    let values = match args {
        [single] => match single.split_once(" to ") {
            Some((from, rest)) => {
                let mut rest = rest.split_whitespace();
                let to = rest.next().and_then(eval_expr);
                let count = rest.next().and_then(|n| n.parse().ok()).unwrap_or(5);

                match (eval_expr(from), to) {
                    (Some(from), Some(to)) => linear(from, to, count),
                    _ => return single.clone(),
                }
            }
            None => match eval_expr(single) {
                Some(value) => vec![value],
                None => return single.clone(),
            },
        },
        _ => match args
            .iter()
            .map(|a| eval_expr(a))
            .collect::<Option<Vec<_>>>()
        {
            Some(values) => values,
            None => return args.join(" / "),
        },
    };

    values
        .into_iter()
        .map(format_value)
        .collect::<Vec<_>>()
        .join(" / ")
}

/// Levels given as `1;9 to 20` or `6 to 18 by 1`
fn parse_levels(spec: &str) -> Option<Vec<u8>> {
    let mut levels = Vec::new();

    for part in spec.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        match part.split_once(" to ") {
            Some((from, rest)) => {
                let to = rest.split(" by ").next()?.trim().parse::<u8>().ok()?;
                levels.extend(from.trim().parse::<u8>().ok()?..=to);
            }
            None => levels.push(part.parse().ok()?),
        }
    }

    (!levels.is_empty()).then_some(levels)
}

/// Progressions like `{{pp|135 to 230}}` over levels 1 to 18, `{{pp|150 + 5*(x-1)
/// for 13|levels=1;9 to 20}}`, or `{{pp|0 to 40 for 11|0 to 100|type=Momentum}}`
/// over some other quantity. Progressions by level are replaced by their first value
/// and saved in `series`, while other ones are replaced by their last value
fn resolve_pp(
    args: &[String],
    named: &BTreeMap<String, String>,
    series: &mut Vec<LevelSeries>,
) -> String {
    let Some(first) = args.first() else {
        return String::new();
    };

    let suffix = match named.get("key").map(String::as_str) {
        Some("%") => "%",
        _ => "",
    };

    // Progressions are by level unless they name another axis, such as
    // "type=Momentum" or "label1=critical strike chance"
    let by_level = ["type", "label1"]
        .iter()
        .filter_map(|key| named.get(*key))
        .all(|t| t.to_ascii_lowercase().contains("level"));

    let levels = named
        .get("levels")
        .or(args.get(1).filter(|_| by_level))
        .and_then(|spec| parse_levels(spec));

    let (expr, count) = match first.rsplit_once(" for ") {
        Some((expr, n)) => (expr, n.trim().parse::<usize>().ok()),
        None => (first.as_str(), None),
    };

    let values = if first.contains(';') {
        first.split(';').map(eval_expr).collect::<Option<Vec<_>>>()
    } else if let Some((from, to)) = expr.split_once(" to ") {
        let count = count.or(levels.as_ref().map(Vec::len)).unwrap_or(MAX_LEVEL);

        eval_expr(from)
            .zip(eval_expr(to))
            .map(|(from, to)| linear(from, to, count))
    } else if expr.contains('x') {
        let count = count.or(levels.as_ref().map(Vec::len)).unwrap_or(MAX_LEVEL);
        (1..=count)
            .map(|x| eval_expr(&expr.replace('x', &format!("({x})"))))
            .collect::<Option<Vec<_>>>()
    } else {
        eval_expr(expr).map(|v| vec![v])
    };

    let Some(values) = values.filter(|v| !v.is_empty()) else {
        return first.clone();
    };

    if !by_level {
        return format_value(values[values.len() - 1]) + suffix;
    }

    let levels = match levels {
        Some(levels) if levels.len() == values.len() => levels,
        _ => (1..=values.len() as u8).collect(),
    };

    if values.len() > 1 {
        series.push(LevelSeries {
            values: values.clone(),
            levels,
            damage: named
                .get("color")
                .is_some_and(|color| color.contains("damage")),
        });
    }

    format_value(values[0]) + suffix
}

const MAX_LEVEL: usize = 18;

/// Evaluates arithmetic with `+ - * / ^` and parentheses
fn eval_expr(input: &str) -> Option<f64> {
    struct Parser<'a> {
        chars: std::iter::Peekable<std::str::Chars<'a>>,
    }

    impl Parser<'_> {
        fn skip(&mut self) {
            while self.chars.peek().is_some_and(|c| c.is_whitespace()) {
                self.chars.next();
            }
        }

        fn expr(&mut self) -> Option<f64> {
            let mut value = self.term()?;
            loop {
                self.skip();
                match self.chars.peek() {
                    Some('+') => {
                        self.chars.next();
                        value += self.term()?;
                    }
                    Some('-') => {
                        self.chars.next();
                        value -= self.term()?;
                    }
                    _ => return Some(value),
                }
            }
        }

        fn term(&mut self) -> Option<f64> {
            let mut value = self.power()?;

            loop {
                self.skip();

                match self.chars.peek() {
                    Some('*') => {
                        self.chars.next();
                        value *= self.power()?;
                    }
                    Some('/') => {
                        self.chars.next();
                        value /= self.power()?;
                    }
                    _ => return Some(value),
                }
            }
        }

        fn power(&mut self) -> Option<f64> {
            let base = self.atom()?;

            self.skip();

            match self.chars.peek() {
                Some('^') => {
                    self.chars.next();
                    Some(base.powf(self.power()?))
                }
                _ => Some(base),
            }
        }

        fn atom(&mut self) -> Option<f64> {
            self.skip();
            match self.chars.peek()? {
                '(' => {
                    self.chars.next();
                    let value = self.expr()?;
                    self.skip();
                    (self.chars.next()? == ')').then_some(value)
                }
                '-' => {
                    self.chars.next();
                    Some(-self.atom()?)
                }
                _ => {
                    let mut number = String::new();

                    while let Some(&c) = self.chars.peek() {
                        if !(c.is_ascii_digit() || c == '.') {
                            break;
                        }

                        number.push(c);

                        self.chars.next();
                    }

                    number.parse().ok()
                }
            }
        }
    }

    let mut parser = Parser {
        chars: input.trim().chars().peekable(),
    };

    let value = parser.expr()?;

    parser.skip();
    parser.chars.next().is_none().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn effect(description: &str, variant: Variant) -> (Effect, Option<Effect>) {
        analyze_effect(description, variant, &BTreeMap::new())
    }

    fn formula(effect: &Effect) -> String {
        effect.simplify_formula(Key::P).unwrap().unwrap_or_default()
    }

    const RANGED: Variant = Variant {
        ranged: true,
        flip: false,
    };

    const FLIP: Variant = Variant {
        ranged: false,
        flip: true,
    };

    #[test]
    fn evaluates_expressions() {
        assert_eq!(eval_expr("60/6"), Some(10.0));
        assert_eq!(eval_expr("(60/6)+10"), Some(20.0));
        assert_eq!(eval_expr("150 + 5*(2-1)"), Some(155.0));
        assert_eq!(eval_expr("2^3 - -1"), Some(9.0));
        assert_eq!(eval_expr("10 *"), None);
    }

    #[test]
    fn resolves_level_progressions() {
        let mut series = Vec::new();
        let named = BTreeMap::new();

        assert_eq!(resolve_pp(&["50 to 150".into()], &named, &mut series), "50");
        assert_eq!(series[0].values.len(), MAX_LEVEL);
        assert_eq!(series[0].values[MAX_LEVEL - 1], 150.0);

        // Values of other axes are replaced by their last value
        let named = BTreeMap::from([("type".into(), "Momentum".into())]);
        assert_eq!(
            resolve_pp(&["0 to 40 for 11".into()], &named, &mut series),
            "40"
        );
        assert_eq!(series.len(), 1);
    }

    #[test]
    fn resolves_ranged_and_level_values() {
        let description = "deals {{as|{{rd|20 to 40|10 to 20|pp=true}}|true damage}} \
            {{as|(+ {{rd|25%|20%}} '''bonus''' armor)|armor}} {{as|'''bonus''' true damage}}.";

        let (melee, _) = effect(description, Variant::default());
        let (ranged, _) = effect(description, RANGED);

        assert_eq!(melee.use_values.as_ref().unwrap()[0], 20.0);
        assert_eq!(ranged.use_values.as_ref().unwrap()[MAX_LEVEL - 1], 20.0);
        assert!(formula(&melee).contains("(0.25) * ctx.bonus_armor"));
        assert!(formula(&ranged).contains("(0.2) * ctx.bonus_armor"));
    }

    #[test]
    fn splits_flipped_text_and_totals() {
        let description = "take{{ft|{{as|1% of the target's '''maximum''' health}} every \
            {{fd|0.5}} seconds|{{as|6% of the target's '''maximum''' health}} total}}.";

        assert_eq!(
            formula(&effect(description, Variant::default()).0),
            "(0.01) * ctx.enemy_max_health"
        );
        assert_eq!(
            formula(&effect(description, FLIP).0),
            "(0.06) * ctx.enemy_max_health"
        );

        let description = "take {{as|{{ap|60/6}} {{as|(+ {{ap|6/6}}% AP)}} magic damage}} \
            every {{fd|0.5}} seconds over 3 seconds, for a total of {{as|60|magic damage}} \
            {{as|(+ 6% AP)}}. Against monsters, deal {{as|10 '''bonus''' magic damage}}.";

        let (tick, total) = effect(description, Variant::default());
        assert_eq!(formula(&tick), "(10.0) + ((0.01) * ctx.ability_power)");
        assert_eq!(
            formula(&total.unwrap()),
            "(60.0) + ((0.06) * ctx.ability_power)"
        );
    }

    #[test]
    fn nests_ratios_of_the_target_health() {
        let description = "deal {{as|12% {{as|(+ 5% per 100 '''bonus''' AD)}} of target's \
            '''maximum''' health}} {{as|'''bonus''' physical damage}}";

        assert_eq!(
            formula(&effect(description, Variant::default()).0),
            "((0.12) + ((0.05) * (ctx.bonus_ad * 0.01))) * ctx.enemy_max_health"
        );

        let description = "deal {{as|'''bonus''' true damage}} equal to {{as|{{rd|{{fd|0.7}}%|\
            {{fd|0.5}}%}} (+ {{fd|0.1}}% per target's ''Reap'' stack) of the target's \
            '''maximum''' health}}";

        assert_eq!(
            formula(&effect(description, RANGED).0),
            "((0.005) + ((0.001) * ctx.stacks)) * ctx.enemy_max_health"
        );
    }
}
