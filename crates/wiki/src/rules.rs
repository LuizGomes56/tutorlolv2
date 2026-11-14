//! Values of the game that belong to no champion, item or rune: the multipliers of
//! the elemental drakes, the stat shards and the global stats of ARAM: Mayhem.

use crate::{HTTP_CLIENT, JsonRead, JsonWrite, MayFail};
use lazy_regex::{Lazy, Regex, regex};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const FULL_PATH: &str = "cache/wiki/rules/full.json";

/// Page holding the stat shards, under `=== Shards ===`
const RUNE_PAGE: &str = "Rune";
/// Page holding the global stat changes of the mode
const ARAM_MAYHEM_PAGE: &str = "ARAM:_Mayhem";
/// Buff of each [`Drakes`] stack, as shown in `Dragon Slayer`
const INFERNAL_MIGHT_PAGE: &str = "Template:Buff_data_Infernal_Might";
const MOUNTAINOUS_VIGOR_PAGE: &str = "Template:Buff_data_Mountainous_Vigor";

const PAGES: [&str; 4] = [
    RUNE_PAGE,
    ARAM_MAYHEM_PAGE,
    INFERNAL_MIGHT_PAGE,
    MOUNTAINOUS_VIGOR_PAGE,
];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rules {
    pub drakes: Drakes,
    pub shards: Shards,
    pub aram_mayhem: AramMayhem,
}

/// Ratios each stack of an elemental drake grants, as `0.03` for 3%
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Drakes {
    /// Attack damage and ability power of Infernal Might
    pub infernal: f64,
    /// Armor and magic resistance of Mountainous Vigor
    pub mountain: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Shards {
    pub adaptive_force: f64,
    /// Percent of bonus attack speed, as `10.0` for 10%
    pub attack_speed: f64,
    pub health: f64,
    /// Bonus health per level of the scaling health shard
    pub health_scaling_per_level: f64,
}

/// Values that grow linearly from level 1 to level 18
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct ByLevel {
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AramMayhem {
    /// Adaptive force for every 1% critical strike chance above 100%
    pub excess_crit_adaptive_force: f64,
    pub melee_base_health: ByLevel,
    pub ranged_base_health: ByLevel,
    /// Bonus armor and magic resistance
    pub melee_bonus_resists: f64,
    pub ranged_bonus_resists: ByLevel,
}

fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/rules")
}

fn page_path(page: &str) -> PathBuf {
    cache().join(page.replace(':', "_")).with_extension("txt")
}

pub async fn download() -> MayFail {
    println!("[fn] rules::download");

    for page in PAGES {
        HTTP_CLIENT
            .fetch_raw(&page_path(page), &format!("{page}?action=raw"))
            .await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] rules::parse");

    let read = |page| crate::read_to_string(page_path(page));
    let rules = Rules {
        drakes: Drakes {
            infernal: Drakes::per_stack(&read(INFERNAL_MIGHT_PAGE)?)
                .map_err(|e| format!("[rules] {INFERNAL_MIGHT_PAGE}: {e}"))?,
            mountain: Drakes::per_stack(&read(MOUNTAINOUS_VIGOR_PAGE)?)
                .map_err(|e| format!("[rules] {MOUNTAINOUS_VIGOR_PAGE}: {e}"))?,
        },
        shards: Shards::from_page(&read(RUNE_PAGE)?)
            .map_err(|e| format!("[rules] {RUNE_PAGE}: {e}"))?,
        aram_mayhem: AramMayhem::from_page(&read(ARAM_MAYHEM_PAGE)?)
            .map_err(|e| format!("[rules] {ARAM_MAYHEM_PAGE}: {e}"))?,
    };

    rules.write_file(FULL_PATH)
}

pub fn read() -> MayFail<Rules> {
    Rules::from_file(FULL_PATH)
}

pub async fn run() -> MayFail {
    download().await?;
    parse()
}

/// Text of the section under `heading`, up to the next heading of any level
fn section<'a>(src: &'a str, heading: &str) -> MayFail<&'a str> {
    let start = src
        .find(heading)
        .ok_or_else(|| format!("No section {heading:?}"))?
        + heading.len();

    let rest = &src[start..];
    let end = rest.find("\n==").unwrap_or(rest.len());

    Ok(&rest[..end])
}

/// Numbers captured by `pattern` in `src`
fn capture<const N: usize>(src: &str, pattern: &Lazy<Regex>) -> MayFail<[f64; N]> {
    let captures = pattern
        .captures(src)
        .ok_or_else(|| format!("Pattern {:?} did not match", pattern.as_str()))?;

    let mut values = [0.0; N];

    for (i, value) in values.iter_mut().enumerate() {
        *value = captures
            .get(i + 1)
            .ok_or_else(|| format!("Pattern {:?} has no group {}", pattern.as_str(), i + 1))?
            .as_str()
            .parse()?;
    }

    Ok(values)
}

impl Drakes {
    /// Ratio of one stack, from `{{pp|key=%|...|3 to 3*4 for 4}}`
    pub fn per_stack(src: &str) -> MayFail<f64> {
        let [first, per_stack, stacks] = capture(
            src,
            regex!(r"\{\{pp\|key=%\|[^\n]*?\|([\d.]+) to ([\d.]+)\*(\d+) for \d+\}\}"),
        )?;

        if first != per_stack || stacks <= 0.0 {
            return Err(
                format!("Expected linear stacks, got {first} to {per_stack}*{stacks}").into(),
            );
        }

        Ok(per_stack / 100.0)
    }
}

impl Shards {
    pub fn from_page(src: &str) -> MayFail<Self> {
        let shards = section(src, "=== Shards ===")?;
        let [adaptive_force] = capture(
            shards,
            regex!(r"Rune shard Adaptive Force\.png[^\n]*<br />\{\{adaptive\|([\d.]+)\}\}"),
        )?;

        let [attack_speed] = capture(
            shards,
            regex!(
                r"Rune shard Attack Speed\.png[^\n]*<br />\{\{as\|([\d.]+)% '''bonus''' attack speed"
            ),
        )?;

        let [health] = capture(
            shards,
            regex!(r"Rune shard Health\.png[^\n]*<br />\{\{as\|([\d.]+) '''bonus''' health"),
        )?;

        let [health_scaling_per_level] = capture(
            shards,
            regex!(r"Rune shard Health Scaling\.png[^\n]*<br />\{\{pp\|([\d.]+) \* x for \d+"),
        )?;

        Ok(Self {
            adaptive_force,
            attack_speed,
            health,
            health_scaling_per_level,
        })
    }
}

impl AramMayhem {
    pub fn from_page(src: &str) -> MayFail<Self> {
        let stats = section(src, "=== Global stat changes ===")?;
        let [excess_crit_adaptive_force] = capture(
            stats,
            regex!(
                r"in excess of \{\{as\|100%\|critical strike chance\}\} grants \{\{adaptive\|([\d.]+)\}\}"
            ),
        )?;

        let [melee_min, melee_max, ranged_min, ranged_max] = capture(
            stats,
            regex!(
                r"\{\{rd\|pp=true\|([\d.]+) to ([\d.]+)\|([\d.]+) to ([\d.]+)\|levels=1 to 18\|label=Health"
            ),
        )?;

        let [melee_bonus_resists, ranged_min_resists, ranged_max_resists] = capture(
            stats,
            regex!(
                r"\{\{rd\|([\d.]+)\|\{\{pp\|([\d.]+) to ([\d.]+)\|1 to 18\|label=Resistances\}\}\}\}"
            ),
        )?;

        Ok(Self {
            excess_crit_adaptive_force,
            melee_base_health: ByLevel {
                min: melee_min,
                max: melee_max,
            },
            ranged_base_health: ByLevel {
                min: ranged_min,
                max: ranged_max,
            },
            melee_bonus_resists,
            ranged_bonus_resists: ByLevel {
                min: ranged_min_resists,
                max: ranged_max_resists,
            },
        })
    }
}
