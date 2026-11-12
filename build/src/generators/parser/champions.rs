use {
    crate::{
        DynError, MayFail,
        generators::{
            GeneratorExt,
            constants::{Constants, WithConstants},
            impls::champions::champion_gen_fn,
            parser::{
                GeneratorFnSig, Parser, WithMapChanges, ZERO, likely_damages, load_map_changes,
                map_changes, single_mode,
            },
            utils::RegExtractor,
        },
    },
    serde::{Deserialize, Serialize},
    serde_with::{Seq, serde_as},
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt::{Display, Write},
    },
    tutorlolv2_types::{
        AbilityId, AbilityName, Attrs, ComboElement, DamageType, DevMergeData, Key, MapMode,
        StatName, Tag,
    },
    tutorlolv2_wiki::{
        JsonRead,
        champions::{WikiChampion, WikiMapStats, abilities::WikiAbility},
        map_changes::{
            ChangeTarget,
            change::{ChangeKind, Values},
            overrides::{ChampionOverrides, SelectedEffect},
        },
        parser::{Effect, Scaling},
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
    /// Effect that each ability was selected from with [`Self::ability_nth`]
    #[serde(skip)]
    pub sources: BTreeMap<AbilityId, SelectedEffect>,
    /// Formulas of the abilities that game modes change, with the maps of each
    #[serde_as(as = "Seq<(_, _)>")]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub map_damages: BTreeMap<AbilityId, Vec<(MapMode, String)>>,
    /// Codec source of the abilities of a v2 generator, written from the same tree as
    /// their damage. The other abilities' source is written from their damage
    #[serde_as(as = "Seq<(_, _)>")]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub docs: BTreeMap<AbilityId, String>,
}

impl Parser<WikiChampion, Champion> for ChampionParser {
    const TAG: Tag = Tag::Champions;
    const FN: fn(&str) -> Option<fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>> =
        champion_generator;

    fn from_root(root: &std::path::Path) -> MayFail<Self> {
        load_map_changes(root)?;

        Ok(Self {
            data: BTreeMap::from_file(root.join("cache/wiki/champions/full.json"))?,
        })
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
                for (name, effect) in &ability.effects {
                    let tag = name.to_lowercase();

                    if (effect.formula.is_some()
                        && tag.contains("damage")
                        && !tag.contains("monster")
                        && !tag.contains("minion"))
                        && !tag.contains("non-champion")
                        || (*key == Key::P && likely_damages(&effect.inner.description))
                    {
                        groups.entry((*key, i)).or_default().push((counter, name));

                        counter += 1;
                    }
                }
            }
        }

        for ((key, i), entries) in groups {
            let args = entries
                .into_iter()
                .map(|(k, comment)| {
                    let (index, alias) = match k {
                        v @ ..9 => (v, ""),
                        v @ 9..17 => (v - 8, "Min"),
                        v @ 17..25 => (v - 16, "Max"),
                        _ => panic!("[{id}] Too many abilities found"),
                    };
                    format!("({comment:?}, _{index}{alias})")
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

/// Generator of `champion_id`: its v2 generator once it is ported, else its v1
fn champion_generator(champion_id: &str) -> Option<GeneratorFnSig<WikiChampion, Champion>> {
    crate::generators_v2::impls::champion_gen_fn_v2(champion_id)
        .or_else(|| champion_gen_fn(champion_id))
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
            sources: Default::default(),
            map_damages: Default::default(),
            docs: Default::default(),
            data,
        })
    }
}

#[allow(dead_code)]
impl Champion {
    fn modify_pattern<const N: usize>(
        key: Key,
        pattern: [(&str, AbilityName); N],
    ) -> [(&str, AbilityId); N] {
        core::array::from_fn(|i| {
            let (a, b) = pattern[i];
            let f = AbilityId::from_key_fn(key);
            (a, f(b))
        })
    }

    /// Selects the effects of the first ability of `key` by their names in the
    /// wiki, assigning each one to its [`AbilityName`]
    pub fn ability<const N: usize>(
        &mut self,
        key: Key,
        pattern: [(&str, AbilityName); N],
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

    pub fn effect_nth(&self, key: Key, nth_ability: usize, effect: &str) -> MayFail<&Effect> {
        self.effect_map(key, nth_ability)?
            .get(effect)
            .ok_or_else(|| format!("Failed to get effect for key: {key:?}[{effect:?}]").into())
    }

    pub fn effect(&self, key: Key, effect: &str) -> MayFail<&Effect> {
        self.effect_nth(key, 0, effect)
    }

    /// Text of one of the champion's wiki exceptions, which are not parsed into
    /// abilities. See [`tutorlolv2_wiki::champions::exceptions`]
    pub fn exception(&self, name: &str) -> MayFail<&str> {
        self.data
            .exceptions
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| format!("{} has no exception {name:?}", self.champion_id).into())
    }

    /// Description of an effect of the first ability of `key`
    pub fn description(&self, key: Key, effect: &str) -> MayFail<&str> {
        Ok(&self.effect(key, effect)?.inner.description)
    }

    /// Values of an effect for each rank of the first ability of `key`, read
    /// from its leveling text. The wiki lists ultimates with 5 values, and only
    /// the 1st, 3rd and 5th are ranks, the same that formulas take
    pub fn ranks(&self, key: Key, effect: &str) -> MayFail<Vec<f64>> {
        let values = self.effect(key, effect)?.inner.leveling.capture_numbers();

        match (key, values.len()) {
            (_, 0) => Err(format!("No rank values for {key:?}[{effect:?}]").into()),
            (Key::R, 5) => Ok(vec![values[0], values[2], values[4]]),
            _ => Ok(values),
        }
    }

    pub fn formula(&self, key: Key, effect: &str) -> MayFail<&str> {
        Ok(self.effect(key, effect)?.formula.as_deref().unwrap_or("0"))
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

    pub fn scaling_nth(&self, key: Key, nth_ability: usize, effect: &str) -> MayFail<&[Scaling]> {
        Ok(self
            .effect_nth(key, nth_ability, effect)?
            .scalings
            .as_slice())
    }

    pub fn scaling(&self, key: Key, effect: &str) -> MayFail<&[Scaling]> {
        self.scaling_nth(key, 0, effect)
    }

    pub fn ability_nth<const N: usize>(
        &mut self,
        nth: usize,
        key: Key,
        pattern: [(&str, AbilityName); N],
    ) -> &mut Self {
        for (effect_name, ability_id) in Self::modify_pattern(key, pattern) {
            if let Some(abilities) = self.wiki_ability(key).ok()
                && let Some(ability) = abilities.get(nth)
            {
                let mut value = Ability {
                    name: ability.name.clone(),
                    damage_type: ability.damage_type,
                    attributes: Attrs::Undefined,
                    comment: "Unknown".into(),
                    damage: ZERO.into(),
                };

                match ability.effects.get(effect_name) {
                    Some(effect) => {
                        value.comment = effect_name.to_string();

                        if let Some(formula) = &effect.formula {
                            value.damage = formula.clone();
                        }

                        self.sources.insert(
                            ability_id,
                            SelectedEffect {
                                key,
                                nth,
                                name: effect_name.to_string(),
                            },
                        );
                    }
                    None => println!(
                        "cargo:warning=[{}] {key:?}[{nth}] has no effect named {effect_name:?}",
                        self.champion_id
                    ),
                }

                self.insert(ability_id, value);
            }
        }

        self
    }

    /// Creates abilities of `key` with no damage, for abilities whose effects the
    /// wiki does not list, so their damage must be set afterwards
    pub fn ability_placeholder<const N: usize>(
        &mut self,
        key: Key,
        names: [AbilityName; N],
    ) -> &mut Self {
        let f = AbilityId::from_key_fn(key);

        for name in names {
            if let Some(ability) = self.wiki_ability(key).ok().and_then(|list| list.first()) {
                let value = Ability {
                    name: ability.name.clone(),
                    damage_type: ability.damage_type,
                    attributes: Attrs::Undefined,
                    comment: "Unknown".into(),
                    damage: ZERO.into(),
                };

                self.insert(f(name), value);
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

    pub fn indexof(&self, key: AbilityId) -> MayFail<usize> {
        Ok(self
            .abilities
            .iter()
            .position(|(k, _)| k == &key)
            .ok_or_else(|| {
                format!("self.abilities.iter().position(|(k, _)| k == &key) failed for: {key:?}")
            })?)
    }

    pub fn damage_of(&self, key: AbilityId) -> MayFail<&String> {
        self.get(key).map(|v| &v.damage)
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

    pub fn combos<const N: usize>(&mut self, combos: [&[ComboElement]; N]) -> MayFail<&mut Self> {
        for combo in combos {
            self.combo(combo)?;
        }

        Ok(self)
    }

    pub fn combo(&mut self, combo: impl AsRef<[ComboElement]>) -> MayFail<&mut Self> {
        let combo = combo.as_ref();
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
        self.apply_map_changes()?;

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

    /// Effect `name` of `key` on each mode that changes it, by the same rules as
    /// the damages, for generators that build a damage from it, as with `merge_sum`
    pub fn map_effect(&self, key: Key, name: &str) -> MayFail<Vec<(MapMode, Effect)>> {
        let selected = self.selected_effects();
        let effect = selected
            .iter()
            .find(|s| s.key == key && s.name == name)
            .ok_or_else(|| format!("[{}] {key:?} {name:?} is not selected", self.champion_id))?;

        let mut result = Vec::new();

        for changes in map_changes() {
            let overrides = ChampionOverrides::new(&self.data, changes, &selected);

            if let Some(changed) = overrides.effects.get(effect) {
                result.push((
                    single_mode(&changes.maps, &self.champion_id)?,
                    changed.clone(),
                ));
            }
        }

        Ok(result)
    }

    /// Sets the damage of `ability` on `mode`, for generators that build it
    /// themselves. Abilities with a damage on a mode aren't changed automatically
    pub fn map_damage(
        &mut self,
        mode: MapMode,
        ability: AbilityId,
        damage: impl Display,
    ) -> &mut Self {
        self.map_damages
            .entry(ability)
            .or_default()
            .push((mode, damage.to_string()));

        self
    }

    /// Damage of `into` on each mode that changes any of its `effects`, as their
    /// sum with [`Self::merge_sum`], whose result doesn't follow their changes
    pub fn map_sum<const N: usize>(
        &mut self,
        into: AbilityId,
        effects: [&str; N],
    ) -> MayFail<&mut Self> {
        let key = into.as_key();
        let mut originals = Vec::new();

        for name in effects {
            originals.push(self.formula(key, name)?.parenthesize());
        }

        let mut by_mode = BTreeMap::<MapMode, Vec<String>>::new();

        for (i, name) in effects.into_iter().enumerate() {
            for (mode, effect) in self.map_effect(key, name)? {
                let formula = effect
                    .formula
                    .ok_or(format!("{key:?} {name:?} has no formula"))?;

                by_mode.entry(mode).or_insert_with(|| originals.clone())[i] =
                    formula.parenthesize();
            }
        }

        for (mode, parts) in by_mode {
            self.map_damage(mode, into, parts.join(" + "));
        }

        Ok(self)
    }

    /// Damage of `ability` on each mode that changes its effect `name`, through
    /// `f`, the same closure that [`Self::modify`] gave it
    pub fn map_modify(
        &mut self,
        ability: AbilityId,
        name: &str,
        f: impl Fn(&str) -> String,
    ) -> MayFail<&mut Self> {
        let key = ability.as_key();

        for (mode, effect) in self.map_effect(key, name)? {
            let formula = effect
                .formula
                .ok_or(format!("{key:?} {name:?} has no formula"))?;

            self.map_damage(mode, ability, f(&formula));
        }

        Ok(self)
    }

    /// Distinct effects that the abilities were selected from
    fn selected_effects(&self) -> Vec<SelectedEffect> {
        self.sources
            .values()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Formulas of the selected effects that game modes change, for the abilities
    /// whose damage is still the formula of their effect. Abilities a generator
    /// changed are skipped, since the change may not apply to them anymore
    fn apply_map_changes(&mut self) -> MayFail {
        let selected = self.selected_effects();

        for changes in map_changes() {
            let mode = single_mode(&changes.maps, &self.champion_id)?;
            let overrides = ChampionOverrides::new(&self.data, changes, &selected);

            // Abilities whose damage on this mode the generator set itself
            let manual = self
                .map_damages
                .iter()
                .filter(|(_, damages)| damages.iter().any(|(m, _)| *m == mode))
                .map(|(id, _)| *id)
                .collect::<BTreeSet<_>>();

            for (effect, changed) in &overrides.effects {
                let original = self
                    .data
                    .wiki_abilities
                    .get(&effect.key)
                    .and_then(|list| list.get(effect.nth))
                    .and_then(|ability| ability.effects.get(&effect.name))
                    .and_then(|effect| effect.formula.as_ref());

                let (Some(original), Some(formula)) = (original, &changed.formula) else {
                    continue;
                };

                let mut applied = false;
                let mut changed = false;

                for (ability_id, source) in &self.sources {
                    let Some(ability) = self.abilities.get(ability_id) else {
                        continue;
                    };

                    if source != effect || manual.contains(ability_id) {
                        continue;
                    }

                    if ability.damage != *original {
                        changed = true;
                        println!(
                            "cargo:warning=[{}] {ability_id:?}: its generator changed the damage, so the changes of {:?} are not applied",
                            self.champion_id, changes.maps
                        );
                        continue;
                    }

                    self.map_damages
                        .entry(*ability_id)
                        .or_default()
                        .push((mode, formula.clone()));

                    applied = true;
                }

                // As when the generator merged the abilities of the effect into another,
                // unless it set the damage of an ability of the same key itself
                let handled = manual.iter().any(|id| id.as_key() == effect.key);

                if !applied && !changed && !handled {
                    println!(
                        "cargo:warning=[{}] {:?}[{}] {:?}: no ability keeps the formula of this effect, so the changes of {:?} are not applied",
                        self.champion_id, effect.key, effect.nth, effect.name, changes.maps
                    );
                }
            }
        }

        Ok(())
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
