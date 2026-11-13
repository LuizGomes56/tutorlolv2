use {
    crate::{
        DynError, MayFail,
        generators::{
            GeneratorExt,
            constants::{Constants, WithConstants},
            parser::{Parser, WithMapChanges, ZERO, load_map_changes, map_changes},
        },
    },
    serde::{Deserialize, Serialize},
    serde_with::{Seq, serde_as},
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt::Write,
    },
    tutorlolv2_types::{
        AbilityId, AbilityName, Attrs, ComboElement, DamageType, DevMergeData, StatName, Tag,
    },
    tutorlolv2_wiki::{
        JsonRead,
        champions::{WikiChampion, WikiMapStats},
        map_changes::{
            ChangeTarget,
            change::{ChangeKind, Values},
        },
    },
};

pub struct ChampionParser {
    pub data: BTreeMap<String, WikiChampion>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ability {
    pub name: String,
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub comment: String,
    pub damage: String,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Champion {
    pub champion_id: String,
    pub data: WikiChampion,
    pub merge: BTreeSet<DevMergeData>,
    pub combo: Vec<Vec<ComboElement>>,
    #[serde_as(as = "Seq<(_, _)>")]
    pub abilities: BTreeMap<AbilityId, Ability>,
    #[serde(default, skip_serializing_if = "Constants::is_empty")]
    pub constants: Constants,
    /// Codec source written from the same structured tree as each damage.
    #[serde_as(as = "Seq<(_, _)>")]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub docs: BTreeMap<AbilityId, String>,
}

impl Parser<WikiChampion, Champion> for ChampionParser {
    const TAG: Tag = Tag::Champions;
    const FN: fn(&str) -> Option<fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>> =
        crate::generators_v2::impls::champion_gen_fn;

    fn from_root(root: &std::path::Path) -> MayFail<Self> {
        load_map_changes(root)?;

        Ok(Self {
            data: BTreeMap::from_file(root.join("cache/wiki/champions/full.json"))?,
        })
    }

    fn map(&self) -> &BTreeMap<String, WikiChampion> {
        &self.data
    }

    fn generator_trait() -> &'static str {
        "GeneratorV2"
    }

    fn generator_declarations(modules: &str) -> String {
        format!("use crate::generators_v2::imports::*;\ncrate::decl_champions_v2!(\n{modules}\n);")
    }

    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool> {
        let models = crate::generators_v2::ChampionV2::models()?;
        let model = models
            .get(id)
            .ok_or_else(|| format!("[{id}] has no structured champion model"))?;

        for (key, pages) in &model.abilities {
            let mut index = 1;
            let mut occurrences = BTreeMap::<&str, usize>::new();

            for page in pages {
                for effect in page.effects.iter().chain(&page.extras) {
                    let nth = occurrences.entry(effect.label.as_str()).or_default();
                    let label = effect.label.to_lowercase();

                    if label.contains("damage")
                        && !label.contains("monster")
                        && !label.contains("minion")
                    {
                        let alias = match index {
                            1..=8 => format!("_{index}"),
                            9..=16 => format!("_{}Min", index - 8),
                            17..=24 => format!("_{}Max", index - 16),
                            _ => return Err(format!("[{id}] {key:?} has too many damage variants for automatic scaffolding").into()),
                        };

                        write!(
                            result,
                            ".take_nth({:?}, {}, {:?}({alias}))?",
                            effect.label, nth, key
                        )?;

                        index += 1;
                    }

                    *nth += 1;
                }
            }
        }

        Ok(true)
    }
}

impl TryFrom<WikiChampion> for Champion {
    type Error = DynError;

    fn try_from(data: WikiChampion) -> Result<Self, Self::Error> {
        Ok(Self {
            champion_id: data.champion_id.clone(),
            abilities: Default::default(),
            merge: Default::default(),
            combo: Default::default(),
            constants: Default::default(),
            docs: Default::default(),
            data,
        })
    }
}

impl Champion {
    /// Text of one of the champion's wiki exceptions, which are not parsed into
    /// abilities. See [`tutorlolv2_wiki::champions::exceptions`]
    pub fn exception(&self, name: &str) -> MayFail<&str> {
        self.data
            .exceptions
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| format!("{} has no exception {name:?}", self.champion_id).into())
    }

    pub fn indexof(&self, key: AbilityId) -> MayFail<usize> {
        Ok(self
            .abilities
            .iter()
            .position(|(k, _)| k == &key)
            .ok_or_else(|| {
                format!("self.abilities.iter().position(|(k, _)| k == &key) failed for: {key:?}")
            })?)
    }

    pub fn get(&self, key: AbilityId) -> MayFail<&Ability> {
        Ok(self
            .abilities
            .get(&key)
            .ok_or_else(|| format!("&self.abilities[..] failed for: {key:?}"))?)
    }

    pub fn nth(&self, index: usize) -> MayFail<&Ability> {
        Ok(self
            .abilities
            .values()
            .nth(index)
            .ok_or_else(|| format!("self.abilities.values().nth({index}) failed"))?)
    }

    pub fn nth_mut(&mut self, index: usize) -> MayFail<&mut Ability> {
        Ok(self
            .abilities
            .values_mut()
            .nth(index)
            .ok_or_else(|| format!("self.abilities.values_mut().nth({index}) failed"))?)
    }

    pub fn end(&mut self) -> MayFail {
        let Self {
            data: WikiChampion { champion_id, .. },
            abilities,
            merge,
            ..
        } = self;

        abilities.retain(|key, ability| {
            let zero = ability.damage == ZERO;

            if zero {
                println!("[{champion_id}] Removing [{key:?}]");
            }

            !zero
        });

        // Verifies if any ability found has unknown damage and emits a warning
        // to the console so it can be fixed by the next time the generator runs
        abilities
            .iter()
            .filter(|(_, value)| value.damage_type == DamageType::Unspecified)
            .for_each(|(key, _)| {
                println!("[{champion_id}]: Ability[{key:?}] has unknown damage type");
            });

        // Checks for minimum damage and maximum damage keys within the hashmap.
        // If it finds any key that is labeled as minimum damage, it will look
        // for keys that represent maximum damage. If it finds one, it will be
        // added to the mergevec, so it can be displayed in the tables as
        // `minimum damage - maximum damage`. If it doesn't find a maximum match,
        // a warning is emitted to the console and the key is skipped.
        for key in abilities.keys().copied() {
            let index = key.ability_name() as u8;
            let make = key.setter();

            if (AbilityName::JMP..=((AbilityName::JMP << 1) - 1)).contains(&index) {
                let mut found = false;

                let name_byte = index + AbilityName::JMP;
                let alias_byte = index - AbilityName::JMP;

                let ability_name = AbilityName::from_repr(name_byte).ok_or(format!(
                    "ability_name: AbilityName::from_repr({name_byte}) failed",
                ))?;

                let ability_id = make(ability_name);
                let name_alias = AbilityName::from_repr(alias_byte).ok_or(format!(
                    "name_alias: AbilityName::from_repr({alias_byte}) failed",
                ))?;

                let alias = make(name_alias);

                if abilities.contains_key(&ability_id) {
                    merge.insert(DevMergeData {
                        min: key,
                        max: ability_id,
                        alias,
                    });

                    found = true;
                }

                if !found {
                    println!("[{champion_id}]: Found a min key: {key:?} with no max matches");
                }
            }
        }

        // Verifies if the mergevec makes sense. It means that the generated map should
        // contain all keys that are present in the mergevec. If it doesn't, the function
        // returns a fail and prints a message to the console.
        if !merge.iter().all(|value| {
            let DevMergeData { min, max, .. } = value;

            abilities.contains_key(min) && abilities.contains_key(max)
        }) {
            println!(
                "[{champion_id}]: inconsistent data inserted into merge: {merge:?},\nkeys of abilities: {:?}",
                abilities.keys().collect::<Vec<_>>()
            );

            return Err("Found inconsistent merge vec".into());
        }

        Ok(())
    }
}

impl Champion {
    /// Stats that game modes change: the ones of `Module:ChampionData`, plus the
    /// base stats that the changes of each mode add, as Arena's "passively gains
    /// {{pp|0 to 187}} increased base health". A progression from level 1 to 18
    /// adds its first value to the base, and its growth over 17 levels per level
    pub fn map_stats(&self) -> Vec<WikiMapStats> {
        let mut list = self.data.map_stats.clone();

        for changes in map_changes() {
            for entry in changes.entries.values() {
                if entry.target
                    != (ChangeTarget::Champion {
                        champion_id: self.champion_id.clone(),
                    })
                {
                    continue;
                }

                for change in &entry.changes {
                    let ChangeKind::BaseStat { stat, amount } = &change.kind else {
                        continue;
                    };

                    let (first, last) = match &amount.values {
                        Values::ByLevel { values, .. } if values.len() > 1 => {
                            (values[0], values[values.len() - 1])
                        }
                        Values::Scalar(value) => (*value, *value),
                        _ => continue,
                    };

                    let index = match list.iter().position(|m| m.maps == changes.maps) {
                        Some(index) => index,
                        None => {
                            list.push(WikiMapStats {
                                maps: changes.maps.clone(),
                                total_attack_speed: 1.0,
                                stats: self.data.stats,
                            });
                            list.len() - 1
                        }
                    };

                    let stats = &mut list[index].stats;
                    let target = match stat {
                        StatName::Health => &mut stats.health,
                        StatName::Armor => &mut stats.armor,
                        StatName::MagicResist => &mut stats.magic_resist,
                        StatName::AttackDamage => &mut stats.attack_damage,
                        _ => continue,
                    };

                    target.base += first as f32;
                    target.per_level += (last - first) as f32 / 17.0;
                }
            }
        }

        list
    }
}

impl WithMapChanges for Champion {
    fn is_map_target(&self, target: &ChangeTarget) -> bool {
        match target {
            ChangeTarget::Champion { champion_id }
            | ChangeTarget::Ability { champion_id, .. }
            | ChangeTarget::NamedAbility { champion_id, .. } => *champion_id == self.champion_id,
            _ => false,
        }
    }
}

impl WithConstants for Champion {
    fn constants_mut(&mut self) -> &mut Constants {
        &mut self.constants
    }
}
