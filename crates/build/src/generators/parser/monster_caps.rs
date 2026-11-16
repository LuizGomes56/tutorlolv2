//! Damage caps against monsters, as "capped at 300 against monsters". They are read
//! from the wiki model into the champion's JSON, and the engine doesn't apply them yet
use {
    lazy_regex::regex,
    serde::{Deserialize, Serialize},
    types::Key,
    wiki::wikitext::{AbilityModel, ChampionModel, EffectModel, Formula},
};

/// One cap against monsters of an ability
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MonsterCap {
    pub key: Key,
    /// Name of the ability page, as `Precision Protocol`
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
    /// Every cap of the abilities of `model`, from effects first, then descriptions
    pub fn extract(model: &ChampionModel) -> Vec<Self> {
        let mut caps = Vec::new();

        for (&key, pages) in &model.abilities {
            for page in pages {
                caps.extend(
                    page.effects
                        .iter()
                        .filter_map(|e| Self::from_effect(key, page, e)),
                );

                for (heading, text) in &page.descriptions {
                    Self::from_description(key, page, heading, text, &mut caps);
                }
            }
        }

        caps
    }

    /// An effect labelled as a monster cap, as `Monster Damage Cap` or
    /// `Maximum Monster Damage`. Plain `Monster Damage` is not a cap
    fn from_effect(key: Key, page: &AbilityModel, effect: &EffectModel) -> Option<Self> {
        let label = effect.label.to_lowercase();

        if !label.contains("monster")
            || !regex!(r"\b(cap|capped|maximum|minimum)\b").is_match(&label)
        {
            return None;
        }

        Some(Self {
            key,
            ability: page.name.clone(),
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

    /// Caps written in a description, as "capped at 100 to 320 against monsters" or
    /// "Against monsters, the damage is capped at 40 to 200 per second". A cap whose
    /// own target names only minions, as "a minimum of 40 against minions", is not
    /// one against monsters, and neither is a cap of stacks
    fn from_description(
        key: Key,
        page: &AbilityModel,
        heading: &str,
        text: &str,
        caps: &mut Vec<Self>,
    ) {
        let keyword = regex!(
            r"(?i)(capped at|maximum(?: threshold)? of|minimum(?: damage)?(?: threshold)? of)\s*"
        );

        // Not at ";", which Yone W's thresholds by level are written with
        let value_end = regex!(r"(?i)\s*(?:against|versus)\s|,|\s+and\s|<br|\.$|$");

        for sentence in Self::sentences(text) {
            let lower = sentence.to_lowercase();

            // A sentence that only says "250% damage to monsters" has no cap for them
            if !regex!(r"(?:against|versus) [a-z\- ]*?\bmonsters?\b").is_match(&lower) {
                continue;
            }

            for found in keyword.captures_iter(sentence) {
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

                caps.push(Self {
                    key,
                    ability: page.name.clone(),
                    source: CapSource::Description(heading.to_owned()),
                    bound: match found[1].to_lowercase().contains("minimum") {
                        true => CapBound::Min,
                        false => CapBound::Max,
                    },
                    part: part.unwrap_or_else(|| CapPart::from_words(&before)),
                    monsters: CapMonsters::from_words(target.as_deref().unwrap_or(&lower)),
                    formula: Self::value_formula(page, heading, value),
                    text: format!("{}{value}", &found[0]),
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

    /// The description table of `heading` that starts and ends at numbers of `value`,
    /// as the level table of "100 to 320" or of "150 + (300-150)/17*(x-1) for 20",
    /// or the value itself when it is a single number
    fn value_formula(page: &AbilityModel, heading: &str, value: &str) -> Option<Formula> {
        if let Ok(number) = value.parse::<f64>() {
            return Some(Formula::Const(number));
        }

        let numbers = regex!(r"\d+(?:\.\d+)?")
            .find_iter(value)
            .filter_map(|number| number.as_str().parse::<f64>().ok())
            .collect::<Vec<_>>();

        let close = |a: f64, b: f64| (a - b).abs() < 1e-6;
        let holds = |x: f64| numbers.iter().any(|&n| close(n, x));

        page.extras
            .iter()
            .filter(|extra| extra.label == heading)
            .find(|extra| match Self::table_values(&extra.formula) {
                [first, .., last] => holds(*first) && holds(*last),
                _ => false,
            })
            .map(|extra| extra.formula.clone())
            .or_else(|| Self::parse_value(value))
    }

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
