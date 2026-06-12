use crate::{
    DynError, JsonRead, MayFail,
    generators::{
        GeneratorExt,
        impls::champions::champion_gen_fn,
        parser::{Parser, ZERO, likely_damages},
        utils::{RegExtractor, Tag},
    },
    model::{
        Effect, Scaling,
        champions::{WikiAbility, WikiChampion},
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};
use tutorlolv2_types::{
    AbilityId, AbilityName, Attrs, ComboElement, DamageType, DevMergeData, Key,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Champion {
    pub champion_id: String,
    pub data: WikiChampion,
    pub merge: BTreeSet<DevMergeData>,
    pub combo: Vec<Vec<ComboElement>>,
    pub abilities: BTreeMap<AbilityId, Ability>,
}

impl Parser<WikiChampion, Champion> for ChampionParser {
    const TAG: Tag = Tag::Champions;
    const FN: fn(&str) -> Option<fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>> =
        champion_gen_fn;

    fn new() -> MayFail<Self> {
        Ok(Self {
            data: BTreeMap::from_file("cache/wiki/champions/full.json")?,
        })
    }

    fn from_map(data: BTreeMap<String, WikiChampion>) -> Self {
        Self { data }
    }

    fn map(&self) -> &BTreeMap<String, WikiChampion> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool> {
        let data = &self.data[id];

        let mut groups = BTreeMap::<_, Vec<_>>::new();

        for (key, abilities) in &data.wiki_abilities {
            let mut counter = 1usize;

            for (i, ability) in abilities.iter().enumerate() {
                for (j, (name, effect)) in ability.effects.iter().enumerate() {
                    let tag = name.to_lowercase();

                    if (effect.formula.is_some()
                        && tag.contains("damage")
                        && !tag.contains("monster")
                        && !tag.contains("minion"))
                        && !tag.contains("non-champion")
                        || (*key == Key::P && likely_damages(&effect.inner.description))
                    {
                        groups
                            .entry((*key, i))
                            .or_default()
                            .push((j, counter, name));

                        counter += 1;
                    }
                }
            }
        }

        for ((key, i), entries) in groups {
            let args = entries
                .into_iter()
                .map(|(j, k, comment)| {
                    let (index, alias) = match k {
                        v @ ..9 => (v, ""),
                        v @ 9..17 => (v - 8, "Min"),
                        v @ 17..25 => (v - 16, "Max"),
                        _ => panic!("[{id}] Too many abilities found"),
                    };
                    format!("({j}, _{index}{alias}) /* {comment} */")
                })
                .collect::<Vec<_>>()
                .join(", ");

            result.push_str(".ability(");

            if i > 0 {
                result.pop();
                write!(result, "_nth({i}, ")?;
            }

            write!(result, "Key::{key:?}, [{args}])")?;
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
            data,
        })
    }
}

impl Champion {
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

    pub fn modify(&mut self, key: AbilityId, f: impl Fn(&str) -> String) -> MayFail<&mut Self> {
        let damage = self.merge_damage([key], |[k]| f(k))?;
        self.set_damage(key, damage)?;
        Ok(self)
    }

    pub fn wiki_ability(&self, key: Key) -> MayFail<&[WikiAbility]> {
        Ok(self
            .data
            .wiki_abilities
            .get(&key)
            .ok_or_else(|| format!("Failed to get wiki abilities for key: {key:?}"))?
            .as_slice())
    }

    pub fn effect_nth(&self, key: Key, nth_ability: usize, nth_effect: usize) -> MayFail<&Effect> {
        self.effect_map(key, nth_ability)?
            .values()
            .nth(nth_effect)
            .ok_or_else(|| format!("Failed to get effect for key: {key:?}[{nth_effect}]").into())
    }

    pub fn effect(&self, key: Key, nth_effect: usize) -> MayFail<&Effect> {
        self.effect_nth(key, 0, nth_effect)
    }

    pub fn formula(&self, key: Key, nth_effect: usize) -> MayFail<&str> {
        Ok(self
            .effect(key, nth_effect)?
            .formula
            .as_deref()
            .unwrap_or("0"))
    }

    pub fn effect_map(&self, key: Key, nth_ability: usize) -> MayFail<&BTreeMap<String, Effect>> {
        Ok(&self
            .wiki_ability(key)?
            .get(nth_ability)
            .ok_or_else(|| {
                format!("Failed to get wiki ability for key: {key:?} nth: {nth_ability}")
            })?
            .effects)
    }

    pub fn scaling_nth(
        &self,
        key: Key,
        nth_ability: usize,
        nth_effect: usize,
    ) -> MayFail<&[Scaling]> {
        Ok(self
            .effect_nth(key, nth_ability, nth_effect)?
            .scalings
            .as_slice())
    }

    pub fn scaling(&self, key: Key, nth_effect: usize) -> MayFail<&[Scaling]> {
        self.scaling_nth(key, 0, nth_effect)
    }

    pub fn ability_nth<const N: usize>(
        &mut self,
        nth: usize,
        key: Key,
        pattern: [(usize, AbilityName); N],
    ) -> &mut Self {
        for (i, ability_id) in Self::modify_pattern(key, pattern) {
            if let Some(abilities) = self.wiki_ability(key).ok()
                && let Some(ability) = abilities.iter().nth(nth)
            {
                let mut value = Ability {
                    name: ability.name.clone(),
                    damage_type: ability.damage_type,
                    attributes: Attrs::Undefined,
                    comment: "Unknown".into(),
                    damage: ZERO.into(),
                };

                if let Some((comment, effect)) = ability.effects.iter().nth(i) {
                    value.comment = comment.clone();

                    if let Some(formula) = &effect.formula {
                        value.damage = formula.clone();
                    }
                }

                self.insert(ability_id, value);
            }
        }

        self
    }

    pub fn insert(&mut self, key: AbilityId, ability: Ability) -> &mut Self {
        self.abilities.insert(key, ability);
        self
    }

    pub fn comment(&mut self, key: AbilityId, comment: impl ToString) -> MayFail<&mut Self> {
        self.get_mut(key)?.comment = comment.to_string();
        Ok(self)
    }

    pub fn merge<const N: usize>(
        &mut self,
        from: [AbilityId; N],
        into: AbilityId,
        sep: &str,
    ) -> MayFail<&mut Self> {
        assert!(N > 0);
        let damage = self.concat(from, sep)?;
        self.clone_to(from[0], into, damage)?;

        let comment = from
            .map(|v| match &self.get(v) {
                Ok(ability) => format!("{v:?} ({c})", c = ability.comment),
                _ => format!("{v:?} (Unknown)"),
            })
            .join(", ");

        self.comment(into, format!("Merged from: [{comment}]"))?;

        for key in from {
            self.delete(key);
        }

        Ok(self)
    }

    pub fn merge_sum<const N: usize>(
        &mut self,
        from: [AbilityId; N],
        into: AbilityId,
    ) -> MayFail<&mut Self> {
        self.merge(from, into, " + ")
    }

    pub fn merge_mul<const N: usize>(
        &mut self,
        from: [AbilityId; N],
        into: AbilityId,
    ) -> MayFail<&mut Self> {
        self.merge(from, into, " * ")
    }

    pub fn sum<const N: usize>(&self, args: [AbilityId; N]) -> MayFail<String> {
        self.concat(args, " + ")
    }

    pub fn mul<const N: usize>(&self, args: [AbilityId; N]) -> MayFail<String> {
        self.concat(args, " * ")
    }

    pub fn attr<const N: usize>(&mut self, attr: Attrs, set: [AbilityId; N]) -> MayFail<&mut Self> {
        for key in set {
            self.get_mut(key)?.attributes = attr;
        }

        Ok(self)
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

    pub fn get_mut(&mut self, key: AbilityId) -> MayFail<&mut Ability> {
        Ok(self
            .abilities
            .get_mut(&key)
            .ok_or_else(|| format!("&mut self.abilities[..] failed for: {key:?}"))?)
    }

    pub fn combo<const N: usize>(&mut self, combo: [ComboElement; N]) -> MayFail<&mut Self> {
        for &c in combo.iter() {
            if let ComboElement::Ability(id) = c
                && self.get(id).is_err()
            {
                return Err(format!("self.combo(...) failed for {c:?}",).into());
            }
        }

        self.combo.push(combo.to_vec());
        Ok(self)
    }

    pub fn delete(&mut self, key: AbilityId) -> &mut Self {
        self.abilities.remove(&key);
        self
    }

    pub fn set_damage(&mut self, key: AbilityId, damage: String) -> MayFail<&mut Self> {
        self.get_mut(key)?.damage = damage;
        Ok(self)
    }

    pub fn clone_with(
        &mut self,
        from: AbilityId,
        into: AbilityId,
        f: impl Fn(&str) -> String,
    ) -> MayFail<&mut Self> {
        let clone_from = self.get(from)?.clone();
        self.insert(into, clone_from);
        let ability = self.get_mut(into)?;
        ability.damage = f(ability.damage.as_str());
        ability.comment = format!("Modified clone of {from:?}");
        Ok(self)
    }

    pub fn clone_to(
        &mut self,
        from: AbilityId,
        into: AbilityId,
        damage: impl ToString,
    ) -> MayFail<&mut Self> {
        self.clone_with(from, into, |_| damage.to_string())
    }

    pub fn damage_types<const N: usize>(
        &mut self,
        keys: [AbilityId; N],
        damage_type: DamageType,
    ) -> MayFail<&mut Self> {
        for key in keys {
            self.damage_type(key, damage_type)?;
        }
        Ok(self)
    }

    pub fn damage_type(&mut self, key: AbilityId, damage_type: DamageType) -> MayFail<&mut Self> {
        self.get_mut(key)?.damage_type = damage_type;
        Ok(self)
    }

    pub fn concat<const N: usize>(&self, args: [AbilityId; N], sep: &str) -> MayFail<String> {
        self.merge_damage(args, |array| {
            array
                .into_iter()
                .map(RegExtractor::parenthesize)
                .collect::<Vec<_>>()
                .join(sep)
        })
    }

    pub fn merge_damage<const N: usize>(
        &self,
        args: [AbilityId; N],
        closure: impl Fn([&str; N]) -> String,
    ) -> MayFail<String> {
        assert!(N > 0);

        let mut formulas = [ZERO; _];

        let mut i = 0;
        while i < N {
            formulas[i] = self.get(args[i])?.damage.as_str();
            i += 1;
        }

        Ok(closure(formulas))
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
