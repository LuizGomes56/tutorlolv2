//! Damage caps against monsters, as "capped at 300 against monsters". They are read
//! from the wiki into the JSON of champions and items, and the engine doesn't apply
//! them yet
use {
    lazy_regex::regex,
    serde::{Deserialize, Serialize},
    types::Key,
    wiki::{
        items::item_parser::{ItemEffects, Variant, resolve_text},
        wikitext::{ChampionModel, EffectModel, Formula, Nodes, Sample},
    },
};

/// One cap against monsters of an ability or of an item's effect
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MonsterCap {
    /// Key of the ability, `None` for items
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<Key>,
    /// Name of the ability page, as `Precision Protocol`, or of the item's effect,
    /// as `Torment`
    pub ability: String,
    pub source: CapSource,
    pub bound: CapBound,
    pub part: CapPart,
    pub monsters: CapMonsters,
    /// Value of the cap, if it could be read. A description's cap takes the formula
    /// of the description table with its numbers, or its only number
    pub formula: Option<Formula>,
    /// Label of the effect, or the description's words for the cap
    pub text: String,
}

/// Where a cap was read from
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CapSource {
    /// An effect labelled as a cap, as `Capped Monster Damage`
    Effect,
    /// A description section, by its heading
    Description(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CapBound {
    /// Damage against monsters is at most the value
    Max,
    /// Damage against monsters is at least the value
    Min,
}

/// Which part of the damage the cap limits
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CapPart {
    /// The whole damage
    Damage,
    /// The part based on the target's health
    HealthRatio,
    /// A bonus damage on top of the ability's damage
    Bonus,
    /// Each tick of a damage over time
    PerTick,
    /// Each hit of an ability that hits more than once
    PerHit,
    /// Each stack consumed, as Varus W's "360/3 per stack"
    PerStack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CapMonsters {
    All,
    Epic,
    NonEpic,
}

impl MonsterCap {
    /// A value written as an expression of the level `x` and of scalings, as
    /// "150 + (300-150)/17*(x-1) for 20" or "15 (+ 100% bonus AD) (+ 50% AP)". `None`
    /// for values with ranges, as "50/4 to (50/4)*3"
    fn parse_value(value: &str) -> Option<Formula> {
        // "for 20" is how many levels the wiki lists, the expression covers them
        let value = regex!(r"\s+for\s+\d+$").replace(value, "");
        let value = regex!(r"\(\+\s*(\d+(?:\.\d+)?)(%?)\s*(bonus AD|AD|AP|per mark|per stack)\)")
            .replace_all(&value, |scaling: &lazy_regex::Captures| {
                let var = match &scaling[3] {
                    "bonus AD" => "bonus_ad",
                    "AD" => "attack_damage",
                    "AP" => "ability_power",
                    _ => "stacks",
                };
                
                let divisor = match &scaling[2] {
                    "%" => " / 100",
                    _ => "",
                };

                format!(" + {}{divisor} * ctx.{var}", &scaling[1])
            });

        let value = regex!(r"\bx\b").replace_all(&value, "ctx.level");

        Formula::parse_expression(&value).ok()
    }

    /// Every cap of the abilities of `model`, from effects first, then descriptions
    pub fn extract(model: &ChampionModel) -> Vec<Self> {
        let mut caps = Vec::new();

        for (&key, pages) in &model.abilities {
            for page in pages {
                caps.extend(
                    page.effects
                        .iter()
                        .filter_map(|e| Self::from_effect(key, &page.name, e)),
                );

                for (heading, text) in &page.descriptions {
                    let description = Description {
                        key: Some(key),
                        ability: &page.name,
                        heading,
                        extras: &page.extras,
                    };
                    
                    description.read(text, &mut caps);
                }
            }
        }

        caps
    }

    /// Every cap of the effects of an item, in their text as melee and ranged
    /// champions read it, and in both texts of `{{ft|text|alternate}}`
    pub fn from_item(effects: &ItemEffects) -> Vec<Self> {
        let mut caps = Vec::new();

        for (source, effect) in [
            ("pass", &effects.pass),
            ("pass2", &effects.pass2),
            ("pass3", &effects.pass3),
            ("pass4", &effects.pass4),
            ("act", &effects.act),
            ("consume", &effects.consume),
        ] {
            let Some(effect) = effect else { continue };
            let Some(raw) = &effect.raw_description else {
                continue;
            };
            let name = effect.name.as_deref().unwrap_or(source);

            for ranged in [false, true] {
                for flip in [false, true] {
                    // Values first, then the text the templates display, as the
                    // `monsters` of `{{tip|monsters}}`
                    let (text, _) = resolve_text(raw, Variant { ranged, flip });
                    let text = Nodes::parse(&text).plain();
                    let mut found = Vec::new();
                    let description = Description {
                        key: None,
                        ability: name,
                        heading: source,
                        extras: &[],
                    };
                    
                    description.read(&text, &mut found);

                    for cap in found {
                        if !caps.contains(&cap) {
                            caps.push(cap);
                        }
                    }
                }
            }
        }

        caps
    }

    /// An effect labelled as a monster cap, as `Monster Damage Cap` or
    /// `Maximum Monster Damage`. Plain `Monster Damage` is not a cap
    fn from_effect(key: Key, ability: &str, effect: &EffectModel) -> Option<Self> {
        let label = effect.label.to_lowercase();

        if !label.contains("monster")
            || !regex!(r"\b(cap|capped|maximum|minimum)\b").is_match(&label)
        {
            return None;
        }

        Some(Self {
            key: Some(key),
            ability: ability.to_owned(),
            source: CapSource::Effect,
            bound: match label.contains("minimum") {
                true => CapBound::Min,
                false => CapBound::Max,
            },
            part: CapPart::from_words(&label),
            monsters: CapMonsters::from_words(&label),
            formula: Some(effect.formula.clone()),
            text: effect.label.clone(),
        })
    }

    /// Values of a formula that is a table, by level or by rank
    fn table_values(formula: &Formula) -> &[f64] {
        match formula {
            Formula::Table { values, .. }
            | Formula::ByRank { values, .. }
            | Formula::NumericTable { values, .. } => values,
            _ => &[],
        }
    }
}

/// A description to read caps from, with the tables written in it
struct Description<'a> {
    key: Option<Key>,
    ability: &'a str,
    heading: &'a str,
    /// Tables of the ability's descriptions, by heading, empty for items
    extras: &'a [EffectModel],
}

impl Description<'_> {
    /// Caps written in a description, as "capped at 100 to 320 against monsters" or
    /// "Against monsters, the damage is capped at 40 to 200 per second". A cap whose
    /// own target names only minions, as "a minimum of 40 against minions", is not
    /// one against monsters, and neither is a cap of stacks
    ///
    /// A cap may continue after its target with another value, as Brand P's "capped
    /// at 50/4 per tick against non-epic monsters and 80/4 per tick against epic
    /// monsters", which has the same bound
    fn read(&self, text: &str, caps: &mut Vec<MonsterCap>) {
        let keyword = regex!(
            r"(?i)(capped at|maximum(?: threshold)? of|minimum(?: damage)?(?: threshold)? of|(?:monsters?|minions?) and)\s*"
        );

        // Not at ";", which Yone W's thresholds by level are written with
        let value_end = regex!(r"(?i)\s*(?:against|versus)\s|,|\s+and\s|<br|\.$|$");

        for sentence in Self::sentences(text) {
            let lower = sentence.to_lowercase();

            // A sentence that only says "250% damage to monsters" has no cap for them
            if !regex!(r"(?:against|versus) [a-z\- ]*?\bmonsters?\b").is_match(&lower) {
                continue;
            }

            let mut bound = CapBound::Max;
            // Whether the last match was a cap, which a continuation must follow, so
            // "40% against minions and 50% against monsters" is not one
            let mut after_cap = false;

            for found in keyword.captures_iter(sentence) {
                let continues = !regex!(r"(?i)capped|maximum|minimum").is_match(&found[1]);

                if continues && !after_cap {
                    continue;
                }

                after_cap = false;

                bound = match found[1].to_lowercase() {
                    _ if continues => bound,
                    keyword if keyword.contains("minimum") => CapBound::Min,
                    _ => CapBound::Max,
                };

                let start = found.get(0).map_or(0, |m| m.end());
                let rest = &sentence[start..];
                let end = value_end.find(rest).map_or(rest.len(), |m| m.start());
                let mut value = rest[..end].trim();

                let against = regex!(r"(?i)(?:against|versus)\s+([a-z\- ]+)");
                let after = &rest[end..];

                // The words after "against" name who the cap is for. A minimum
                // joined to the cap after it, as "a minimum of 15 and is capped at
                // 150 against minions and monsters", shares its target
                let target = regex!(r"(?i)^\s*(?:against|versus)\s")
                    .is_match(after)
                    .then(|| against.captures(after))
                    .or_else(|| {
                        regex!(r"(?i)^\s+and (?:is |has )?(?:capped|a maximum)")
                            .is_match(after)
                            .then(|| against.captures(after))
                    })
                    .flatten()
                    .map(|t| t[1].to_lowercase());

                // Without one, monsters must be named before the cap, as "Against
                // monsters, the damage is capped at …"
                let named_before = against
                    .captures_iter(&sentence[..found.get(0).map_or(0, |m| m.start())])
                    .any(|t| t[1].to_lowercase().contains("monster"));

                if target
                    .as_ref()
                    .map_or(!named_before, |t| !t.contains("monster"))
                    || value.is_empty()
                    || !value.starts_with(|c: char| c.is_ascii_digit())
                    || value.ends_with("stacks")
                {
                    continue;
                }

                // Words of the damage after its value, as Gwen P's "3 (+ 5% AP) magic
                // damage"
                if let Some(words) =
                    regex!(r"\s+(?:magic|physical|true|total)\s+damage\b").find(value)
                {
                    value = value[..words.start()].trim();
                }

                let mut part = None;

                for (unit, cap_part) in [
                    ("per tick", CapPart::PerTick),
                    ("per second", CapPart::PerTick),
                    ("per hit", CapPart::PerHit),
                    ("per stack", CapPart::PerStack),
                ] {
                    if let Some(stripped) = value.strip_suffix(unit) {
                        value = stripped.trim();
                        part = Some(cap_part);
                    }
                }

                // Words before the cap, as "the damage based on their health ratio"
                let before = sentence[..found.get(0).map_or(0, |m| m.start())].to_lowercase();

                after_cap = true;
                caps.push(MonsterCap {
                    key: self.key,
                    ability: self.ability.to_owned(),
                    source: CapSource::Description(self.heading.to_owned()),
                    bound,
                    part: part.unwrap_or_else(|| CapPart::from_words(&before)),
                    monsters: CapMonsters::from_words(target.as_deref().unwrap_or(&lower)),
                    formula: self.value_formula(value),
                    text: match continues {
                        true => value.to_owned(),
                        false => format!("{}{value}", &found[0]),
                    },
                });
            }
        }
    }

    /// Sentences of a description. A period ends one when a space or a capital
    /// follows it, as in "the duration.Hemorrhage deals", but not inside a number,
    /// as "12.5"
    fn sentences(text: &str) -> impl Iterator<Item = &str> {
        let mut start = 0;
        let bytes = text.as_bytes();

        (0..=bytes.len()).filter_map(move |i| {
            let end = i == bytes.len()
                || (bytes[i] == b'.'
                    && bytes
                        .get(i + 1)
                        .is_some_and(|next| *next == b' ' || next.is_ascii_uppercase()));

            (end && start <= i).then(|| {
                let sentence = &text[start..i];
                start = i + 1;
                sentence
            })
        })
    }

    /// The description table of the heading that starts and ends at numbers of
    /// `value`, as the level table of "100 to 320" or of "150 + (300-150)/17*(x-1)
    /// for 20", or the stacks table of "50/4 to (50/4)*3", whose ends are
    /// computed. Else the value itself, when it is one number or expression
    fn value_formula(&self, value: &str) -> Option<Formula> {
        if let Ok(number) = value.parse::<f64>() {
            return Some(Formula::Const(number));
        }

        let mut numbers = regex!(r"\d+(?:\.\d+)?")
            .find_iter(value)
            .filter_map(|number| number.as_str().parse::<f64>().ok())
            .collect::<Vec<_>>();

        let range = regex!(r"\s+for\s+\d+$").replace(value, "");
      
        numbers.extend(
            range
                .split(" to ")
                .filter_map(|end| MonsterCap::parse_value(end.trim())?.eval(&Sample::default())),
        );

        let close = |a: f64, b: f64| (a - b).abs() < 1e-6;
        let holds = |x: f64| numbers.iter().any(|&n| close(n, x));

        self.extras
            .iter()
            .filter(|extra| extra.label == self.heading)
            .find(|extra| match MonsterCap::table_values(&extra.formula) {
                [first, .., last] => holds(*first) && holds(*last),
                _ => false,
            })
            .map(|extra| extra.formula.clone())
            .or_else(|| match value.contains(" to ") {
                true => None,
                false => MonsterCap::parse_value(value),
            })
    }
}

impl CapPart {
    /// The part named by the words around a cap, the whole damage by default
    fn from_words(words: &str) -> Self {
        match words {
            _ if words.contains("per tick") || words.contains("per second") => Self::PerTick,
            _ if words.contains("per hit") => Self::PerHit,
            _ if regex!(r"health (ratio|damage)|target's (maximum |current |missing )?health")
                .is_match(words) =>
            {
                Self::HealthRatio
            }
            _ if words.contains("bonus") => Self::Bonus,
            _ => Self::Damage,
        }
    }
}

impl CapMonsters {
    fn from_words(words: &str) -> Self {
        let words = words.to_lowercase();

        match words {
            _ if words.contains("non-epic") => Self::NonEpic,
            _ if words.contains("epic") => Self::Epic,
            _ => Self::All,
        }
    }
}
