//! Parser of the source of wiki pages, downloaded with `?action=raw`. Unlike the
//! rendered html parsed by [`crate::parser`], it keeps the templates that produce
//! each value, such as `{{ap|45 to 225}}` instead of `45 / 90 / 135 / 180 / 225`.
//!
//! Despite being saved with the `lua` extension, the source of `Template:` pages is
//! wikitext, a tree of `{{name|arg|key = value}}` templates mixed with text

use {
    lazy_regex::regex,
    std::{collections::BTreeMap, fmt},
};

pub mod ability;
pub mod build;
pub mod compare;
pub mod expr;
pub mod lookup;
pub mod map;
pub mod model;
pub mod passive;
pub mod progression;
pub mod scaling;

pub use {
    ability::{RawAbility, SkillTab},
    build::PageContext,
    compare::{Difference, LegacyFormula, Report},
    expr::Expr,
    lookup::Lookup,
    map::{MapChangeEntry, MapChangesReport, MapValue, PlacedEffect, Placement},
    model::{AbilityModel, ChampionModel, EffectModel, Formula, IntoFormula, Sample},
    passive::PassiveProgression,
    progression::{Progression, Value},
    scaling::Scaling,
};

/// Piece of wikitext, either plain text or a template call
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Text(String),
    Template(Template),
}

/// Template call such as `{{ap|45 to 225}}`. Parser functions such as
/// `{{#var:b1}}` are named by the part before the colon, `#var`, and the
/// part after it is their first argument
#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    pub name: String,
    pub args: Vec<Arg>,
}

/// Argument of a [`Template`], positional or `name = value`
#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub name: Option<String>,
    pub value: Nodes,
}

/// Sequence of [`Node`], displayed back as the wikitext it was parsed from
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Nodes(pub Vec<Node>);

impl Nodes {
    /// Parses `src` after removing what is only relevant to the wiki itself:
    /// comments, `<noinclude>` tags and the `{{{1|default}}}` parameters of the
    /// page, which are replaced by their default as when viewing the page
    pub fn parse(src: &str) -> Self {
        let mut src = regex!(r"(?s)<!--.*?(?:-->|$)")
            .replace_all(src, "")
            .into_owned();

        src = regex!(r"</?noinclude>").replace_all(&src, "").into_owned();

        loop {
            let replaced = regex!(r"\{\{\{[^{}|]*(?:\|([^{}]*))?\}\}\}")
                .replace_all(&src, "$1")
                .into_owned();

            if replaced == src {
                break;
            }

            src = replaced;
        }

        Parser::new(&src).nodes(false).0
    }

    pub fn is_blank(&self) -> bool {
        self.0.iter().all(|node| match node {
            Node::Text(text) => text.trim().is_empty(),
            Node::Template(_) => false,
        })
    }

    /// Every template, including the ones nested in arguments, outer first
    pub fn templates(&self) -> Vec<&Template> {
        let mut out = Vec::new();

        for node in &self.0 {
            if let Node::Template(template) = node {
                out.push(template);

                for arg in &template.args {
                    out.extend(arg.value.templates());
                }
            }
        }

        out
    }

    /// Text as displayed to readers, approximated by replacing templates with
    /// their first positional argument (the text of `{{ai}}` icons) and removing
    /// bold and italic markup
    pub fn plain(&self) -> String {
        self.0
            .iter()
            .map(|node| match node {
                Node::Text(text) => text.clone(),
                // `{{ai|Name|Champion|Text}}` displays the icon of `Name` next to `Text`
                Node::Template(template) if template.name == "ai" => template
                    .positional(2)
                    .or(template.positional(0))
                    .map(Nodes::plain)
                    .unwrap_or_default(),
                Node::Template(template) => {
                    template.positional(0).map(Nodes::plain).unwrap_or_default()
                }
            })
            .collect::<String>()
            .replace("'''", "")
            .replace("''", "")
    }
}

impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|node| match node {
            Node::Text(text) => f.write_str(text),
            Node::Template(template) => write!(f, "{template}"),
        })
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{{{}", self.name)?;

        for (i, arg) in self.args.iter().enumerate() {
            let separator = match i == 0 && self.name.starts_with('#') {
                true => ':',
                false => '|',
            };

            match &arg.name {
                Some(name) => write!(f, "{separator}{name}={}", arg.value)?,
                None => write!(f, "{separator}{}", arg.value)?,
            }
        }

        f.write_str("}}")
    }
}

impl Template {
    /// Value of the argument named `name`
    pub fn named(&self, name: &str) -> Option<&Nodes> {
        self.args
            .iter()
            .find(|arg| arg.name.as_deref() == Some(name))
            .map(|arg| &arg.value)
    }

    /// Value of the positional argument at `index`, starting at 0
    pub fn positional(&self, index: usize) -> Option<&Nodes> {
        self.args
            .iter()
            .filter(|arg| arg.name.is_none())
            .nth(index)
            .map(|arg| &arg.value)
    }

    pub fn positionals(&self) -> impl Iterator<Item = &Nodes> {
        self.args
            .iter()
            .filter(|arg| arg.name.is_none())
            .map(|arg| &arg.value)
    }
}

struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn rest(&self) -> &'a str {
        &self.src[self.pos..]
    }

    /// Parses until the end of the source or, inside a template, until the `|`
    /// or `}}` that ends the current argument, which is not consumed. Returns
    /// whether an argument separator or the end of the template was found
    fn nodes(&mut self, in_template: bool) -> (Nodes, bool) {
        let mut nodes = Vec::new();
        let mut text = String::new();

        let flush = |text: &mut String, nodes: &mut Vec<Node>| {
            if !text.is_empty() {
                nodes.push(Node::Text(std::mem::take(text)));
            }
        };

        while let Some(c) = self.rest().chars().next() {
            let rest = self.rest();

            if in_template && (rest.starts_with("}}") || c == '|') {
                flush(&mut text, &mut nodes);
                return (Nodes(nodes), true);
            }

            if rest.starts_with("{{") {
                let start = self.pos;
                self.pos += 2;

                match self.template() {
                    Some(template) => {
                        flush(&mut text, &mut nodes);
                        nodes.push(Node::Template(template));
                    }
                    // Unclosed braces are kept as text
                    None => {
                        self.pos = start + 2;
                        text.push_str("{{");
                    }
                }

                continue;
            }

            // The `|` of links such as `[[File:Icon.png|20px]]` does not separate arguments
            if rest.starts_with("[[") {
                let link = self.link();
                text.push_str(link);
                continue;
            }

            text.push(c);
            self.pos += c.len_utf8();
        }

        flush(&mut text, &mut nodes);
        (Nodes(nodes), !in_template)
    }

    /// Consumes a link and returns it as written, including nested links
    fn link(&mut self) -> &'a str {
        let start = self.pos;
        let mut depth = 0;

        while !self.rest().is_empty() {
            if self.rest().starts_with("[[") {
                depth += 1;
                self.pos += 2;
            } else if self.rest().starts_with("]]") {
                depth -= 1;
                self.pos += 2;

                if depth == 0 {
                    break;
                }
            } else {
                self.pos += self.rest().chars().next().map_or(1, char::len_utf8);
            }
        }

        &self.src[start..self.pos]
    }

    /// Parses the template whose `{{` was just consumed, up to its `}}`
    fn template(&mut self) -> Option<Template> {
        let mut segments = Vec::new();

        loop {
            let (nodes, closed) = self.nodes(true);

            if !closed {
                return None;
            }

            segments.push(nodes);

            if self.rest().starts_with("}}") {
                self.pos += 2;
                break;
            }

            // Separator `|`
            self.pos += 1;
        }

        let mut segments = segments.into_iter();
        let mut head = segments.next()?.0;

        let function = match head.first() {
            Some(Node::Text(text)) if text.trim_start().starts_with('#') => text
                .split_once(':')
                .map(|(name, rest)| (name.trim().to_string(), rest.to_string())),
            _ => None,
        };

        let (name, first) = match function {
            // `{{#var:b1}}`, where the part after the colon is the first argument,
            // which is never named, even if it has a `=`
            Some((name, rest)) => {
                head[0] = Node::Text(rest);
                head.retain(|node| node != &Node::Text(String::new()));

                let first = Arg {
                    name: None,
                    value: Nodes(head),
                };

                (name, Some(first))
            }
            None => (Nodes(head).to_string().trim().to_string(), None),
        };

        Some(Template {
            name,
            args: first.into_iter().chain(segments.map(Arg::new)).collect(),
        })
    }
}

impl Arg {
    /// Arguments are named when their text before any template has a `=`, in which
    /// case the name and value are trimmed, as the wiki does. A name has no link, so
    /// the `=` of `[[File:Icon.png|link=]]` does not name its argument
    fn new(value: Nodes) -> Self {
        let mut nodes = value.0;

        if let Some(Node::Text(text)) = nodes.first()
            && let Some((name, rest)) = text.split_once('=')
            && !name.contains("[[")
        {
            let name = name.trim().to_string();
            nodes[0] = Node::Text(rest.trim_start().to_string());

            if let Some(Node::Text(last)) = nodes.last_mut() {
                *last = last.trim_end().to_string();
            }

            nodes.retain(|node| node != &Node::Text(String::new()));

            return Self {
                name: Some(name),
                value: Nodes(nodes),
            };
        }

        Self {
            name: None,
            value: Nodes(nodes),
        }
    }
}

/// Variables of a page, defined with `{{#vardefine:name|value}}` and read with
/// `{{#var:name}}`, as provided by the Variables extension of the wiki
#[derive(Clone, Debug, Default)]
pub struct Variables(pub BTreeMap<String, String>);

impl Variables {
    /// Replaces the parser functions `#vardefine`, `#vardefineecho`, `#var` and `#expr`
    /// by their result, in the order they appear, as the wiki does when rendering
    pub fn expand(&mut self, nodes: &Nodes) -> Nodes {
        let mut out = Vec::new();

        for node in &nodes.0 {
            let Node::Template(template) = node else {
                out.push(node.clone());
                continue;
            };

            let text = |this: &mut Self, index: usize| -> String {
                template
                    .positional(index)
                    .map(|value| this.expand(value).to_string().trim().to_string())
                    .unwrap_or_default()
            };

            match template.name.as_str() {
                "#vardefine" | "#vardefineecho" => {
                    let name = text(self, 0);
                    let value = text(self, 1);

                    if template.name == "#vardefineecho" {
                        out.push(Node::Text(value.clone()));
                    }

                    self.0.insert(name, value);
                }
                "#var" => {
                    let name = text(self, 0);
                    let value = match self.0.get(&name) {
                        Some(value) => value.clone(),
                        None => text(self, 1),
                    };

                    out.push(Node::Text(value));
                }
                "#expr" => {
                    let expression = text(self, 0);
                    let value = match Expr::new(&expression).eval() {
                        Some(value) => Value::Number(value).to_string(),
                        None => expression,
                    };

                    out.push(Node::Text(value));
                }
                _ => {
                    let args = template
                        .args
                        .iter()
                        .map(|arg| Arg {
                            name: arg.name.clone(),
                            value: self.expand(&arg.value),
                        })
                        .collect();

                    out.push(Node::Template(Template {
                        name: template.name.clone(),
                        args,
                    }));
                }
            }
        }

        // Text produced by the expansion is merged with the text around it
        let mut merged = Vec::<Node>::new();

        for node in out {
            match (merged.last_mut(), node) {
                (Some(Node::Text(last)), Node::Text(text)) => last.push_str(&text),
                (_, node) => merged.push(node),
            }
        }

        Nodes(merged)
    }
}
