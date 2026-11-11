//! Scalings written with `{{as}}` (`Template:Ability scaling`), which only colors its
//! text. Their text follows a few shapes, where the ratio may be a number or `{{ap}}`:
//! - `(+ 45% AP)`, `(+ 140% bonus AD)` or `(+ 8% of her maximum health)`
//! - `(+ 5% per 100 AP)`, a ratio applied to every 100 of the stat
//! - `6% {{as|(+ 1.5% per 100 AP)}} of target's maximum health`, whose ratio scales too
//! - `4% of target's maximum health`, without `(+`, when the scaling is the whole value

use {
    super::{Node, Nodes, Progression, Template},
    lazy_regex::regex,
    serde::{Deserialize, Serialize},
    tutorlolv2_types::Key,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Scaling {
    /// Percentages, one per rank when the ratio levels up with the ability
    pub ratios: Vec<f64>,
    /// `100` of `per 100 AP`, when the ratio applies to every `per` of the stat
    pub per: Option<f64>,
    /// Stat as written, without markup, such as `bonus AD` or `of target's maximum health`
    pub stat: String,
    /// Whether it is written as `(+ ...)`, added to the values before it
    pub additive: bool,
    /// Scalings of the ratio itself, written right after it
    pub nested: Vec<Scaling>,
}

impl Scaling {
    /// Parses the `{{as}}` `template` of an ability of `skill`, or returns `None`
    /// when its text does not follow one of the known shapes
    pub fn new(template: &Template, skill: Key) -> Option<Self> {
        let text = template.positional(0)?;

        let mut ratios = None;
        let mut nested = Vec::new();
        let mut flat = String::new();

        // Templates are replaced by markers so only the text shape is matched
        for node in &text.0 {
            match node {
                Node::Text(text) => flat.push_str(text),
                Node::Template(inner) => match inner.name.as_str() {
                    "ap" if ratios.is_none() => {
                        ratios = Some(Progression::new(inner, skill).ok()?.numbers()?);
                        flat.push_str("RATIO");
                    }
                    "as" => {
                        nested.push(Self::new(inner, skill)?);
                        flat.push_str("NESTED");
                    }
                    "fd" => flat.push_str(&inner.positional(0)?.to_string()),
                    _ => flat.push_str(&Nodes(vec![node.clone()]).plain()),
                },
            }
        }

        let flat = flat.replace("'''", "").replace("''", "");
        let flat = regex!(r"\s+").replace_all(flat.trim(), " ");

        let captures = regex!(
            r"^(?<additive>\(\+ ?)?(?<ratio>RATIO|-?\d+(?:\.\d+)?)%(?: ?NESTED)*(?: per (?<per>\d+(?:\.\d+)?))? (?<stat>[^()]+?)(?<close>\))?$"
        )
        .captures(&flat)?;

        let additive = captures.name("additive").is_some();

        // Parentheses must be balanced
        if additive != captures.name("close").is_some() {
            return None;
        }

        let ratios = match &captures["ratio"] {
            "RATIO" => ratios?,
            ratio => vec![ratio.parse().ok()?],
        };

        Some(Self {
            ratios,
            per: captures.name("per").and_then(|per| per.as_str().parse().ok()),
            stat: captures["stat"].trim().to_string(),
            additive,
            nested,
        })
    }
}
