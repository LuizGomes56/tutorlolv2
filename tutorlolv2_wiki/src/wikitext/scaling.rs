//! Scalings written with `{{as}}` (`Template:Ability scaling`), which only colors its
//! text. Their text follows a few shapes, where the ratio may be a number or `{{ap}}`:
//! - `(+ 45% AP)`, `(+ 140% bonus AD)` or `(+ 8% of her maximum health)`
//! - `(+ 5% per 100 AP)`, a ratio applied to every 100 of the stat. Stats written
//!   in percent, as `(+ 750% per 100% bonus attack speed)`, are read the same way
//! - `6% {{as|(+ 1.5% per 100 AP)}} of target's maximum health`, whose ratio scales too
//! - `4% of target's maximum health`, without `(+`, when the scaling is the whole value
//! - `(+ 1.25% AP per 100 bonus health)`, a ratio of a stat that grows with another one

use {
    super::{Arg, Node, Nodes, Progression, Template},
    lazy_regex::regex,
    serde::{Deserialize, Serialize},
    tutorlolv2_types::Key,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Scaling {
    /// Percentages, one per rank when the ratio levels up with the ability
    pub ratios: Vec<f64>,
    /// `100` of `per 100 AP`, when the ratio applies to every `per` of the stat, or
    /// to every `per` of [`Self::per_stat`] when it is given
    pub per: Option<f64>,
    /// `bonus health` of `1.25% AP per 100 bonus health`, a ratio of `stat` that
    /// grows with another stat, as Tahm Kench's Q
    pub per_stat: Option<String>,
    /// Stat as written, without markup, such as `bonus AD` or `of target's maximum health`
    pub stat: String,
    /// Whether it is written as `(+ ...)`, added to the values before it
    pub additive: bool,
    /// Scalings of the ratio itself, written right after it
    pub nested: Vec<Scaling>,
    /// Power of the whole term, written as `(+ 1.14% AP)<sup>2</sup>`, which is
    /// `(0.0114 * AP)²`. 1 otherwise
    pub power: u8,
    /// `{{pp}}` of a ratio by level, as Xin Zhao's `(+ {{pp|5 to 20 for 4|…|key=%}} AP)`,
    /// in place of [`Self::ratios`]
    #[serde(skip)]
    pub table: Option<Template>,
}

impl Scaling {
    /// Parses the `{{as}}` `template` of an ability of `skill`, or returns `None`
    /// when its text does not follow one of the known shapes
    pub fn new(template: &Template, skill: Key) -> Option<Self> {
        let text = template.positional(0)?;

        let mut ratios = None;
        let mut table = None;
        let mut nested = Vec::new();
        let mut flat = String::new();

        // Templates are replaced by markers so only the text shape is matched
        for node in &text.0 {
            match node {
                Node::Text(text) => flat.push_str(text),
                Node::Template(inner) => match inner.name.as_str() {
                    "ap" if ratios.is_none() && table.is_none() => {
                        ratios = Some(Progression::new(inner, skill).ok()?.numbers()?);
                        flat.push_str("RATIO");
                    }
                    // A `{{pp}}` in percent, whose `%` is its `key=`
                    "pp" | "pplevel"
                        if ratios.is_none()
                            && table.is_none()
                            && inner
                                .named("key")
                                .is_some_and(|key| key.plain().trim() == "%") =>
                    {
                        table = Some(inner.clone());
                        flat.push_str("RATIO%");
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

        // The power is written after the closing parenthesis
        let (flat, power) = match regex!(r"^(.*?) ?<sup>(\d+)</sup>$").captures(&flat) {
            Some(captures) => (captures[1].to_string(), captures[2].parse().ok()?),
            None => (flat.into_owned(), 1),
        };

        // `(+ 1.5 per 1 Lethality)`, a ratio of each point of the stat, as Pyke's R
        if let Some(captures) = regex!(
            r"^\(\+ ?(?<ratio>-?\d+(?:\.\d+)?) per (?<per>\d+(?:\.\d+)?) (?<stat>[^()%]+?)\)$"
        )
        .captures(&flat)
            && nested.is_empty()
        {
            return Some(Self {
                ratios: vec![captures["ratio"].parse::<f64>().ok()? * 100.0],
                per: Some(captures["per"].parse().ok()?),
                per_stat: None,
                stat: captures["stat"].trim().to_string(),
                additive: true,
                nested,
                power,
                table: None,
            });
        }

        // Named collected units, as Dark Harvest's `(+ 11 per Soul)`.
        if let Some(captures) =
            regex!(r"^\(\+ ?(?<ratio>-?\d+(?:\.\d+)?) per (?<stat>[A-Za-z ]+)\)$").captures(&flat)
            && super::build::stat_var(captures["stat"].trim()) == tutorlolv2_types::CtxVar::Stacks
        {
            return Some(Self {
                ratios: vec![captures["ratio"].parse::<f64>().ok()? * 100.0],
                per: None,
                per_stat: None,
                stat: captures["stat"].trim().into(),
                additive: true,
                nested,
                power,
                table: None,
            });
        }

        let captures = regex!(
            r"^(?<additive>\(\+ ?)?(?<ratio>RATIO|-?\d+(?:\.\d+)?)%(?: ?NESTED)*(?: per (?<per>\d+(?:\.\d+)?)%?)? (?<stat>[^()]+?)(?: per (?<per2>\d+(?:\.\d+)?)%? (?<per_stat>[^()]+?))?(?<close>\))?$"
        )
        .captures(&flat)?;

        let additive = captures.name("additive").is_some();

        // Parentheses must be balanced
        if additive != captures.name("close").is_some() {
            return None;
        }

        let ratios = match &captures["ratio"] {
            "RATIO" if table.is_some() => Vec::new(),
            // `{{ap}}` without arguments has no ratio
            "RATIO" => ratios.filter(|ratios| !ratios.is_empty())?,
            ratio => vec![ratio.parse().ok()?],
        };

        Some(Self {
            ratios,
            per: captures
                .name("per")
                .or(captures.name("per2"))
                .and_then(|per| per.as_str().parse().ok()),
            per_stat: captures
                .name("per_stat")
                .map(|stat| stat.as_str().trim().to_string()),
            stat: captures["stat"].trim().to_string(),
            additive,
            nested,
            power,
            table,
        })
    }

    /// Parses a scaling written across two `{{as}}`, as
    /// `{{as|(+ 8%|hp}} {{as|of target's maximum health)}}`, where the first one opens
    /// the parenthesis that the second one closes
    pub fn merged(first: &Template, second: &Template, skill: Key) -> Option<Self> {
        let mut text = first.positional(0)?.0.clone();
        text.push(Node::Text(" ".into()));
        text.extend(second.positional(0)?.0.iter().cloned());

        let template = Template {
            name: "as".into(),
            args: vec![Arg {
                name: None,
                value: Nodes(text),
            }],
        };

        Self::new(&template, skill)
    }
}
