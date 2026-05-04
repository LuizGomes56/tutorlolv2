use crate::{
    ENV_CONFIG, JsonRead, JsonWrite, MayFail, Progress,
    client::{SaveTo, Tag},
    gen_champions::champion_gen_fn,
    gen_utils::RegExtractor,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use tutorlolv2_fmt::rustfmt;
use tutorlolv2_types::{
    AbilityId, AbilityName, Attrs, ComboElement, DamageType, DevMergeData, Key,
};
use tutorlolv2_wiki::champions::WikiChampion;

pub struct ChampionParser {
    pub data: BTreeMap<String, WikiChampion>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DamageFormula {
    Progression(String),
    Unknown(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ability {
    pub name: String,
    pub damage_type: DamageType,
    pub attributes: Attrs,
    pub damage: DamageFormula,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Champion {
    pub data: WikiChampion,
    pub merge: BTreeSet<DevMergeData>,
    pub combo: Vec<Vec<ComboElement>>,
    pub abilities: BTreeMap<AbilityId, Ability>,
    progress: Progress,
    version: String,
}

impl ChampionParser {
    pub fn new() -> MayFail<Self> {
        let data = BTreeMap::<String, WikiChampion>::from_file("cache/wiki/champions/full.json")?;
        Ok(Self { data })
    }

    pub fn run_all(self) -> MayFail<()> {
        for champion_id in self.data.keys() {
            self.run(champion_id)?
        }
        Ok(())
    }

    pub fn progress() {
        todo!()
    }

    pub fn run_fn(&self, champion_id: &str) -> MayFail<Champion> {
        self.data
            .get(champion_id)
            .and_then(|data| {
                let function = champion_gen_fn(champion_id)?;
                Some(function(data.clone()))
            })
            .ok_or_else(|| format!("[WikiFactory::run] {champion_id} not found"))?
            .call()
    }

    pub fn run(&self, champion_id: &str) -> MayFail {
        match self.run_fn(champion_id) {
            Ok(champion) => {
                champion.into_file(SaveTo::InternalRaw(Tag::Champions, champion_id).path())
            }
            Err(e) => Ok(println!("Error generating {champion_id:?}: {e:?}")),
        }
    }

    pub fn create(&self) -> MayFail {
        for (champion_id, data) in &self.data {
            if let Ok(data) =
                std::fs::read_to_string(SaveTo::GeneratorRaw(Tag::Champions, champion_id).path())
                && (data.contains(".progress(Stable)") || data.contains(".progress(Preserve)"))
            {
                println!("[stable] Skipping generator for {champion_id:?}");
                continue;
            }

            let mut result = format!(
                "use super::*;

                impl Generator for {champion_id} {{
                    fn generate(&mut self) -> MayFail {{ self"
            );

            let mut groups = BTreeMap::<_, Vec<_>>::new();

            for (key, abilities) in &data.abilities {
                let mut alias_counter = 1;

                for (i, ability) in abilities.iter().enumerate() {
                    for (j, (name, effect)) in ability.effects.iter().enumerate() {
                        let tag = name.to_lowercase();

                        if effect.formula.is_some()
                            && tag.contains("damage")
                            && !tag.contains("monster")
                            && !tag.contains("minion")
                        {
                            groups
                                .entry((*key, i))
                                .or_default()
                                .push((j, alias_counter, name));

                            alias_counter += 1;
                        }
                    }
                }
            }

            for ((key, i), entries) in groups {
                let args = entries
                    .iter()
                    .map(|(j, k, comment)| {
                        let alias = match k {
                            ..9 => format_args!("{k}"),
                            9..18 => format_args!("{k}Min"),
                            18..27 => format_args!("{k}Max"),
                            _ => panic!("[{champion_id}] Too many abilities found"),
                        };
                        format!("({j}, _{alias}) /* {comment} */")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                result.push_str(".ability(");

                if i > 0 {
                    result.pop();
                    result.push_str(&format!("_nth({i}, "));
                }

                result.push_str(&format!("Key::{key:?}, [{args}])"));
            }

            result.push_str(".end()}}");

            let formatted = rustfmt(&result, None);
            let content = match formatted.is_empty() {
                true => result,
                false => formatted,
            };

            let path = SaveTo::GeneratorRaw(Tag::Champions, champion_id).path();

            std::fs::write(&path, content)?;

            println!("[write] {path:?}")
        }

        Ok(())
    }
}

impl Champion {
    pub fn new(data: WikiChampion) -> Self {
        Self {
            data,
            abilities: Default::default(),
            merge: Default::default(),
            combo: Default::default(),
            progress: Default::default(),
            version: ENV_CONFIG.lol_version.clone(),
        }
    }

    pub fn is_outdated(&self) -> bool {
        self.version != ENV_CONFIG.lol_version
    }

    pub const fn progress(&mut self, progress: Progress) -> &mut Self {
        self.progress = progress;
        self
    }

    fn modify_pattern<const N: usize>(
        key: Key,
        pattern: [(usize, AbilityName); N],
    ) -> [(usize, AbilityId); N] {
        core::array::from_fn(|i| {
            let (a, b) = pattern[i];
            let f = AbilityId::from_key_fn(key);
            (a, f(b))
        })
    }

    pub fn ability<const N: usize>(
        &mut self,
        key: Key,
        pattern: [(usize, AbilityName); N],
    ) -> &mut Self {
        self.ability_nth(0, key, pattern)
    }

    pub fn ability_nth<const N: usize>(
        &mut self,
        nth: usize,
        key: Key,
        pattern: [(usize, AbilityName); N],
    ) -> &mut Self {
        for (i, ability_id) in Self::modify_pattern(key, pattern) {
            if let Some(abilities) = self.data.abilities.get(&key)
                && let Some(ability) = abilities.iter().nth(nth)
                && let Some(effect) = ability.effects.values().nth(i)
            {
                let mut value = Ability {
                    name: ability.name.clone(),
                    damage_type: ability.damage_type,
                    attributes: Attrs::Undefined,
                    damage: DamageFormula::Unknown(Vec::new()),
                };

                if let Some(formula) = &effect.formula {
                    value.damage = DamageFormula::Progression(formula.clone());
                }

                self.abilities.insert(ability_id, value);
            }
        }

        self
    }

    pub fn attr<const N: usize>(&mut self, attr: Attrs, set: [AbilityId; N]) -> MayFail<&mut Self> {
        for key in set {
            self.get_mut(key)?.attributes = attr;
        }

        Ok(self)
    }

    pub fn get(&self, key: AbilityId) -> MayFail<&Ability> {
        Ok(self.abilities.get(&key).ok_or_else(|| {
            format!(
                "[{champion_id}] &self.abilities[..] failed for: {key:?}",
                champion_id = self.data.champion_id
            )
        })?)
    }

    pub fn get_mut(&mut self, key: AbilityId) -> MayFail<&mut Ability> {
        Ok(self.abilities.get_mut(&key).ok_or_else(|| {
            format!(
                "[{champion_id}] &mut self.abilities[..] failed for: {key:?}",
                champion_id = self.data.champion_id
            )
        })?)
    }

    pub fn combo<const N: usize>(&mut self, combo: [ComboElement; N]) -> MayFail<&mut Self> {
        for &c in combo.iter() {
            if let ComboElement::Ability(id) = c
                && self.get(id).is_err()
            {
                return Err(format!(
                    "[{champion_id}] self.combo(...) failed for {c:?}",
                    champion_id = self.data.champion_id
                )
                .into());
            }
        }

        self.combo.push(combo.to_vec());
        Ok(self)
    }

    pub fn end(&mut self) -> MayFail {
        let Self {
            data: WikiChampion { champion_id, .. },
            abilities,
            merge,
            ..
        } = self;

        // Verifies if any ability found has unknown damage and emits a warning
        // to the console so it can be fixed by the next time the generator runs
        abilities
            .iter()
            .filter(|(_, value)| value.damage_type == DamageType::Unknown)
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

                let ability_name = AbilityName::from_u8(name_byte).ok_or(format!(
                    "ability_name: AbilityName::from_u8({name_byte}) failed",
                ))?;

                let ability_id = make(ability_name);
                let name_alias = AbilityName::from_u8(alias_byte).ok_or(format!(
                    "name_alias: AbilityName::from_u8({alias_byte}) failed",
                ))?;

                let alias = make(name_alias);
                if abilities.contains_key(&ability_id) {
                    merge.insert(DevMergeData {
                        minimum_damage: key,
                        maximum_damage: ability_id,
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
            let DevMergeData {
                minimum_damage,
                maximum_damage,
                ..
            } = value;
            abilities.contains_key(minimum_damage) && abilities.contains_key(maximum_damage)
        }) {
            println!(
                "[{champion_id}]: inconsistent data inserted into merge: {merge:?},\n`keys of abilities: {:?}",
                abilities.keys().collect::<Vec<_>>()
            );
            return Err("Found inconsistent merge vec".into());
        }

        Ok(())
    }

    pub fn clone_to(&mut self, from: AbilityId, into: AbilityId) -> MayFail<&mut Ability> {
        let clone_from = self.get(from)?.clone();
        let into_key = into;
        self.abilities.insert(into_key, clone_from);
        self.get_mut(into_key)
    }

    pub fn damage_type(&mut self, key: AbilityId, damage_type: DamageType) -> MayFail<&mut Self> {
        self.get_mut(key)?.damage_type = damage_type;
        Ok(self)
    }

    pub fn sum<const N: usize>(&self, args: [AbilityId; N]) -> MayFail<DamageFormula> {
        self.merge_damage(
            |array| {
                array
                    .iter()
                    .map(RegExtractor::parens)
                    .collect::<Vec<_>>()
                    .join(" + ")
            },
            args,
        )
    }

    pub fn merge_damage<const N: usize>(
        &self,
        closure: fn([&str; N]) -> String,
        args: [AbilityId; N],
    ) -> MayFail<DamageFormula> {
        let mut formulas = Vec::with_capacity(N);

        for arg in args {
            formulas.push(&self.get(arg)?.damage);
        }

        assert!(
            !formulas.is_empty(),
            "Closure must take at least one argument"
        );

        let mut len = None;

        for f in &formulas {
            if let DamageFormula::Unknown(v) = f {
                match len {
                    Some(l) => assert!(l == v.len(), "Mismatched lengths"),
                    None => len = Some(v.len()),
                }
            }
        }

        match len {
            Some(len) => {
                let mut result = Vec::with_capacity(len);

                for i in 0..len {
                    let closure_args = core::array::from_fn(|j| match formulas[j] {
                        DamageFormula::Progression(s) => s.as_str(),
                        DamageFormula::Unknown(v) => v[i].as_str(),
                    });

                    result.push(closure(closure_args));
                }

                Ok(DamageFormula::Unknown(result))
            }
            None => {
                let closure_args = core::array::from_fn(|i| match formulas[i] {
                    DamageFormula::Progression(s) => s.as_str(),
                    DamageFormula::Unknown(_) => unreachable!(),
                });

                Ok(DamageFormula::Progression(closure(closure_args)))
            }
        }
    }
}
