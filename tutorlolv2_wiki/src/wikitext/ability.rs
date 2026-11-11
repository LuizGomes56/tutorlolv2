//! Ability data templates, `Template:Data_{champion}/{ability}`, whose `leveling`
//! fields hold the values of the ability in `{{st}}` (`Template:Skill Tabs`) as
//! `{{st|label|value|label|value...}}`. Values are almost always a `{{ap}}` with
//! the base values, followed by a unit and the `{{as}}` of each scaling, as in
//! `{{st|Bonus Movement Speed|{{ap|25 to 55 6}}% {{as|(+ 5% per 100 bonus AD)}}}}`

use {
    super::{Node, Nodes, Progression, Scaling, Template, Variables},
    serde::{Deserialize, Serialize},
    tutorlolv2_types::Key,
};

/// Name of the template that holds the fields of an ability data page
const ABILITY_DATA: &str = "Ability data";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RawAbility {
    pub name: String,
    pub skill: Key,
    /// Value of the `damagetype` field, such as `Physical`
    pub damage_type: Option<String>,
    pub tabs: Vec<SkillTab>,
}

/// Label and value of a `{{st}}`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SkillTab {
    /// Field the tab comes from, such as `leveling2`
    pub field: String,
    pub label: String,
    /// Value of the first `{{ap}}`, if the value starts with one
    pub base: Option<Progression>,
    /// Text right after the base values, such as `%` or `seconds`
    pub unit: String,
    pub scalings: Vec<Scaling>,
    /// `{{as}}` and `{{pp}}` of the value that could not be parsed, as written
    pub unparsed: Vec<String>,
    /// Value as written, after variables were replaced
    pub raw: String,
}

impl RawAbility {
    /// Parses the source of an ability data page, which is `None` for pages that
    /// do not use `Template:Ability data`, such as the ones that only redirect
    pub fn new(src: &str) -> Result<Option<Self>, String> {
        let nodes = Variables::default().expand(&Nodes::parse(src));

        let Some(data) = nodes
            .templates()
            .into_iter()
            .find(|t| t.name == ABILITY_DATA)
        else {
            return Ok(None);
        };

        let field = |name: &str| {
            data.named(name)
                .map(|value| value.plain().trim().to_string())
        };
        let skill = field("skill").ok_or("Ability data without a skill field")?;
        let skill = match skill.chars().collect::<Vec<_>>()[..] {
            [c] => Key::try_from(c)?,
            _ => return Err(format!("Invalid skill {skill:?}")),
        };

        let tabs = data
            .args
            .iter()
            .filter_map(|arg| Some((arg.name.as_deref()?, &arg.value)))
            .filter(|(name, _)| name.starts_with("leveling"))
            .flat_map(|(name, value)| SkillTab::all(name, value, skill))
            .collect();

        Ok(Some(Self {
            name: data.positional(0).map(Nodes::plain).unwrap_or_default(),
            skill,
            damage_type: field("damagetype").filter(|value| !value.is_empty()),
            tabs,
        }))
    }
}

impl SkillTab {
    /// Tabs of every `{{st}}` of the `field` with `value`
    fn all(field: &str, value: &Nodes, skill: Key) -> Vec<Self> {
        value
            .templates()
            .into_iter()
            .filter(|template| template.name == "st")
            .flat_map(|st| {
                st.positionals()
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .filter_map(|pair| match pair {
                        [label, value] if !label.is_blank() => {
                            Some(Self::new(field, label.plain().trim(), value, skill))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn new(field: &str, label: &str, value: &Nodes, skill: Key) -> Self {
        let mut tab = Self {
            field: field.to_string(),
            label: label.to_string(),
            base: None,
            unit: String::new(),
            scalings: Vec::new(),
            unparsed: Vec::new(),
            raw: value.to_string().trim().to_string(),
        };

        let mut nodes = value.0.iter().skip_while(|node| match node {
            Node::Text(text) => text.trim().is_empty(),
            Node::Template(_) => false,
        });

        if let Some(Node::Template(first)) = nodes.clone().next()
            && first.name == "ap"
        {
            nodes.next();
            tab.base = Progression::new(first, skill).ok();

            if let Some(Node::Text(unit)) = nodes.clone().next() {
                nodes.next();
                tab.unit = unit.trim().to_string();
            }
        }

        for node in nodes {
            if let Node::Template(template) = node {
                tab.push(template, skill);
            }
        }

        tab
    }

    fn push(&mut self, template: &Template, skill: Key) {
        match template.name.as_str() {
            "as" => match Scaling::new(template, skill) {
                Some(scaling) => self.scalings.push(scaling),
                None => self.unparsed.push(template.to_string()),
            },
            "pp" | "pplevel" | "ap" => self.unparsed.push(template.to_string()),
            _ => {}
        }
    }
}
