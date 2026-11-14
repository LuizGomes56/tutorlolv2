//! Select source presentation branches without evaluating their formula trees.

use super::{Arg, Node, Nodes, Template};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceVariant {
    pub ranged: bool,
    pub alternate: bool,
}

impl Nodes {
    /// Resolve `rd` (melee/ranged) and `ft` (primary/alternate) recursively.
    /// Variables must be expanded separately. Only selected branches are visited,
    /// so unsupported content in an unused presentation does not reject a source.
    pub fn for_variant(&self, variant: SourceVariant) -> Result<Self, String> {
        let mut out = Self::default();
        for node in &self.0 {
            match node {
                Node::Text(_) => out.push_variant_node(node.clone()),
                Node::Template(template) => {
                    for node in template.for_variant(variant)?.0 {
                        out.push_variant_node(node);
                    }
                }
            }
        }
        Ok(out)
    }

    fn push_variant_node(&mut self, node: Node) {
        match (self.0.last_mut(), node) {
            (Some(Node::Text(previous)), Node::Text(text)) => previous.push_str(&text),
            (_, node) => self.0.push(node),
        }
    }
}

impl Template {
    fn for_variant(&self, variant: SourceVariant) -> Result<Nodes, String> {
        let branch = match self.name.as_str() {
            "rd" => Some(usize::from(variant.ranged)),
            "ft" => Some(usize::from(variant.alternate)),
            _ => None,
        };
        if let Some(index) = branch {
            let selected = self
                .positional(index)
                .ok_or_else(|| {
                    format!(
                        "{} template is missing selected branch {}",
                        self.name,
                        index + 1
                    )
                })?
                .for_variant(variant)?;
            let progression = self.name == "rd" && self.named("pp").is_some_and(|value| {
                matches!(value.0.as_slice(), [Node::Text(text)] if text.trim().eq_ignore_ascii_case("true"))
            });
            if !progression {
                return Ok(selected);
            }
            let mut args = vec![Arg {
                name: None,
                value: selected,
            }];
            if let Some(levels) = self.named("levels") {
                args.push(Arg {
                    name: None,
                    value: levels.for_variant(variant)?,
                });
            }
            for arg in &self.args {
                if arg.name.is_some() && !matches!(arg.name.as_deref(), Some("pp" | "levels")) {
                    args.push(Arg {
                        name: arg.name.clone(),
                        value: arg.value.for_variant(variant)?,
                    });
                }
            }
            return Ok(Nodes(vec![Node::Template(Self {
                name: "pp".into(),
                args,
            })]));
        }
        let args = self
            .args
            .iter()
            .map(|arg| {
                Ok(Arg {
                    name: arg.name.clone(),
                    value: arg.value.for_variant(variant)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Nodes(vec![Node::Template(Self {
            name: self.name.clone(),
            args,
        })]))
    }
}
