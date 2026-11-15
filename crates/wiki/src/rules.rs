//! Values of the game that belong to no champion, item or rune: the multipliers of
//! the elemental drakes, the stat shards, the global stats of ARAM: Mayhem, the
//! resistances of turrets and the soft caps of movement speed.

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
/// Page holding the infobox of every turret and the Bulwark passive
const TURRET_PAGE: &str = "Turret";
/// Page holding the soft caps of movement speed, under `== Movement speed caps ==`
const MOVEMENT_SPEED_PAGE: &str = "Movement_speed";

const PAGES: [&str; 6] = [
    RUNE_PAGE,
    ARAM_MAYHEM_PAGE,
    INFERNAL_MIGHT_PAGE,
    MOUNTAINOUS_VIGOR_PAGE,
    TURRET_PAGE,
    MOVEMENT_SPEED_PAGE,
];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rules {
    pub drakes: Drakes,
    pub shards: Shards,
    pub aram_mayhem: AramMayhem,
    pub turret: Turret,
    pub move_speed: MoveSpeedCaps,
}

/// Soft caps that turn the raw movement speed into the one the game shows
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MoveSpeedCaps {
    /// Raw speed above this is multiplied by `first_ratio`, up to `second_cap`
    pub first_cap: f64,
    pub first_ratio: f64,
    /// Raw speed above this is multiplied by `second_ratio`
    pub second_cap: f64,
    pub second_ratio: f64,
    /// Raw speed below this becomes `lower_floor + raw * lower_ratio`
    pub lower_cap: f64,
    pub lower_floor: f64,
    pub lower_ratio: f64,
}

/// Outer turret of Summoner's Rift, attacked by a single champion with no other
/// champion nearby
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Turret {
    /// Armor and magic resistance, which are the same
    pub resists: f64,
    /// Ability power ratio of basic attacks against turrets
    pub ap_ratio: f64,
    /// Bonus armor and magic resistance of each Bulwark stack, with one enemy
    /// champion nearby
    pub bulwark_resists_per_stack: f64,
    pub bulwark_max_stacks: u8,
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
    /// Percent of bonus movement speed, as `2.5` for 2.5%
    pub move_speed: f64,
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
        turret: Turret::from_page(&read(TURRET_PAGE)?)
            .map_err(|e| format!("[rules] {TURRET_PAGE}: {e}"))?,
        move_speed: MoveSpeedCaps::from_page(&read(MOVEMENT_SPEED_PAGE)?)
            .map_err(|e| format!("[rules] {MOVEMENT_SPEED_PAGE}: {e}"))?,
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

        let [move_speed] = capture(
            shards,
            regex!(
                r"Rune shard Movement Speed\.png[^\n]*<br />\{\{as\|\{\{fd\|([\d.]+)\}\}% '''bonus''' movement speed"
            ),
        )?;

        Ok(Self {
            adaptive_force,
            attack_speed,
            health,
            health_scaling_per_level,
            move_speed,
        })
    }
}

impl MoveSpeedCaps {
    pub fn from_page(src: &str) -> MayFail<Self> {
        let [first_cap] = capture(
            src,
            regex!(r"When the raw movement speed is greater than ([\d.]+), there are two soft caps"),
        )?;

        let [between_low, second_cap, first_percent] = capture(
            src,
            regex!(r"The raw speed between ([\d.]+) and ([\d.]+) gets multiplied by ([\d.]+)%"),
        )?;

        let [over, second_percent] = capture(
            src,
            regex!(r"The raw speed over ([\d.]+) gets multiplied by ([\d.]+)%"),
        )?;

        if between_low != first_cap || over != second_cap {
            return Err(format!(
                "Soft caps disagree: {first_cap} vs {between_low}, {second_cap} vs {over}"
            )
            .into());
        }

        let [lower_cap] = capture(
            src,
            regex!(r"When the raw movement speed is less than ([\d.]+) another"),
        )?;

        // `{{tt|110|220*(1-0.5)}}{{plus}}(Raw MS){{times}}{{fd|0.5}}`
        let [low, lower_floor, lower_ratio] = capture(
            src,
            regex!(
                r"If the raw speed is below ([\d.]+), final speed is increased to \{\{tt\|([\d.]+)\|[^}]*\}\}\{\{plus\}\}\(Raw MS\)\{\{times\}\}\{\{fd\|([\d.]+)\}\}"
            ),
        )?;

        if low != lower_cap {
            return Err(format!("Lower soft cap disagrees: {lower_cap} vs {low}").into());
        }

        Ok(Self {
            first_cap,
            first_ratio: first_percent / 100.0,
            second_cap,
            second_ratio: second_percent / 100.0,
            lower_cap,
            lower_floor,
            lower_ratio,
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

impl Turret {
    pub fn from_page(src: &str) -> MayFail<Self> {
        // The infobox of the Outer turret, up to the next one
        let start = src
            .find("|name         = Outer turret")
            .ok_or("No infobox of the Outer turret")?;

        let infobox = &src[start..];
        let infobox = &infobox[..infobox.find("{{Infobox").unwrap_or(infobox.len())];

        let [armor] = capture(infobox, regex!(r"\|armor\s*=\s*([\d.]+)\s*\n"))?;
        let [magic_resist] = capture(infobox, regex!(r"\|magicresist\s*=\s*([\d.]+)\s*\n"))?;

        if armor != magic_resist {
            return Err(format!(
                "Outer turret armor {armor} differs from magic resist {magic_resist}"
            )
            .into());
        }

        let damaging = section(src, "== Damaging turrets ==")?;
        let [ap_ratio] = capture(
            damaging,
            regex!(
                r"Unmodified basic attacks' damage against turrets is equal to [^\n]*?\(\+ ([\d.]+)% AP\)"
            ),
        )?;

        // `{{pp|30;30 to 50|0 to 5 by 1|...}}`: 30 with no enemy champion nearby,
        // then 30 to 50 from one to five. The attacker is the one enemy nearby
        let [_, one_enemy, _] = capture(
            src,
            regex!(
                r"Each stack grants the turret \{\{pp\|([\d.]+);([\d.]+) to ([\d.]+)\|0 to 5 by 1\|type=number of nearby enemy champions"
            ),
        )?;

        let [_, max_stacks] = capture(src, regex!(r"up to a maximum of \{\{pp\|([\d.]+)\*(\d+);"))?;

        Ok(Self {
            resists: armor,
            ap_ratio: ap_ratio / 100.0,
            bulwark_resists_per_stack: one_enemy,
            bulwark_max_stacks: max_stacks as u8,
        })
    }
}
