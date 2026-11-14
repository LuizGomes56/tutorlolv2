//! One bullet of the changes of a game mode, parsed into what it changes

use crate::{
    items::item_parser::{LevelSeries, Variant, resolve_text},
    parser::Scaling,
};
use lazy_regex::{Captures, regex};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use types::StatName;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MapChange {
    /// Bullet as written in the wiki, without its `*`
    pub text: String,
    /// `**` notes written under the bullet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    pub kind: ChangeKind,
    /// The wiki marks the change with `{{bug}}`
    #[serde(default, skip_serializing_if = "Not::not")]
    pub has_bug: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChangeKind {
    /// "AP ratio changed to {{as|75% AP}}", or "Bonus attack damage changed to
    /// {{as|… AD}}". `part` is the text before "ratio" or "changed", as "AP" or
    /// "Bonus attack damage"
    Ratio {
        part: String,
        scaling: Scaling,
        /// Scaling for ranged champions, when `{{rd}}` gives them another one
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ranged: Option<Scaling>,
    },
    /// "Base damage changed to {{ap|70 to 190}}". `part` is the text before
    /// "damage", as "Base" or "Bring It Down"
    Base { part: String, amount: Amount },
    /// "Attack damage changed to 55", on an item
    ItemStat { stat: StatName, amount: Amount },
    /// "Now passively gains {{as|{{pp|0 to 187}} increased '''base''' health}}"
    BaseStat { stat: StatName, amount: Amount },
    /// "Damage reduction changed to 30%"
    Modifier { part: String, amount: Amount },
    /// "Eminence bonus attack damage per stack changed to {{rd|2|1}}", a value
    /// that only a generator can tell apart
    Named { part: String, amount: Amount },
    /// "This item is disabled."
    Disabled,
    /// "{{sbc|New Effect:}} …"
    NewEffect,
    /// "{{sbc|Removed …}} …"
    Removed,
    /// Cooldowns, costs, durations, heals and other values that no damage
    /// depends on. `keyword` is the word that matched
    Ignored { keyword: String },
    /// Didn't match any rule
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Amount {
    pub values: Values,
    /// Values for ranged champions, when `{{rd}}` gives them other ones
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranged: Option<Values>,
    /// The values are followed by `%`
    #[serde(default, skip_serializing_if = "Not::not")]
    pub percent: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Values {
    Scalar(f64),
    /// One value per rank, as `{{ap|70 to 190}}` or `{{ap|90 to 60 3}}`
    ByRank(Vec<f64>),
    /// Progression by level, as `{{pp|0 to 187}}`
    ByLevel {
        values: Vec<f64>,
        levels: Vec<u8>,
    },
}

impl MapChange {
    /// What a Ratio or Base change changes, as "Bonus damage per stack": the bullet's
    /// text before "changed", which has more than the kind's part, "Bonus" for a Base
    /// change. `None` for other kinds
    pub fn subject(&self) -> Option<&str> {
        let part = match &self.kind {
            ChangeKind::Ratio { part, .. } | ChangeKind::Base { part, .. } => part.as_str(),
            _ => return None,
        };

        let subject = self
            .text
            .find(" changed")
            .map(|end| self.text[..end].trim());

        Some(
            subject
                .filter(|subject| !subject.is_empty())
                .unwrap_or(part),
        )
    }

    /// Parses the bulleted list of one key. `is_item` reads changes of stats, as
    /// "Attack damage changed to 55", which are only stats on items
    pub fn parse_list(body: &str, is_item: bool) -> Vec<Self> {
        let mut changes = Vec::<Self>::new();

        for line in body.lines().map(str::trim).filter(|l| !l.is_empty()) {
            if let Some(note) = line.strip_prefix("**") {
                if let Some(change) = changes.last_mut() {
                    change.notes.push(note.trim().to_string());
                }
            } else if let Some(bullet) = line.strip_prefix('*') {
                changes.push(Self::parse(bullet, is_item));
            } else if let Some(change) = changes.last_mut() {
                // A bullet that continues on the next line
                change.text.push(' ');
                change.text.push_str(line);
                change.kind = ChangeKind::parse(&change.text, is_item);
            }
        }

        changes
    }

    pub fn parse(bullet: &str, is_item: bool) -> Self {
        let text = bullet.trim().to_string();

        Self {
            has_bug: text.contains("{{bug"),
            kind: ChangeKind::parse(&text, is_item),
            notes: Vec::new(),
            text,
        }
    }
}

/// The text of a change, with its templates resolved for one attack type
struct Resolved {
    plain: String,
    series: Vec<LevelSeries>,
}

impl Resolved {
    fn new(text: &str, ranged: bool) -> Self {
        let (resolved, series) = resolve_text(
            text,
            Variant {
                ranged,
                flip: false,
            },
        );

        Self {
            plain: flatten(&resolved),
            series,
        }
    }
}

impl ChangeKind {
    /// Name used in the lock file and the report
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ratio { .. } => "Ratio",
            Self::Base { .. } => "Base",
            Self::ItemStat { .. } => "ItemStat",
            Self::BaseStat { .. } => "BaseStat",
            Self::Modifier { .. } => "Modifier",
            Self::Named { .. } => "Named",
            Self::Disabled => "Disabled",
            Self::NewEffect => "NewEffect",
            Self::Removed => "Removed",
            Self::Ignored { .. } => "Ignored",
            Self::Unknown => "Unknown",
        }
    }

    /// Whether a change of this kind may change a computed damage, by the
    /// name of [`Self::name`]
    pub fn is_damage_relevant(name: &str) -> bool {
        !matches!(name, "Ignored" | "Unknown")
    }

    pub fn parse(text: &str, is_item: bool) -> Self {
        let melee = Resolved::new(text, false);
        let ranged = Resolved::new(text, true);
        let plain = melee.plain.as_str();

        if regex!(r"(?i)^this (?:item|rune|ability|summoner spell) is disabled").is_match(plain) {
            return Self::Disabled;
        }

        if text.starts_with("{{sbc|Removed") {
            return Self::Removed;
        }

        let pair = |re: &lazy_regex::Regex| -> Option<(String, Amount)> {
            let (part, value) = groups(re.captures(plain)?);
            let ranged_value = re.captures(&ranged.plain).map(|c| groups(c).1);
            let amount = Amount::parse(
                &value,
                &melee.series,
                ranged_value.as_deref(),
                &ranged.series,
            )?;

            Some((part, amount))
        };

        if let Some((stat, amount)) = pair(regex!(
            r"(?i)passively gains (?P<value>.+?) (?:increased|decreased) base (?P<part>health|armor|magic resistance|attack damage)\b"
        )) && let Some(stat) = stat_name(&stat)
        {
            return Self::BaseStat { stat, amount };
        }

        if text.starts_with("{{sbc|New") {
            return Self::NewEffect;
        }

        if is_item
            && let Some((stat, amount)) = pair(regex!(
                r"(?i)^(?P<part>attack damage|ability power|health|armor|magic resistance|attack speed|critical strike chance|lethality|mana|ability haste|armor penetration|magic penetration|omnivamp|life steal|heal and shield power) changed to (?P<value>.+?)\.?$"
            ))
            && let Some(stat) = stat_name(&stat)
        {
            return Self::ItemStat { stat, amount };
        }

        // Words of the changes that no computed damage depends on
        if let Some(keyword) = regex!(
            r"(?i)\b(cooldown|cost|duration|range|movement speed|gold|recharge|cast time|refund|lockout|radius|width|spawn\w*|vision|timer|missile speed|dash speed|restor\w*|regeneration|experience|respawn|ammunition|tenacity|slow\w*|heal(?:s|ed|ing)?|shield(?:s|ed|ing)?|mana per|more mana|stacks gained|stacks lost|minions|structures)\b"
        )
        .captures(plain)
        {
            return Self::Ignored {
                keyword: keyword[1].to_lowercase(),
            };
        }

        for re in [
            regex!(r"(?i)^(?P<part>.*?)\s*ratio changed to (?P<value>.+?)\.?$"),
            regex!(
                r"(?i)^(?P<part>.*?) changed to (?P<value>-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)*%\s*(?:of\s+)?(?:(?:the\s+)?(?:target's|their|its|your)\s+)?(?:(?:bonus|total|maximum|missing|current|base)\s+)*(?:AD|AP|attack damage|ability power|health|armor|magic resistance|mana)\b.*?)\.?$"
            ),
        ] {
            if let Some(captures) = re.captures(plain) {
                let (part, value) = groups(captures);
                let scaling = Scaling::parse(&value);
                let ranged = re
                    .captures(&ranged.plain)
                    .map(|c| Scaling::parse(&groups(c).1))
                    .filter(|other| *other != scaling);

                return Self::Ratio {
                    part,
                    scaling,
                    ranged,
                };
            }
        }

        if let Some((part, amount)) = pair(regex!(
            r"(?i)^(?P<part>.*?)\s*damage(?:\s+per\s+[\w-]+)? changed to (?P<value>.+?)\.?$"
        )) {
            return Self::Base { part, amount };
        }

        if let Some((part, amount)) = pair(regex!(
            r"(?i)^(?P<part>.*?(?:damage reduction|increased damage|reduced damage|damage dealt|damage taken).*?) changed to (?P<value>.+?)\.?$"
        )) {
            return Self::Modifier { part, amount };
        }

        if let Some((part, amount)) =
            pair(regex!(r"(?i)^(?P<part>.+?) changed to (?P<value>.+?)\.?$"))
        {
            return Self::Named { part, amount };
        }

        Self::Unknown
    }
}

impl ChangeKind {
    /// The part and values of a change that names a value, as "Eminence base
    /// bonus attack" of a Base change
    pub fn part_amount(&self) -> Option<(&str, &Amount)> {
        match self {
            Self::Base { part, amount }
            | Self::Modifier { part, amount }
            | Self::Named { part, amount } => Some((part, amount)),
            _ => None,
        }
    }
}

impl Amount {
    /// The single value for melee or ranged champions, if it is one
    pub fn single(&self, ranged: bool) -> Option<f64> {
        let values = match (ranged, &self.ranged) {
            (true, Some(ranged)) => ranged,
            _ => &self.values,
        };

        match values {
            Values::Scalar(value) => Some(*value),
            _ => None,
        }
    }

    /// Values at the start of `value`, by level when a `{{pp}}` progression starts
    /// with them, and the ranged ones when they differ
    fn parse(
        value: &str,
        series: &[LevelSeries],
        ranged_value: Option<&str>,
        ranged_series: &[LevelSeries],
    ) -> Option<Self> {
        let (values, percent) = Values::parse(value, series)?;
        let ranged = ranged_value
            .and_then(|v| Values::parse(v, ranged_series))
            .map(|(v, _)| v)
            .filter(|v| *v != values);

        Some(Self {
            values,
            ranged,
            percent,
        })
    }
}

impl Values {
    fn parse(value: &str, series: &[LevelSeries]) -> Option<(Self, bool)> {
        let value = value.trim();
        let numbers = regex!(r"^-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)*").find(value)?;
        let percent = value[numbers.end()..].trim_start().starts_with('%');
        let values = numbers
            .as_str()
            .split('/')
            .map(|n| n.trim().parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        let values = match values.as_slice() {
            [single] => match series.iter().find(|s| s.values.first() == Some(single)) {
                Some(series) => Self::ByLevel {
                    values: series.values.clone(),
                    levels: series.levels.clone(),
                },
                None => Self::Scalar(*single),
            },
            _ => Self::ByRank(values),
        };

        Some((values, percent))
    }
}

/// Named groups `part` and `value` of a match
fn groups(captures: Captures) -> (String, String) {
    let get = |name| {
        captures
            .name(name)
            .map_or("", |m| m.as_str())
            .trim()
            .to_string()
    };

    (get("part"), get("value"))
}

fn stat_name(text: &str) -> Option<StatName> {
    Some(match text.to_lowercase().as_str() {
        "attack damage" => StatName::AttackDamage,
        "ability power" => StatName::AbilityPower,
        "health" => StatName::Health,
        "armor" => StatName::Armor,
        "magic resistance" => StatName::MagicResist,
        "attack speed" => StatName::AttackSpeed,
        "critical strike chance" => StatName::CritChance,
        "lethality" => StatName::Lethality,
        "mana" => StatName::Mana,
        "ability haste" => StatName::AbilityHaste,
        "armor penetration" => StatName::ArmorPenetration,
        "magic penetration" => StatName::MagicPenetration,
        "omnivamp" => StatName::Omnivamp,
        "life steal" => StatName::LifeSteal,
        "heal and shield power" => StatName::HealAndShieldPower,
        _ => return None,
    })
}

/// Text of a change without its markup: `{{as|x}}` becomes `x`, `{{tip|a|b}}`
/// becomes `b`, links become their label, and bold and italics are removed
fn flatten(text: &str) -> String {
    let mut text = text.replace("'''", "").replace("''", "");

    loop {
        let next = regex!(r"\{\{([^{}|]+)((?:\|[^{}]*)?)\}\}")
            .replace_all(&text, |c: &Captures| {
                let args = c[2].split('|').skip(1).collect::<Vec<_>>();

                match c[1].trim().to_lowercase().as_str() {
                    "bug" => String::new(),
                    "as" | "g" | "sbc" => args.first().copied().unwrap_or_default().to_string(),
                    _ => args.last().copied().unwrap_or_default().to_string(),
                }
            })
            .into_owned();

        if next == text {
            break;
        }

        text = next;
    }

    let text = regex!(r"\[\[(?:[^\]|]*\|)?([^\]]*)\]\]").replace_all(&text, "$1");

    regex!(r"\s+").replace_all(text.trim(), " ").into_owned()
}
