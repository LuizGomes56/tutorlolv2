//! Builds the [`AbilityModel`] of an ability data page, and the [`ChampionModel`]
//! of every page of a champion. Each `{{st}}` label and value becomes an effect
//! whose formula is the sum of the terms of the value: the base `{{ap}}` or `{{pp}}`,
//! rank values written in the text, and each `{{as}}` scaling

use {
    super::{
        AbilityModel, Arg, ChampionModel, EffectModel, Expr, Formula, Lookup, Node, Nodes,
        PassiveProgression, Progression, Scaling, Template, Value, Variables,
    },
    crate::MayFail,
    crate::parser::assign_ctx_var,
    std::{
        collections::BTreeMap,
        path::{Path, PathBuf},
        str::FromStr,
    },
    tutorlolv2_types::{CtxVar, DamageType, Key},
};

/// Name of the template that holds the fields of an ability data page
const ABILITY_DATA: &str = "Ability data";

/// What the pages of one champion share
pub struct PageContext<'a> {
    pub lookup: &'a Lookup,
    /// Whether the champion's ultimate has 4 ranks and is learned at level 1,
    /// whose rank also defines the values of its other abilities
    pub four_rank_ultimate: bool,
    /// Key of every ability of the champion by name, to resolve a `{{pp}}` by the
    /// rank of another ability, as `type=Realm Warp's Rank`
    pub ability_keys: BTreeMap<String, Key>,
}

impl AbilityModel {
    /// Builds the model of an ability data page, with its key. `None` for pages
    /// without `Ability data`, such as the ones that only redirect. Fails when the
    /// `skill` field is not a single key
    pub fn new(src: &str, context: &PageContext) -> Result<Option<(Key, Self)>, String> {
        let nodes = Variables::default().expand(&context.lookup.expand(&Nodes::parse(src)));

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
        let key = match skill.chars().collect::<Vec<_>>()[..] {
            [c] => Key::try_from(c)?,
            _ => return Err(format!("Invalid skill {skill:?}")),
        };

        let mut model = Self {
            page: String::new(),
            name: data
                .positional(0)
                .map(|name| name.plain().trim().to_string())
                .unwrap_or_default(),
            damage_type: DamageType::from_str(&Self::capitalize(
                &field("damagetype").unwrap_or_default(),
            ))
            .unwrap_or(DamageType::Unspecified),
            effects: Vec::new(),
            extras: Vec::new(),
        };

        for arg in &data.args {
            let Some(name) = arg.name.as_deref() else {
                continue;
            };

            if let Some(suffix) = name.strip_prefix("leveling") {
                let section = suffix.parse::<u8>().map_or(0, |n| n.saturating_sub(1));

                for st in arg.value.templates().into_iter().filter(|t| t.name == "st") {
                    let args = st.positionals().collect::<Vec<_>>();

                    for pair in args.chunks(2) {
                        if let [label, value] = pair
                            && !label.is_blank()
                        {
                            let terms = Terms::new(context, key);
                            model
                                .effects
                                .push(terms.effect(label.plain().trim(), section, value));
                        }
                    }
                }
            }

            if name.starts_with("description") {
                for pp in arg.value.templates() {
                    if matches!(pp.name.as_str(), "pp" | "pplevel") {
                        let mut terms = Terms::new(context, key);
                        let unit = terms.table(pp).unwrap_or_default();
                        model.extras.push(terms.finish(name, 0, unit));
                    }
                }
            }
        }

        Ok(Some((key, model)))
    }

    /// `magic` is written as `Magic`, as [`DamageType`] parses it
    fn capitalize(text: &str) -> String {
        let mut chars = text.chars();

        match chars.next() {
            Some(first) => first.to_uppercase().chain(chars).collect(),
            None => String::new(),
        }
    }
}

/// Terms of the value of one effect, and what could not be understood
struct Terms<'a> {
    context: &'a PageContext<'a>,
    key: Key,
    terms: Vec<Formula>,
    issues: Vec<String>,
    /// Indexes of the terms written in percent, as `{{as|{{ap|6 to 8}}%|hp}}`,
    /// which become ratios when a stat follows them
    percents: Vec<usize>,
    /// Terms written before `{{times}}`, which multiply the ones after it
    multiplied: Option<Formula>,
    /// Index of the first term inside a parenthesis that a stat closes later
    group: Option<usize>,
}

impl<'a> Terms<'a> {
    /// Templates that display a value next to an icon or in a tooltip, whose
    /// `{{ap}}` or `{{pp}}` is the value itself, as Karma's
    /// `{{ai|Mantra|Karma|{{ap|40 to 220 4}}}}`
    const WRAPPERS: [&str; 3] = ["ai", "g", "tt"];

    fn new(context: &'a PageContext<'a>, key: Key) -> Self {
        Self {
            context,
            key,
            terms: Vec::new(),
            issues: Vec::new(),
            percents: Vec::new(),
            multiplied: None,
            group: None,
        }
    }

    fn effect(mut self, label: &str, section: u8, value: &Nodes) -> EffectModel {
        let nodes = value
            .0
            .iter()
            .skip_while(|node| matches!(node, Node::Text(text) if text.trim().is_empty()))
            .collect::<Vec<_>>();

        let mut unit = String::new();
        let mut rest = &nodes[..];

        // The base, with the unit written right after it
        match rest {
            [Node::Template(first), tail @ ..]
                if matches!(first.name.as_str(), "ap" | "pp" | "pplevel" | "fd") =>
            {
                match first.name.as_str() {
                    // `{{fd|1.7}}`, a number with its decimals formatted
                    "fd" => match first
                        .positional(0)
                        .and_then(|n| Expr::new(&n.plain()).eval())
                    {
                        Some(value) => self.terms.push(Formula::Const(value)),
                        None => self.issues.push(first.to_string()),
                    },
                    _ => self.value(first),
                }

                rest = tail;

                if let [Node::Text(text), ..] = rest {
                    unit = text
                        .split('(')
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                }
            }
            [Node::Text(text), tail @ ..] => {
                // A base written as a plain number, as Cho'Gath's `1200 {{as|(+ 50% AP)}}`
                let head = text.split('(').next().unwrap_or_default().trim();
                let mut words = head.splitn(2, char::is_whitespace);

                match words.next().and_then(|number| Expr::new(number).eval()) {
                    Some(value) => {
                        self.terms.push(Formula::Const(value));
                        unit = words.next().unwrap_or_default().trim().to_string();
                    }
                    None if !head.is_empty() => self.issues.push(head.to_string()),
                    None => {}
                }

                rest = tail;
            }
            _ => {}
        }

        // A base per collected thing, as Thresh's `1.7 per {{ai|…|Soul}} collected`,
        // is by stacks. The icon of the thing is not a value
        let mut stack_icon = false;

        if (unit == "per" || unit.starts_with("per "))
            && let Some(base) = self.terms.pop()
        {
            self.terms.push(base * Formula::Var(CtxVar::Stacks));
            stack_icon = true;
        }

        // A base that is a percent of a stat, as Yone's `{{ap|25 to 35}}% of damage dealt`
        if let Some(stat) = unit.strip_prefix('%').map(str::trim)
            && stat.starts_with("of ")
            && let Some(base) = self.terms.pop()
        {
            let stat = self.stat(stat);
            self.terms.push(base.percent() * Formula::Var(stat));
            unit = String::new();
        }

        self.read(rest, &mut stack_icon);
        self.finish(label, section, unit)
    }

    /// Reads the terms of `nodes`, the value after its base
    fn read(&mut self, nodes: &[&Node], stack_icon: &mut bool) {
        let mut index = 0;

        while let Some(node) = nodes.get(index) {
            index += 1;

            let template = match node {
                Node::Template(template) => template,
                Node::Text(text) => {
                    // `(+ 2 per {{ai|…|Soul}} collected)`, as Thresh's W, is by stacks
                    if let Some(captures) =
                        lazy_regex::regex!(r"\(\+ ?(\d+(?:\.\d+)?) per\s*$").captures(text)
                        && let Some(Node::Template(icon)) = nodes.get(index)
                        && icon.name == "ai"
                    {
                        let value = captures[1].parse().unwrap_or_default();
                        self.terms
                            .push(Formula::Const(value) * Formula::Var(CtxVar::Stacks));
                        index += 1;
                    } else if let Some(end) = self.text_scaling(text, &nodes[index..]) {
                        index += end;
                    }

                    continue;
                }
            };

            match template.name.as_str() {
                "times" => {
                    self.multiply();
                    continue;
                }
                "ai" if std::mem::take(stack_icon) && Self::wrapped(template).is_none() => {
                    continue;
                }
                // An icon alone, as Tahm Kench's `{{ai|An Acquired Taste|…|icononly=true}}`
                "ai" if template.named("icononly").is_some() => continue,
                "as" => {}
                _ => {
                    // A scaling inside an icon, as Gwen's `{{ai|A Thousand Cuts|Gwen|{{as|…}}}}`
                    match Self::wrapped_scaling(template)
                        .and_then(|inner| Scaling::new(inner, self.key))
                    {
                        Some(scaling) => self.scaling(&scaling),
                        None => self.value(template),
                    }

                    continue;
                }
            }

            if let Some((value, percent)) = Self::colored_value(template) {
                if percent {
                    self.percents.push(self.terms.len());
                }

                self.value(value);
                continue;
            }

            if let Some(scaling) = Scaling::new(template, self.key) {
                self.scaling(&scaling);
                continue;
            }

            if self.opening(template)
                || self.trailing_stat(template)
                || self.multiplier(template)
                || self.container(template, stack_icon)
            {
                continue;
            }

            // A scaling written across this `{{as}}` and the next one
            let next = nodes[index..]
                .iter()
                .position(|node| !matches!(node, Node::Text(text) if text.trim().is_empty()))
                .map(|offset| index + offset);

            if let Some(next) = next
                && let Node::Template(second) = nodes[next]
                && second.name == "as"
                && let Some(scaling) = Scaling::merged(template, second, self.key)
            {
                self.scaling(&scaling);
                index = next + 1;
                continue;
            }

            self.issues.push(template.to_string());
        }
    }

    /// A scaling written as text around templates rather than inside `{{as}}`, as
    /// Sett's `(+ 25% {{as|(+ 25% per 100 bonus AD)}} of expended ''Grit'')`, where
    /// `text` has the `(+` and `after` are the nodes after it. Returns how many of
    /// `after` it took, up to the one with the last closing parenthesis it read
    fn text_scaling(&mut self, text: &str, after: &[&Node]) -> Option<usize> {
        let open = text.find("(+")?;
        let mut group = Vec::new();

        // Text after the closing parenthesis, which may open another scaling, with the
        // nodes after it
        let (taken, remainder, rest) = match text[open..].find(')') {
            Some(end) => {
                let end = open + end;
                group.push(Node::Text(text[open..=end].to_string()));
                (0, &text[end + 1..], after)
            }
            None => {
                let close = after
                    .iter()
                    .position(|node| matches!(node, Node::Text(text) if text.contains(')')))?;

                group.push(Node::Text(text[open..].to_string()));
                group.extend(after[..close].iter().map(|node| (*node).clone()));

                let Node::Text(closing) = after[close] else {
                    return None;
                };

                let end = closing.find(')').unwrap_or_default();
                group.push(Node::Text(closing[..=end].to_string()));
                (close + 1, &closing[end + 1..], &after[close + 1..])
            }
        };

        let template = Template {
            name: "as".into(),
            args: vec![Arg {
                name: None,
                value: Nodes(group),
            }],
        };

        let scaling = Scaling::new(&template, self.key)?;
        self.scaling(&scaling);

        // Scalings written one after the other, as `(+ 1.25% AP per 100 bonus health)
        // (+ 4% bonus health)` once the `{{as}}` around them is removed
        let more = self.text_scaling(remainder, rest).unwrap_or(0);
        Some(taken + more)
    }

    /// `{{as}}` that only groups other `{{as}}`, as Tahm Kench's
    /// `{{as|({{as|+ 1.25% AP}} {{as|per 100 bonus health)|hp}} {{as|(+ 4% bonus health)}}}}`,
    /// whose pieces are read as if they were written without it
    fn container(&mut self, template: &Template, stack_icon: &mut bool) -> bool {
        let Some(text) = template.positional(0) else {
            return false;
        };

        let groups = text
            .0
            .iter()
            .any(|node| matches!(node, Node::Template(inner) if inner.name == "as"));
      
        let has_value = text
            .0
            .iter()
            .any(|node| matches!(node, Node::Template(inner) if inner.name == "ap"));

        if !groups || has_value {
            return false;
        }

        // Each inner `{{as}}` is replaced by its text, joined with the text around it
        let mut flat = Vec::<Node>::new();

        for node in &text.0 {
            let pieces = match node {
                Node::Template(inner) if inner.name == "as" => match inner.positional(0) {
                    Some(pieces) => pieces.0.clone(),
                    None => continue,
                },
                node => vec![node.clone()],
            };

            for piece in pieces {
                match (flat.last_mut(), piece) {
                    (Some(Node::Text(last)), Node::Text(text)) => last.push_str(&text),
                    (_, piece) => flat.push(piece),
                }
            }
        }

        self.read(&flat.iter().collect::<Vec<_>>(), stack_icon);
        true
    }

    /// Variable of a stat as written. Damage stored or dealt can't be computed, so it
    /// is read as stacks, and Sett's Grit is his resource, read as his maximum mana
    fn stat(&mut self, text: &str) -> CtxVar {
        let lowercase = text.to_lowercase();

        let var = if lowercase.contains("damage stored") || lowercase.contains("damage dealt") {
            CtxVar::Stacks
        } else if lowercase.contains("grit") {
            CtxVar::MaxMana
        } else {
            assign_ctx_var(text)
        };

        if var == CtxVar::Unknown {
            self.issues.push(format!("Unknown stat {text:?}"));
        }

        var
    }

    /// Sum of the terms read so far, which are taken
    fn take_sum(&mut self) -> Option<Formula> {
        Self::sum(std::mem::take(&mut self.terms))
    }

    fn sum(terms: Vec<Formula>) -> Option<Formula> {
        let mut terms = terms.into_iter();
        let first = terms.next()?;
        Some(terms.fold(first, |sum, term| sum + term))
    }

    /// `{{as|(+ 8%|hp}}`, a ratio whose parenthesis a later `{{as|of …)}}` closes with
    /// its stat, as K'Sante's W. The ratio starts the group of terms the stat multiplies
    fn opening(&mut self, template: &Template) -> bool {
        let Some(text) = template.positional(0) else {
            return false;
        };

        let nodes = text
            .0
            .iter()
            .filter(|node| !matches!(node, Node::Text(text) if text.trim().is_empty()))
            .collect::<Vec<_>>();

        let start = self.terms.len();

        match nodes[..] {
            [Node::Text(open), Node::Template(ratio), Node::Text(percent)]
                if open.trim() == "(+" && ratio.name == "ap" && percent.trim() == "%" =>
            {
                self.value(ratio);
            }
            [Node::Text(text)] => {
                let Some(captures) =
                    lazy_regex::regex!(r"^\(\+ ?(-?\d+(?:\.\d+)?)%$").captures(text.trim())
                else {
                    return false;
                };

                let value = captures[1].parse().unwrap_or_default();
                self.terms.push(Formula::Const(value));
            }
            _ => return false,
        }

        if self.terms.len() > start {
            self.percents.push(start);
        }

        self.group = Some(start);
        true
    }

    /// `{{times}}`: the terms before it multiply the ones after it
    fn multiply(&mut self) {
        match self.take_sum() {
            Some(sum) => self.multiplied = Some(sum),
            None => self.issues.push("{{times}} without terms before it".into()),
        }

        self.percents.clear();
    }

    /// `{{as|of target's maximum health}}`, the stat of the ratio written before it,
    /// as Yorick's `{{as|{{ap|6 to 8}}%|hp}} {{as|(+ 3% per 100 AP)}} {{as|of …}}`
    fn trailing_stat(&mut self, template: &Template) -> bool {
        let Some(text) = template.positional(0).map(|text| text.plain()) else {
            return false;
        };

        let text = text.trim();

        if !text.starts_with("of ") || text.contains('%') || self.terms.is_empty() {
            return false;
        }

        // The terms of the group a parenthesis opened, or every term before it
        let start = self.group.take().unwrap_or(0).min(self.terms.len());

        for index in std::mem::take(&mut self.percents) {
            if index >= start && index < self.terms.len() {
                let term = std::mem::replace(&mut self.terms[index], Formula::Const(0.0));
                self.terms[index] = term.percent();
            }
        }

        let stat = self.stat(text.trim_end_matches(')'));
        if let Some(ratio) = Self::sum(self.terms.split_off(start)) {
            self.terms.push(ratio * Formula::Var(stat));
        }

        true
    }

    /// `{{as|(1 + 0.3 per 100% bonus attack speed)}}`, a multiplier that grows with a
    /// stat, written after `{{times}}`
    fn multiplier(&mut self, template: &Template) -> bool {
        let Some(text) = template.positional(0).map(|text| text.plain()) else {
            return false;
        };

        let Some(captures) = lazy_regex::regex!(
            r"^\((?<base>\d+(?:\.\d+)?) \+ (?<ratio>\d+(?:\.\d+)?) per (?<per>\d+(?:\.\d+)?)%? (?<stat>[^()]+)\)$"
        )
        .captures(text.trim()) else {
            return false;
        };

        let number = |name: &str| captures[name].parse::<f64>().unwrap_or_default();
        let stat = self.stat(&captures["stat"]);
        let grows = Formula::Var(stat) * number("ratio") / number("per");
        self.terms.push(Formula::Const(number("base")) + grows);
        true
    }

    fn finish(mut self, label: &str, section: u8, unit: String) -> EffectModel {
        let sum = self.take_sum();

        let formula = match (self.multiplied.take(), sum) {
            (Some(lhs), Some(rhs)) => lhs * rhs,
            (Some(lhs), None) => lhs,
            (None, Some(sum)) => sum,
            (None, None) => Formula::Const(0.0),
        };

        EffectModel {
            label: label.to_string(),
            section,
            unit,
            formula,
            issues: self.issues,
        }
    }

    /// `{{as|{{ap|20 to 30}}|mana}}`, a value colored by a keyword rather than a
    /// scaling, whose `{{ap}}` is returned with whether it is written in percent
    fn colored_value(template: &Template) -> Option<(&Template, bool)> {
        template.positional(1)?;

        let mut nodes = template
            .positional(0)?
            .0
            .iter()
            .filter(|node| !matches!(node, Node::Text(text) if text.trim().is_empty()));

        let Some(Node::Template(value)) = nodes.next() else {
            return None;
        };

        let rest = nodes.collect::<Vec<_>>();
        let rest_is_unit = rest
            .iter()
            .all(|node| matches!(node, Node::Text(text) if text.trim() == "%"));

        (value.name == "ap" && rest_is_unit).then_some((value, !rest.is_empty()))
    }

    /// `{{as}}` of a template in [`Self::WRAPPERS`]
    fn wrapped_scaling(template: &Template) -> Option<&Template> {
        if !Self::WRAPPERS.contains(&template.name.as_str()) {
            return None;
        }

        template.positionals().find_map(|arg| {
            arg.0.iter().find_map(|node| match node {
                Node::Template(inner) if inner.name == "as" => Some(inner),
                _ => None,
            })
        })
    }

    /// `{{ap}}` or `{{pp}}` of a template in [`Self::WRAPPERS`]
    fn wrapped(template: &Template) -> Option<&Template> {
        if !Self::WRAPPERS.contains(&template.name.as_str()) {
            return None;
        }

        template.positionals().find_map(|arg| {
            arg.0.iter().find_map(|node| match node {
                Node::Template(inner) if matches!(inner.name.as_str(), "ap" | "pp" | "pplevel") => {
                    Some(inner)
                }
                _ => None,
            })
        })
    }

    /// Adds the term of a value template: `{{ap}}`, `{{pp}}`, or one of them inside
    /// a template of [`Self::WRAPPERS`]. Others are issues
    fn value(&mut self, template: &Template) {
        match template.name.as_str() {
            "ap" => self.rank_values(template),
            "pp" | "pplevel" => {
                self.table(template);
            }
            _ => match Self::wrapped(template) {
                Some(inner) => self.value(inner),
                None => self.issues.push(template.to_string()),
            },
        }
    }

    /// Adds the values of an `{{ap}}`, by rank. Values that are not numbers are issues
    fn rank_values(&mut self, template: &Template) {
        let progression = match Progression::new(template, self.key) {
            Ok(progression) => progression,
            Err(e) => return self.issues.push(e),
        };

        let values = progression
            .values
            .iter()
            .filter_map(Value::number)
            .collect::<Vec<_>>();

        if values.len() != progression.values.len() {
            self.issues.push(template.to_string());
        }

        if !values.is_empty() {
            self.terms.push(self.by_rank(values));
        }
    }

    /// Values by rank. 3 values in a basic ability come from the ultimate's rank, as
    /// Heimerdinger's, and so do 4 values of the champions whose ultimate has 4 ranks.
    /// Otherwise, 4 linear values of a basic ability miss its 5th rank, as Aurelion Sol's
    /// Q, which continues with the same step
    fn by_rank(&self, mut values: Vec<f64>) -> Formula {
        if let [value] = values[..] {
            return Formula::Const(value);
        }

        if let (Key::Q | Key::W | Key::E, &[a, b, c, d]) = (self.key, &values[..])
            && !self.context.four_rank_ultimate
            && ((c - b) - (b - a)).abs() < 1e-9
            && ((d - c) - (b - a)).abs() < 1e-9
        {
            values.push(d + (b - a));
        }

        let rank = match (self.key, values.len()) {
            (Key::Q | Key::W | Key::E, 3) => CtxVar::RLevel,
            (_, 4) if self.context.four_rank_ultimate => CtxVar::RLevel,
            (key, _) => key.as_ctx_var(),
        };

        Formula::ByRank { rank, values }
    }

    /// Adds the table of a `{{pp}}`, returning its `key=`
    fn table(&mut self, template: &Template) -> Option<String> {
        let progression = match PassiveProgression::new(template) {
            Ok(progression) => progression,
            Err(e) => {
                self.issues.push(e);
                return None;
            }
        };

        let var = self.axis(progression.axis.as_deref());

        if var == CtxVar::Unknown {
            self.issues.push(template.to_string());
        }

        self.terms.push(Formula::Table {
            var,
            keys: progression.keys.iter().map(|key| *key as u8).collect(),
            values: progression.values,
        });

        Some(progression.key)
    }

    /// Variable of a `{{pp}}` axis: the champion level by default, or the rank of
    /// the ability it names
    fn axis(&self, axis: Option<&str>) -> CtxVar {
        let Some(axis) = axis else {
            return CtxVar::Level;
        };

        let lowercase = axis.to_lowercase();

        if lowercase.contains("level") {
            return CtxVar::Level;
        }

        self.context
            .ability_keys
            .iter()
            .find(|(name, _)| lowercase.contains("rank") && axis.contains(name.as_str()))
            .map_or(CtxVar::Unknown, |(_, key)| key.as_ctx_var())
    }

    fn scaling(&mut self, scaling: &Scaling) {
        let term = self.scaling_term(scaling);
        self.terms.push(term);
    }

    /// `ratio × stat [× per_stat] [/ per]`, where the ratio adds its own scalings, raised
    /// to the power
    fn scaling_term(&mut self, scaling: &Scaling) -> Formula {
        let ratios = scaling.ratios.iter().map(|ratio| ratio / 100.0).collect();
        let mut ratio = self.by_rank(ratios);

        for nested in &scaling.nested {
            ratio = ratio + self.scaling_term(nested);
        }

        let stat = self.stat(&scaling.stat);
        let mut term = ratio * Formula::Var(stat);

        // A ratio of the stat that grows with another one, as `1.25% AP per 100 bonus health`
        if let Some(per_stat) = &scaling.per_stat {
            let per_stat = self.stat(per_stat);
            term = term * Formula::Var(per_stat);
        }

        if let Some(per) = scaling.per {
            term = term / per;
        }

        match scaling.power {
            0 | 1 => term,
            power => Formula::Pow(Box::new(term), power),
        }
    }
}

impl ChampionModel {
    /// Champions whose ultimate has 4 ranks and is learned at level 1
    const FOUR_RANK_ULTIMATE: [&str; 5] = ["Elise", "Jayce", "Karma", "Nidalee", "Udyr"];

    /// Builds the model of every page in `abilities`, the folder of one champion.
    /// Also returns the pages that failed, as `"{path}: {error}"`
    pub fn from_dir(champion_id: &str, abilities: &Path, lookup: &Lookup) -> (Self, Vec<String>) {
        let pages = Self::pages(abilities);

        // Names of the abilities, so a `{{pp}}` can name the rank of another one
        let names = PageContext {
            lookup,
            four_rank_ultimate: false,
            ability_keys: BTreeMap::new(),
        };
        let ability_keys = pages
            .iter()
            .filter_map(|(_, src)| AbilityModel::new(src, &names).ok()??.into())
            .map(|(key, ability): (Key, AbilityModel)| (ability.name, key))
            .collect();

        let context = PageContext {
            lookup,
            four_rank_ultimate: Self::FOUR_RANK_ULTIMATE.contains(&champion_id),
            ability_keys,
        };

        let mut model = Self {
            champion_id: champion_id.to_string(),
            abilities: BTreeMap::new(),
        };
        let mut skipped = Vec::new();

        for (path, src) in &pages {
            match AbilityModel::new(src, &context) {
                Ok(Some((key, mut ability))) => {
                    ability.page = path
                        .file_stem()
                        .map(|stem| stem.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    model.abilities.entry(key).or_default().push(ability);
                }
                Ok(None) => {}
                Err(e) => skipped.push(format!("{}: {e}", path.display())),
            }
        }

        (model, skipped)
    }

    /// Source of every `*.lua` page of `abilities`, ordered by key (`P`, `Q`, `W`,
    /// `E`, `R`) and then by number, so each key lists its pages as `Q0`, `Q1`, …
    fn pages(abilities: &Path) -> Vec<(PathBuf, String)> {
        let order = |path: &PathBuf| {
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let key = "PQWER".find(stem.get(..1).unwrap_or_default()).unwrap_or(5);
            let number = stem
                .get(1..)
                .and_then(|n| n.parse::<u32>().ok())
                .unwrap_or(0);
            (key, number)
        };

        let mut paths = std::fs::read_dir(abilities)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "lua"))
            .collect::<Vec<_>>();

        paths.sort_by_key(order);

        paths
            .into_iter()
            .filter_map(|path| Some((path.clone(), std::fs::read_to_string(&path).ok()?)))
            .collect()
    }
}

/// Writes the model of every champion of the cache to `cache/wiki/champions/model.json`
pub fn parse_model() -> MayFail {
    println!("[fn] wikitext::build::parse_model");

    let root = Path::new("cache/wiki/champions");
    let lookup = Lookup::from_cache()?;

    let mut champions = BTreeMap::new();
    let mut skipped = Vec::new();

    for entry in crate::read_dir(root)? {
        let abilities = entry.path().join("abilities");

        if !abilities.is_dir() {
            continue;
        }

        let champion_id = crate::file_name(&entry)?;
        let (model, failed) = ChampionModel::from_dir(&champion_id, &abilities, &lookup);

        skipped.extend(failed);
        champions.insert(champion_id, model);
    }

    for page in &skipped {
        println!("[skipped] {page}");
    }

    let effects = champions
        .values()
        .flat_map(|champion| champion.abilities.values().flatten())
        .map(|ability| ability.effects.len())
        .sum::<usize>();

    println!(
        "[model] {} champions, {effects} effects, {} pages skipped",
        champions.len(),
        skipped.len()
    );

    crate::write(
        root.join("model.json"),
        serde_json::to_string_pretty(&champions)?,
    )
}
