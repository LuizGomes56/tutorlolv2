//! Builds the [`AbilityModel`] of an ability data page, and the [`ChampionModel`]
//! of every page of a champion. Each `{{st}}` label and value becomes an effect
//! whose formula is the sum of the terms of the value: the base `{{ap}}` or `{{pp}}`,
//! rank values written in the text, and each `{{as}}` scaling

use {
    super::{
        AbilityModel, ChampionModel, EffectModel, Formula, Lookup, Node, Nodes, PassiveProgression, Progression,
        Scaling, Template, Value, Variables,
    },
    crate::parser::assign_ctx_var,
    crate::MayFail,
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

        let Some(data) = nodes.templates().into_iter().find(|t| t.name == ABILITY_DATA) else {
            return Ok(None);
        };

        let field = |name: &str| data.named(name).map(|value| value.plain().trim().to_string());

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
            damage_type: DamageType::from_str(&Self::capitalize(&field("damagetype").unwrap_or_default()))
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
                            model.effects.push(terms.effect(label.plain().trim(), section, value));
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
}

impl<'a> Terms<'a> {
    fn new(context: &'a PageContext<'a>, key: Key) -> Self {
        Self {
            context,
            key,
            terms: Vec::new(),
            issues: Vec::new(),
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
        if let [Node::Template(first), tail @ ..] = rest
            && matches!(first.name.as_str(), "ap" | "pp" | "pplevel")
        {
            self.value(first);
            rest = tail;

            if let [Node::Text(text), ..] = rest {
                unit = text.split('(').next().unwrap_or_default().trim().to_string();
            }
        }

        let mut index = 0;

        while let Some(node) = rest.get(index) {
            index += 1;

            let Node::Template(template) = node else {
                continue;
            };

            if template.name != "as" {
                self.value(template);
                continue;
            }

            if let Some(value) = Self::colored_value(template) {
                self.value(value);
                continue;
            }

            if let Some(scaling) = Scaling::new(template, self.key) {
                self.scaling(&scaling);
                continue;
            }

            // A scaling written across this `{{as}}` and the next one
            let next = rest[index..]
                .iter()
                .position(|node| !matches!(node, Node::Text(text) if text.trim().is_empty()))
                .map(|offset| index + offset);

            if let Some(next) = next
                && let Node::Template(second) = rest[next]
                && second.name == "as"
                && let Some(scaling) = Scaling::merged(template, second, self.key)
            {
                self.scaling(&scaling);
                index = next + 1;
                continue;
            }

            self.issues.push(template.to_string());
        }

        self.finish(label, section, unit)
    }

    fn finish(self, label: &str, section: u8, unit: String) -> EffectModel {
        let formula = match self.terms.len() {
            0 => Formula::Const(0.0),
            1 => self.terms.into_iter().next().unwrap_or(Formula::Const(0.0)),
            _ => Formula::Sum(self.terms),
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
    /// scaling, whose `{{ap}}` is returned
    fn colored_value(template: &Template) -> Option<&Template> {
        template.positional(1)?;

        let mut nodes = template
            .positional(0)?
            .0
            .iter()
            .filter(|node| !matches!(node, Node::Text(text) if text.trim().is_empty()));

        let Some(Node::Template(value)) = nodes.next() else {
            return None;
        };

        let rest_is_unit = nodes.all(|node| matches!(node, Node::Text(text) if text.trim() == "%"));
        (value.name == "ap" && rest_is_unit).then_some(value)
    }

    /// Adds the term of a value template: `{{ap}}` or `{{pp}}`. Others are issues
    fn value(&mut self, template: &Template) {
        match template.name.as_str() {
            "ap" => self.rank_values(template),
            "pp" | "pplevel" => {
                self.table(template);
            }
            _ => self.issues.push(template.to_string()),
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
    /// Heimerdinger's, and so do 4 values of the champions whose ultimate has 4 ranks
    fn by_rank(&self, values: Vec<f64>) -> Formula {
        if let [value] = values[..] {
            return Formula::Const(value);
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

    /// `ratio × stat [/ per]`, where the ratio adds its own scalings, raised to the power
    fn scaling_term(&mut self, scaling: &Scaling) -> Formula {
        let ratios = scaling.ratios.iter().map(|ratio| ratio / 100.0).collect();
        let mut ratio = self.by_rank(ratios);

        for nested in &scaling.nested {
            ratio = ratio + self.scaling_term(nested);
        }

        let stat = assign_ctx_var(&scaling.stat);

        if stat == CtxVar::Unknown {
            self.issues.push(format!("Unknown stat {:?}", scaling.stat));
        }

        let mut term = ratio * Formula::Var(stat);

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
            let number = stem.get(1..).and_then(|n| n.parse::<u32>().ok()).unwrap_or(0);
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

    crate::write(root.join("model.json"), serde_json::to_string_pretty(&champions)?)
}
