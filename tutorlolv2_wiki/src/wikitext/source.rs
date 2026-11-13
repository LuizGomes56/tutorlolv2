//! Named values extracted directly from wikitext descriptions.
//! Roles annotate source context; strict queries reject ambiguous selections.
use super::{
    Arg, Expr, Formula, Node, Nodes, PageContext, PassiveProgression, SourceVariant, Template,
    Variables,
};
use serde::{Deserialize, Serialize};
use tutorlolv2_types::Key;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceRole {
    Damage,
    Healing,
    Amplification,
    ResourceRestore,
    Shield,
    Condition,
    Stat,
    Other,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceForm {
    Base,
    Maximum,
    Total,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceProgression {
    pub axis: Option<String>,
    pub keys: Vec<f64>,
    pub values: Vec<f64>,
    pub unit: String,
    pub key_unit: String,
    pub stepped: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceValue {
    pub role: SourceRole,
    pub form: SourceForm,
    pub raw: String,
    pub context: String,
    /// Local clause around the value, useful for explicit target/event selection.
    pub surrounding: String,
    pub following: String,
    pub unit: String,
    pub expression: Option<Formula>,
    pub issues: Vec<String>,
    /// Exact original numeric domains, including unsupported non-level axes.
    pub progressions: Vec<SourceProgression>,
}
impl SourceValue {
    pub fn formula(&self) -> Result<&Formula, String> {
        self.expression.as_ref().ok_or_else(|| {
            format!(
                "Unsupported value {:?}/{:?}: {:?}; source {}",
                self.role, self.form, self.issues, self.raw
            )
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceDocument {
    pub name: String,
    pub source: String,
    pub variant: SourceVariant,
    pub values: Vec<SourceValue>,
}
impl SourceDocument {
    pub fn parse(
        name: &str,
        source: &str,
        context: &PageContext,
        variant: SourceVariant,
    ) -> Result<Self, String> {
        let nodes = Nodes::parse(source).for_variant(variant)?;
        let nodes = Variables::default().expand(&context.lookup.expand(&nodes));
        let mut document = Self::from_nodes(name, &nodes, context, variant)?;
        document.source = source.into();
        Ok(document)
    }

    /// Accepts item-scoped variable expansion performed before individual slots.
    pub fn from_nodes(
        name: &str,
        nodes: &Nodes,
        context: &PageContext,
        variant: SourceVariant,
    ) -> Result<Self, String> {
        // Variables may contain template markup. Reparse after expansion so a
        // shared item variable contributes its formula rather than literal text.
        let nodes = Nodes::parse(&nodes.to_string()).formatted_numbers()?;
        let mut candidates = nodes.description_values_at(Key::P);
        // Plain damage bases are grammar in the wikitext itself (no HTML/text
        // filtering): Dark Harvest writes `deals 30 {{as|(+ 11 per Soul)}} ...`.
        for (index, node) in nodes.0.iter().enumerate() {
            if candidates
                .iter()
                .any(|(start, end, _)| index >= *start && index < *end)
            {
                continue;
            }
            if let Node::Text(text) = node {
                for captures in
                    lazy_regex::regex!(r"(?i)\b(?:deals?|dealing)\s+(-?\d+(?:\.\d+)?)\s*$")
                        .captures_iter(text)
                {
                    let mut value = vec![Node::Text(captures[1].to_string())];
                    let length = Nodes::value_tail(&nodes.0[index + 1..]);
                    value.extend_from_slice(&nodes.0[index + 1..index + 1 + length]);
                    candidates.push((index, index + 1 + length, Nodes(value)));
                }
            } else if let Node::Template(template) = node
                && template.name == "as"
                && let Some(inner) = template.positional(0)
            {
                let text = inner.plain();

                if lazy_regex::regex!(r"^\s*\d+(?:\.\d+)?(?:%|\s)").is_match(&text)
                    && inner.templates().iter().any(|t| t.name == "as")
                {
                    let length = Nodes::value_tail(&nodes.0[index + 1..]);
                    let mut value = inner.0.clone();

                    value.extend_from_slice(&nodes.0[index + 1..index + 1 + length]);
                    candidates.push((index, index + 1 + length, Nodes(value)));

                    continue;
                }

                if lazy_regex::regex!(r"^\s*\d+(?:\.\d+)?%\s*$").is_match(&text) {
                    let length = Nodes::value_tail(&nodes.0[index + 1..]);
                    let mut value = vec![Node::Text(text.trim().replace('%', " %"))];

                    value.extend_from_slice(&nodes.0[index + 1..index + 1 + length]);
                    candidates.push((index, index + 1 + length, Nodes(value)));

                    continue;
                }

                if lazy_regex::regex!(
                        r"^\d+(?:\.\d+)?% (?:base |bonus |maximum |total )?(?:AD|AP|health|mana)(?: from items)?$"
                    )
                    .is_match(text.trim())
                    {
                        let length = Nodes::value_tail(&nodes.0[index + 1..]);
                        let mut value = vec![node.clone()];

                        value.extend_from_slice(&nodes.0[index + 1..index + 1 + length]);
                        candidates.push((index, index + 1 + length, Nodes(value)));

                        continue;
                    }

                if let Some(captures) =
                    lazy_regex::regex!(r"^(-?\d+(?:\.\d+)?)\s+([^%()]+)$").captures(text.trim())
                {
                    candidates.push((
                        index,
                        index + 1,
                        Nodes(vec![Node::Text(format!(
                            "{} {}",
                            &captures[1], &captures[2]
                        ))]),
                    ));
                }
            }
        }

        candidates.sort_by_key(|(start, _, _)| *start);

        let mut values = Vec::new();

        for (start, end, value) in candidates {
            let after = Nodes(nodes.0[end..].to_vec()).plain();
            let mut before = Nodes(nodes.0[..start].to_vec()).plain();

            if let Some(Node::Text(text)) = nodes.0.get(start) {
                before.push_str(text);
            }

            let form = Self::form(&before);
            let local_after = after.trim_start();
            let inferred_role = if value.plain().trim_end().ends_with("damage")
                || lazy_regex::regex!(
                    r"(?i)^(?:as |bonus |physical |magic |true |adaptive )*damage\b"
                )
                .is_match(local_after)
            {
                SourceRole::Damage
            } else {
                Self::role(&before)
            };

            let role = match (inferred_role, form) {
                (SourceRole::Amplification, SourceForm::Maximum) => SourceRole::Damage,
                (role, _) => role,
            };

            let model = value.effect_model(context, Key::P, name);
            let progressions = value
                .templates()
                .into_iter()
                .filter(|t| matches!(t.name.as_str(), "pp" | "pplevel"))
                .filter_map(|t| {
                    PassiveProgression::new(t).ok().map(|p| SourceProgression {
                        axis: p.axis,
                        keys: p.keys,
                        values: p.values,
                        unit: p.key,
                        key_unit: t.named("key1").map(Nodes::plain).unwrap_or_default(),
                        stepped: t
                            .positional(0)
                            .is_some_and(|v| v.to_string().contains(" by ")),
                    })
                })
                .collect::<Vec<_>>();

            let issues = model.issues;
            let expression = issues.is_empty().then_some(model.formula);

            values.push(SourceValue {
                role,
                form,
                raw: value.to_string(),
                surrounding: format!(
                    "{} {}",
                    before.rsplit(['.', ';', '\n']).next().unwrap_or(&before),
                    Nodes(nodes.0[end..].to_vec())
                        .plain()
                        .split(['.', ';', '\n'])
                        .next()
                        .unwrap_or("")
                ),
                following: after,
                context: before,
                unit: model.unit,
                expression,
                issues,
                progressions,
            });
        }

        Ok(Self {
            name: name.into(),
            source: nodes.to_string(),
            variant,
            values,
        })
    }

    /// Named selection is deliberately strict: inserted healing/threshold values
    /// do not move a damage selection, and duplicate damage requires a finer key.
    pub fn select(&self, role: SourceRole, form: SourceForm) -> Result<&SourceValue, String> {
        self.select_where(role, form, |_| true)
    }

    /// Strict semantic selection. The predicate expresses source-specific event,
    /// target or path context; zero and multiple matches are both diagnostics.
    pub fn select_where(
        &self,
        role: SourceRole,
        form: SourceForm,
        predicate: impl Fn(&SourceValue) -> bool,
    ) -> Result<&SourceValue, String> {
        let values = self
            .values
            .iter()
            .filter(|value| value.role == role && value.form == form && predicate(value))
            .collect::<Vec<_>>();
        match values.as_slice() {
            [value] => Ok(value),
            _ => Err(format!(
                "{}: expected one {role:?}/{form:?}, found {}; candidates {:?}",
                self.name,
                values.len(),
                self.values
                    .iter()
                    .map(|v| (v.role, v.form, &v.raw))
                    .collect::<Vec<_>>()
            )),
        }
    }

    fn role(prefix: &str) -> SourceRole {
        let prefix = prefix.replace("attack damage", "attack stat");
        let mut latest = None;
        for captures in lazy_regex::regex!(r"(?i)\b(deal|deals|dealing|damage|burn|burns|heal|heals|healing|restore|restores|shield|shields|below|above|under|gain|gains|grant|grants|increase|increases|increased|reap)\b").captures_iter(&prefix) {
            let word=captures[1].to_ascii_lowercase();
            let role=match word.as_str() {
                "deal"|"deals"|"dealing"|"damage"|"burn"|"burns"=>SourceRole::Damage,
                "heal"|"heals"|"healing"=>SourceRole::Healing,
                "restore"|"restores"=>SourceRole::ResourceRestore,
                "shield"|"shields"=>SourceRole::Shield,
                "below"|"above"|"under"=>SourceRole::Condition,
                "increased"=>SourceRole::Amplification,
                _=>SourceRole::Stat,
            };
            latest=Some(role);
        }
        latest.unwrap_or(SourceRole::Other)
    }
    fn form(prefix: &str) -> SourceForm {
        let prefix = prefix
            .to_ascii_lowercase()
            .replace("maximum health", "health");
        if lazy_regex::regex!(r"(?:maximum|at most)(?:\W+\w+){0,4}\W*$").is_match(&prefix) {
            SourceForm::Maximum
        } else if lazy_regex::regex!(r"total(?:\W+\w+){0,3}\W*$").is_match(&prefix) {
            SourceForm::Total
        } else {
            SourceForm::Base
        }
    }
}

impl Nodes {
    fn formatted_numbers(&self) -> Result<Self, String> {
        let mut out = Vec::new();
        for node in &self.0 {
            let node = match node {
                Node::Text(text) => Node::Text(text.clone()),
                Node::Template(template) if template.name == "fd" => {
                    let number = template
                        .positional(0)
                        .and_then(|text| Expr::new(&text.plain()).eval())
                        .ok_or_else(|| format!("Unsupported formatted number: {template}"))?;
                    Node::Text(number.to_string())
                }
                Node::Template(template) => Node::Template(Template {
                    name: template.name.clone(),
                    args: template
                        .args
                        .iter()
                        .map(|arg| {
                            Ok(Arg {
                                name: arg.name.clone(),
                                value: arg.value.formatted_numbers()?,
                            })
                        })
                        .collect::<Result<_, String>>()?,
                }),
            };
            match (out.last_mut(), node) {
                (Some(Node::Text(previous)), Node::Text(text)) => previous.push_str(&text),
                (_, node) => out.push(node),
            }
        }
        Ok(Self(out))
    }
}
