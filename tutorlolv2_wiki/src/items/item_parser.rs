use crate::{
    client::MayFail,
    items::{ItemEffectRaw, ItemRaw, cache},
    parser::{Effect, EffectInner, Scaling, assign_ctx_var, vec_dedup},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tutorlolv2_types::Key;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemEffect {
    pub name: Option<String>,
    pub unique: Option<bool>,
    pub raw_description: Option<String>,
    pub ast: Vec<MarkupNode>,
    pub effect: Effect,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ItemEffects {
    pub pass: Option<ItemEffect>,
    pub act: Option<ItemEffect>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WikiItem {
    pub id: u32,
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

            assign_formula(value.effects.act.as_mut());
            assign_formula(value.effects.pass.as_mut());

            (name, value)
        })
        .collect::<BTreeMap<_, _>>();

    let json = serde_json::to_string_pretty(&result)?;
    crate::write(cache().join("parsed").with_extension("json"), &json)
}

fn parse_item(raw: ItemRaw) -> WikiItem {
    WikiItem {
        id: raw.id,
        tier: raw.tier,
        modes: raw.modes,
        stats: raw.stats,
        effects: ItemEffects {
            pass: raw.effects.pass.map(parse_item_effect),
            act: raw.effects.act.map(parse_item_effect),
        },
        recipe: raw.recipe,
        buy: raw.buy,
    }
}

fn parse_item_effect(raw: ItemEffectRaw) -> ItemEffect {
    let raw_description = raw.description.clone();
    let ast = raw
        .description
        .as_deref()
        .map(parse_markup)
        .unwrap_or_default();

    let mut analysis = analyze_sequence(&ast, false);
    vec_dedup(&mut analysis.scalings);

    ItemEffect {
        name: raw.name,
        unique: raw.unique,
        raw_description: raw_description.clone(),
        effect: Effect {
            index: 0,
            formula: None,
            inner: EffectInner {
                description: raw_description.unwrap_or_default(),
                leveling: String::new(),
            },
            scalings: analysis.scalings,
            use_formula: None,
            use_values: None,
            base: analysis.base,
        },
        ast,
    }
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

        if input[next_start..].starts_with("{{") {
            if let Some(end) = find_balanced(input, next_start, "{{", "}}") {
                let content = &input[next_start + 2..end - 2];
                out.push(MarkupNode::Template(parse_template(content)));
                i = end;
                continue;
            }
        }

        if input[next_start..].starts_with("[[") {
            if let Some(end) = find_balanced(input, next_start, "[[", "]]") {
                let content = &input[next_start + 2..end - 2];
                out.push(parse_link(content));
                i = end;
                continue;
            }
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
            let melee = positional.get(0).cloned().unwrap_or_default();
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

    if let Some(first) = positional.first() {
        out.merge(analyze_sequence(first, true));
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
        .replace('\u{2013}', "-")
        .replace('\u{2014}', "-")
        .replace('\u{2212}', "-")
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
