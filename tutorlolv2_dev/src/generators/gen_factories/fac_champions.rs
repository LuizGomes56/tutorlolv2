use crate::{
    ENV_CONFIG, JsonRead, JsonWrite, MayFail, Progress,
    champions::{Ability, Champion, MerakiAbility, MerakiChampion, Modifiers},
    client::{SaveTo, Tag},
    generators::{
        gen_champions::*,
        gen_utils::{F64Ext, RegExtractor},
    },
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    str::FromStr,
};
use tutorlolv2_fmt::rustfmt;
use tutorlolv2_gen::{
    AbilityId, AbilityName, AdaptiveType, AttackType, Attrs, ComboElement, DamageType,
    DevMergeData, Key, Position,
};

pub struct ChampionData {
    pub data: MerakiChampion,
    pub map: BTreeMap<AbilityId, Ability>,
    pub mergevec: BTreeSet<DevMergeData>,
    pub combo: Vec<Vec<ComboElement>>,
    progress: Progress,
    version: String,
}

/// Struct that creates and runs files that implement the trait [`Generator`].
/// They generate intermediary representations of data that will be used by the
/// `tutorlolv2_build` to generate the `tutorlolv2_gen` library. They will read
/// the cached data from meraki analytics api and parse strings to generate the
/// final json file containing only the useful information we could extract from it
pub struct ChampionFactory;

impl ChampionFactory {
    pub const GENERATOR_NAMES: &[&str] = champion_gen_names();

    pub fn progress() {
        let mut stables = 0;
        let mut preserve = 0;
        let mut unstables = 0;
        let mut total = 0;
        for name in Self::GENERATOR_NAMES {
            if let Ok(data) =
                std::fs::read_to_string(SaveTo::GeneratorRaw(Tag::Champions, name).path())
            {
                if data.contains("Stable") {
                    stables += 1;
                } else if data.contains("Preserve") {
                    preserve += 1;
                } else {
                    unstables += 1;
                }
                total += 1;
            }
        }

        println!(
            concat!(
                "ChampionFactory::progress\n",
                "{stables:>3} / {total} stable\n",
                "{preserve:>3} / {total} preserved\n",
                "{unstables:>3} / {total} unstable\n",
            ),
            stables = stables,
            preserve = preserve,
            unstables = unstables,
            total = total
        );
    }

    /// Creates a new generator file, given a [`ChampionId`]
    pub fn create(name: &str) -> MayFail<String> {
        if let Ok(data) = std::fs::read_to_string(SaveTo::GeneratorRaw(Tag::Champions, name).path())
            && (data.contains("self.progress(Stable)") || data.contains("self.progress(Preserve)"))
        {
            println!("[stable] Skipping generator for {name:?}");
            return Ok(data);
        }

        let bind_function = |ability_char: char, meraki_offsets: &[MerakiOffset]| -> String {
            let offsets = meraki_offsets
                .into_iter()
                .map(|meraki_offset| {
                    let MerakiOffset {
                        effect,
                        leveling,
                        binding,
                    } = meraki_offset;

                    format!("({effect}, {leveling}, {binding:?})")
                })
                .collect::<Vec<String>>();
            format!(
                "self.ability(Key::{ability_char}, [{offsets}]);",
                offsets = offsets.join(","),
            )
        };

        let mut generated_content = format!(
            "use super::*;

            impl Generator<Champion> for {name} {{
                fn generate(mut self: Box<Self>) -> MayFail<Champion> {{"
        );

        let meraki_champion = MerakiChampion::from_file(
            SaveTo::MerakiCache(Tag::Champions, &name).path(),
        )
        .map_err(|e| format!("Error calling MerakiChampion::from_file for {name:?}: {e:?}"))?;
        for (ability_char, ability_vec) in meraki_champion.abilities.into_iter() {
            let meraki_offsets = ChampionData::get_ability_offsets(ability_vec);
            if meraki_offsets.len() > 0 {
                generated_content.push_str(&bind_function(ability_char, &meraki_offsets));
            }
        }

        generated_content.push_str("self.end()}}");

        let formatted = rustfmt(&generated_content, None);
        Ok(match formatted.is_empty() {
            true => generated_content,
            false => formatted,
        })
    }

    /// Creates the whole folder of champion generators. Fails if an error
    /// is thrown in some iteration
    pub fn create_all() -> MayFail {
        let dir = SaveTo::GeneratorDir(Tag::Champions).path();
        if !std::fs::exists(&dir)? {
            std::fs::create_dir(dir)?;
        }

        Self::GENERATOR_NAMES.into_par_iter().for_each(|name| {
            let Ok(data) = Self::create(name) else {
                return println!("Unable to create generator file for {name:?}");
            };
            std::fs::write(
                SaveTo::GeneratorRaw(Tag::Champions, name).path(),
                data.as_bytes(),
            )
            .unwrap();
        });

        Ok(())
    }

    /// Runs all generator files. It means that several `.json` files will be created
    /// in the internal cache folder
    pub fn run_all() -> MayFail {
        Self::GENERATOR_NAMES
            .into_par_iter()
            .copied()
            .try_for_each(Self::run)
    }

    pub fn run(name: &str) -> MayFail {
        match Self::run_fn(name) {
            Ok(champion) => champion.into_file(SaveTo::InternalRaw(Tag::Champions, name).path()),
            Err(e) => Ok(println!("Error generating {name:?}: {e:?}")),
        }
    }

    pub fn run_fn(name: &str) -> MayFail<Champion> {
        let data = MerakiChampion::from_file(SaveTo::MerakiCache(Tag::Champions, &name).path())?;
        let function =
            champion_gen_fn(name).ok_or(format!("Unable to find generator function for {name}"))?;
        let generator = function(data);
        generator.generate()
    }
}

pub struct MerakiOffset {
    pub effect: usize,
    pub leveling: usize,
    pub binding: AbilityName,
}

impl ChampionData {
    /// Create empty results for the current champion, associating
    /// the struct to the new generator
    pub fn new(data: MerakiChampion) -> Self {
        Self {
            data,
            map: Default::default(),
            mergevec: Default::default(),
            combo: Default::default(),
            progress: Default::default(),
            version: ENV_CONFIG.lol_version.clone(),
        }
    }

    pub fn is_outdated(&self) -> bool {
        self.version != ENV_CONFIG.lol_version
    }

    pub const fn progress(&mut self, progress: Progress) {
        self.progress = progress;
    }

    /// Converts the [`ChampionData`] into a [`Champion`], ending
    /// the work of the generator
    pub fn finish(self) -> Champion {
        Champion {
            name: self.data.name,
            adaptive_type: AdaptiveType::from_str(&self.data.adaptive_type).unwrap_or_default(),
            attack_type: AttackType::from_str(&self.data.attack_type).unwrap_or_default(),
            positions: self
                .data
                .positions
                .into_iter()
                .map(|pos| Position::from_str(&pos).unwrap_or_default())
                .collect(),
            stats: self.data.stats,
            abilities: self.map.into_iter().collect(),
            merge_data: self.mergevec,
            combo: self.combo,
        }
    }

    /// Returns the [`MerakiAbility`] for some [`AbilityId`] and its offset
    pub fn get_meraki_ability<'a>(&'a self, key: Key, ability_offset: usize) -> &'a MerakiAbility {
        &self.data.abilities[key][ability_offset]
    }
    /// Searchs through the whole [`MerakiAbility`] struct and returns metadata
    /// about the offsets in such vector that likely contain a damaging ability
    pub fn get_ability_offsets(abilities: Vec<MerakiAbility>) -> Vec<MerakiOffset> {
        let mut fn_args = Vec::<MerakiOffset>::new();
        let mut bindings = 1;

        for ability in abilities.into_iter() {
            for (effect_index, effect) in ability.effects.into_iter().enumerate() {
                for (leveling_index, leveling) in effect.leveling.into_iter().enumerate() {
                    let attribute = leveling
                        .attribute
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase();

                    if !attribute.contains("damage") {
                        continue;
                    }

                    let offset = MerakiOffset {
                        effect: effect_index,
                        leveling: leveling_index,
                        binding: {
                            const MIN_BOUNDS: u8 = AbilityName::JMP << 1;
                            let enum_match = match bindings {
                                ..AbilityName::JMP => format!("\"_{bindings}\""),
                                AbilityName::JMP..MIN_BOUNDS => format!("\"_{}Min\"", bindings - 8),
                                _ => format!("\"_{}Max\"", bindings - MIN_BOUNDS),
                            };
                            serde_json::from_str::<AbilityName>(&enum_match).unwrap()
                        },
                    };
                    fn_args.push(offset);
                    bindings += 1u8;
                }
            }
        }

        fn_args
    }

    /// Receives a slice of [`Modifiers`] and returns a [`Vec<String>`] containing
    /// the damages of the extracted ability, for each level. An empty vector
    /// means that nothing could be extracted from such modifier
    pub fn extract_ability(modifiers: &[Modifiers]) -> Vec<String> {
        let mut result = Vec::new();
        if modifiers.is_empty() {
            return result;
        }
        let length = modifiers[0].values.len();
        for i in 0..length {
            let mut parts = Vec::new();
            for modifier in modifiers {
                if let Some(value) = modifier.values.get(i) {
                    let mut raw_unit = modifier.units[i].trim();

                    if raw_unit == "\u{00d7}" {
                        raw_unit = "";
                    }

                    let scalings = raw_unit.get_scalings();
                    let unit = raw_unit.remove_parenthesis();
                    let cleaned_string = if unit.contains('%') {
                        let parts = unit.split('%').collect::<Vec<&str>>();
                        let suffix = parts
                            .get(1)
                            .map_or(Default::default(), |s| s.trim())
                            .to_string();
                        let coef = value / 100.0;
                        if coef == 1.0 && !suffix.is_empty() {
                            suffix
                        } else if !suffix.is_empty() {
                            format!("({} * {suffix})", coef.trim())
                        } else {
                            coef.trim().to_string()
                        }
                    } else if unit.is_empty() {
                        value.trim()
                    } else {
                        format!("{}{unit}", value.trim())
                    };
                    let formatted_string = cleaned_string.replace_keys();
                    let final_string = if scalings.is_empty() {
                        formatted_string
                    } else {
                        format!("{formatted_string} + {scalings}")
                    };
                    parts.push(final_string);
                }
            }
            result.push(parts.join(" + "));
        }
        result
    }

    const fn modify_pattern<const N: usize>(
        key: Key,
        pattern: [(usize, usize, AbilityName); N],
    ) -> [(usize, usize, AbilityId); N] {
        let mut offsets: [(usize, usize, AbilityId); N] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        while i < N {
            let offset = pattern[i];
            let (a, b, c) = offset;
            offsets[i] = (
                a,
                b,
                match key {
                    Key::P => AbilityId::P(c),
                    Key::Q => AbilityId::Q(c),
                    Key::W => AbilityId::W(c),
                    Key::E => AbilityId::E(c),
                    Key::R => AbilityId::R(c),
                },
            );
            i += 1;
        }
        offsets
    }

    /// Inserts a new ability into [`Self::map`].
    pub fn insert(&mut self, key: AbilityId, ability: Ability) {
        self.map.insert(key, ability);
    }

    /// Returns a mutable reference to some key in [`Self::map`],
    /// with custom error message
    pub fn get_mut(&mut self, key: AbilityId) -> MayFail<&mut Ability> {
        Ok(self
            .map
            .get_mut(&key)
            .ok_or(format!("[get_mut] Failed to find key: {key:?}"))?)
    }

    /// Returns a reference to some key in [`Self::map`], with custom error message
    pub fn get(&self, key: AbilityId) -> MayFail<&Ability> {
        Ok(self
            .map
            .get(&key)
            .ok_or(format!("[get] Failed to find key: {key:?}"))?)
    }

    pub fn combo<const N: usize>(&mut self, combo: [ComboElement; N]) -> MayFail {
        for &c in combo.iter() {
            if let ComboElement::Ability(id) = c
                && self.get(id).is_err()
            {
                return Err(format!("[combo] Failed to find key: {id:?}").into());
            }
        }
        Ok(self.combo.push(combo.to_vec()))
    }

    /// Receives some ability key and a pattern of that helps locate where
    /// in the effects and levelings array some ability of that kind is located,
    /// and the desired name to call it through the application.
    ///
    /// ```rs
    /// self.ability(
    ///     Key::Q,
    ///     [(0, 0, _1), (0, 1, _1Max), (2, 1, _2)]
    /// )
    /// ```
    pub fn ability<const N: usize>(&mut self, key: Key, pattern: [(usize, usize, AbilityName); N]) {
        let offsets = Self::modify_pattern(key, pattern);
        self.extract_ability_damage(key, 0, &offsets);
    }

    pub fn ability_nth<const N: usize>(
        &mut self,
        key: Key,
        offset: usize,
        pattern: [(usize, usize, AbilityName); N],
    ) {
        let offsets = Self::modify_pattern(key, pattern);
        self.extract_ability_damage(key, offset, &offsets);
    }

    /// Adds the attribute to all abilities in the provided array. If any ability in that
    /// array does not exist in [`Self::map`], this function will fail.
    /// If there's an ability with a different [`AbilityId`] kind, you may want to use the
    /// macro [`dynarr`]
    pub fn attr<const N: usize>(&mut self, attr: Attrs, set: [AbilityId; N]) -> MayFail {
        for key in set {
            self.get_mut(key)?.attributes = attr;
        }
        Ok(())
    }

    /// Use method [`Self::ability`] instead. It allows the usage of [`AbilityName`] values
    /// in the third value of the pattern tuples instead of a full [`AbilityId`] enum
    pub fn extract_ability_damage(
        &mut self,
        ability: Key,
        ability_offset: usize,
        pattern: &[(usize, usize, AbilityId)],
    ) {
        let mut indexes = HashMap::new();

        for (effect_index, leveling_index, ability_like) in pattern.into_iter() {
            indexes
                .entry(*effect_index)
                .or_insert(HashMap::new())
                .insert(*leveling_index, ability_like);
        }

        for (effect_index, leveling) in indexes.into_iter() {
            for (leveling_index, keyname) in leveling.into_iter() {
                let meraki_ability = self.get_meraki_ability(ability, ability_offset);
                if let Some(effects) = meraki_ability.effects.get(effect_index) {
                    if let Some(level_entry) = effects.leveling.get(leveling_index) {
                        let modifiers = &level_entry.modifiers;
                        self.map.insert(
                            *keyname,
                            meraki_ability.format(Self::extract_ability(&modifiers)),
                        );
                    } else {
                        println!(
                            "[{champion_name}] Inexistent leveling index: {leveling_index} for ability: {ability:?}",
                            champion_name = self.data.name
                        );
                        continue;
                    }
                } else {
                    println!(
                        "[{champion_name}] Inexistent effect index: {effect_index} for ability: {ability:?}",
                        champion_name = self.data.name
                    );
                    continue;
                }
            }
        }
    }

    /// Returns the current passive details and its description in a tuple, given its offsets
    pub fn get_passive_description(
        &self,
        ability_index: usize,
        effect_index: usize,
    ) -> (&MerakiAbility, &str) {
        let passive = self
            .data
            .abilities
            .p
            .get(ability_index)
            .expect("Self::get_passive_description: ability_index is invalid.");

        (
            passive,
            &passive
                .effects
                .get(effect_index)
                .expect("Self::get_passive_description: effect_index is invalid.")
                .description,
        )
    }

    /// Finishes the generator function, performing damage type checks and creating
    /// the [`Self::mergevec`] vector which represents what abilities should be displayed
    /// in a single table cell, and printing useful warning messages to the console
    pub fn end(mut self) -> MayFail<Champion> {
        let name = &self.data.name;

        // Verifies if any ability found has unknown damage and emits a warning
        // to the console so it can be fixed by the next time the generator runs
        self.map
            .iter()
            .filter(|(_, value)| value.damage_type == DamageType::Unknown)
            .for_each(|(key, _)| {
                println!("[{name}]: Key {key:?} has unknown damage type",);
            });

        // Checks for minimum damage and maximum damage keys within the hashmap.
        // If it finds any key that is labeled as minimum damage, it will look
        // for keys that represent maximum damage. If it finds one, it will be
        // added to the mergevec, so it can be displayed in the tables as
        // `minimum damage - maximum damage`. If it doesn't find a maximum match,
        // a warning is emitted to the console and the key is skipped.
        let keys = self.map.keys().cloned().collect::<Vec<_>>();
        for key in keys {
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
                if self.map.contains_key(&ability_id) {
                    self.mergevec.insert(DevMergeData {
                        minimum_damage: key,
                        maximum_damage: ability_id,
                        alias,
                    });
                    found = true;
                }

                if !found {
                    println!("[{name}]: Found a min key: {key:?} with no max matches",);
                }
            }
        }

        // Verifies if the mergevec makes sense. It means that the generated hashmap should
        // contain all keys that are present in the mergevec. If it doesn't, the function
        // returns a fail and prints a message to the console.
        if !self.mergevec.iter().all(|value| {
            let DevMergeData {
                minimum_damage,
                maximum_damage,
                ..
            } = value;
            self.map.contains_key(minimum_damage) && self.map.contains_key(maximum_damage)
        }) {
            println!(
                "{name}: inconsistent data inserted in macro `merge!`.\nmerge_vec: {:?},\n`hashmap_keys: {:?}",
                self.mergevec,
                self.map.keys().collect::<Vec<_>>()
            );
            return Err("Found inconsistent merge vec".into());
        }

        Ok(self.finish())
    }

    /// Extracts some passive from the merakianalytics data.
    /// - `postfix` is an optional field that will add some string to the end of every
    /// single damage value, for every level at the end. This could be a multiplication
    /// or sum of some value that is constant and could not be extracted directly from
    /// the passive description
    /// - `scalings` is rarely used, it defines where the scalings such as `+ 80% AP` for
    /// example are located, in which `effect` index of that array.
    ///
    /// ```rs
    /// self.passive(Void, (0, 2), None, None);
    /// self.passive(_1, (0, 0), Some(" + 0.8 * {AttackDamage}"), None);
    /// self.passive(_4, (0, 1), Some(" + 94"), Some(1));
    /// ```
    pub fn passive(
        &mut self,
        name: AbilityName,
        offsets: (usize, usize),
        postfix: Option<String>,
        scalings: Option<usize>,
    ) {
        let (ability_index, effect_index) = offsets;
        let (passive, passive_description) =
            self.get_passive_description(ability_index, effect_index);

        let description = match scalings {
            Some(scalings) => &passive.effects[scalings].description,
            None => passive_description,
        };

        let passive_bounds = description
            .get_interval()
            .expect("Couldn't extract numeric values for passive.");

        let mut damage = str::process_linear_scalings(passive_bounds, 18, postfix);

        let scalings = description.get_scalings();
        if scalings.len() > 0 {
            damage.iter_mut().for_each(|dmg| {
                *dmg = format!("{dmg} + {scalings}");
            });
        }

        self.map.insert(AbilityId::P(name), passive.format(damage));
    }

    /// Takes in two fields and returns a mutable reference to a new cloned value, that was
    /// already inserted to [`Self::map`]. The first enum is from where it is being cloned,
    /// and the second one is the new name it will have, and will be identical.
    pub fn clone_to(&mut self, from: AbilityId, into: AbilityId) -> MayFail<&mut Ability> {
        let clone_from = self.get(from)?.clone();
        let into_key = into;
        self.insert(into_key, clone_from);
        self.get_mut(into_key)
    }

    /// Associates some damage type to a key in [`Self::map`]. If that key is missing,
    /// this function will fail
    pub fn damage_type(&mut self, key: AbilityId, damage_type: DamageType) -> MayFail {
        self.get_mut(key)?.damage_type = damage_type;
        Ok(())
    }

    /// Takes in a closure that receives as argument a constant sized array of strings,
    /// and must return a new string. It will take each damage, for each level, for one
    /// or more abilities and concatenate them into a single vector, generating a new
    /// vector of damages for some new ability
    ///
    /// ```rs
    /// let merge = |args| {
    ///     self.merge_damage(
    ///         |[q1, q2, q3]| format!("({q1}) + ({q2}) + ({q3})"),
    ///         args
    ///     )
    /// };
    /// self.merge_damage(|[r]| format!("3 * ({r})"), [R::Min])?;
    /// self.merge_damage(|[q]| format!("({q}) * {MagicMultiplier} + ({q})"), [Q::Min])?;
    /// ```
    ///
    /// Note that it can also be used to duplicate the damage values of some ability
    pub fn merge_damage<const N: usize>(
        &self,
        closure: fn([String; N]) -> String,
        args: [AbilityId; N],
    ) -> MayFail<Vec<String>> {
        let mut sizes = Vec::<usize>::new();
        for arg in args {
            let result = self.get(arg)?;
            sizes.push(result.damage.len());
        }
        assert!(!sizes.is_empty(), "Closure must take at least one argument");
        assert!(
            sizes.windows(2).all(|w| w[0] == w[1]),
            "Can't compare abilities with different sizes"
        );
        let len = sizes[0];
        let mut result = Vec::<String>::with_capacity(len);
        for i in 0..len {
            let mut closure_args = std::array::from_fn(|_| String::new());
            for (j, &arg) in args.iter().enumerate() {
                closure_args[j] = self.get(arg)?.damage[i].clone();
            }
            result.push(closure(closure_args));
        }
        Ok(result)
    }
}
