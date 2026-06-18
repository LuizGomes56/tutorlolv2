use crate::{
    JsonWrite, MayFail, client::SaveTo, parallel_read, riot::RiotCdnRune, setup::riot::RiotCdnItem,
};
use heck::ToPascalCase;
use regex::Regex;
use std::{collections::BTreeMap, sync::LazyLock};
use tutorlolv2_types::StatName;

/// Reads the cached runes json extracted from Riot's API and generates a new file containing
/// only the names of each rune, and their ids
pub fn setup_runes_names() -> MayFail {
    let result: Vec<Vec<(String, usize)>> =
        parallel_read(SaveTo::RiotRunes.path(), |_, rune: RiotCdnRune| {
            let mut runes = Vec::new();

            for slot in rune.slots.into_iter() {
                for riot_rune in slot.runes.into_iter() {
                    runes.push((riot_rune.name, riot_rune.id));
                }
            }

            Ok(runes)
        })?;

    result
        .into_iter()
        .flatten()
        .collect::<BTreeMap<String, usize>>()
        .into_file(SaveTo::InternalRuneNames.path())
}

impl RiotCdnItem {
    /// Returns the value that will be added to key `prettified_stats` for each item.
    /// Depends on Riot API `item.json` and requires manual maintainance if a new XML tag is added
    pub fn pretiffy_stats(&self) -> MayFail<BTreeMap<StatName, u16>> {
        static TAGS: [&str; 4] = ["buffedStat", "nerfedStat", "attention", "ornnBonus"];
        static RE_LINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*?)<br>").unwrap());
        static RE_TAG: LazyLock<Regex> = LazyLock::new(|| {
            let tags = TAGS.join("|");
            Regex::new(&format!(r#"<({tags})>(.*?)<\/({tags})>"#)).unwrap()
        });
        static RE_PERCENT_PREFIX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^\s*\d+\s*%?\s*").unwrap());
        static RE_TAG_STRIP: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"<\/?[^>]+(>|$)").unwrap());

        let mut result = BTreeMap::<_, _>::default();

        let lines = RE_LINE.captures_iter(&self.description).collect::<Vec<_>>();
        let mut line_index = 0usize;

        for caps in RE_TAG.captures_iter(&self.description) {
            let t = &caps[1];
            let v = caps[2].replace('%', "");
            let mut n = None;

            if line_index < lines.len() {
                let tag_strip = RE_TAG_STRIP.replace_all(&lines[line_index][1], "");
                let cleaned = tag_strip.trim();
                if !cleaned.is_empty() {
                    n = Some(cleaned.to_string());
                }

                line_index += 1;
            }

            if TAGS.contains(&t)
                && let Some(n_val) = &n
            {
                let percent_prefix = RE_PERCENT_PREFIX.replace(n_val, "");
                let j = percent_prefix.trim();

                if let Ok(num) = v.parse::<u16>()
                    && !j.is_empty()
                {
                    result.insert(j.to_pascal_case(), num);
                }
            }
        }

        let json = result
            .into_iter()
            .filter_map(|(stat, value)| {
                let variant = format!("{stat:?}")
                    .replace("CriticalStrikeChance", "CritChance")
                    .replace("CriticalStrikeDamage", "CritDamage");
                let key = serde_json::from_str(&variant).ok()?;
                Some((key, value))
            })
            .collect();

        Ok(json)
    }
}
