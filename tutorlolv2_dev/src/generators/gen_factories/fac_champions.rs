use crate::{
    JsonRead, JsonWrite, MayFail,
    champions::{Ability, Champion, MerakiAbility, MerakiChampion, Modifiers},
    client::{SaveTo, Tag},
    generators::{
        Generator,
        gen_decl::decl_champions::*,
        gen_utils::{F64Ext, RegExtractor},
    },
};
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    str::FromStr,
};
use tutorlolv2_fmt::rustfmt;
use tutorlolv2_gen::{
    AbilityId, AbilityName, AdaptativeType, AttackType, Attrs, ChampionId, DamageType,
    DevMergeData, Position,
};

pub struct ChampionData {
    pub data: MerakiChampion,
    pub map: BTreeMap<AbilityId, Ability>,
    pub mergevec: BTreeSet<DevMergeData>,
}

/// Struct that creates and runs files that implement the trait [`Generator`].
/// They generate intermediary representations of data that will be used by the
/// `tutorlolv2_build` to generate the `tutorlolv2_gen` library. They will read
/// the cached data from meraki analytics api and parse strings to generate the
/// final json file containing only the useful information we could extract from it
pub struct ChampionFactory;

static RE_MACRO: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)ability(\s*\[\s*([A-Za-z])\s*,(.*?)\])").expect("MACRO_RE"));

static RE_TUPLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\(\s*(\d+)\s*,\s*(\d+)\s*,").expect("TUPLE_RE"));

impl ChampionFactory {
    /// Array containing all the `::new` functions of every champion generator struct
    pub const GENERATOR_FUNCTIONS: [fn(MerakiChampion) -> Box<dyn Generator<Champion>>;
        ChampionId::VARIANTS] =
        tutorlolv2_macros::expand_dir!("../internal/champions", |[Name]| Name::new);

    /// Creates a new generator file, given a [`ChampionId`]
    pub fn create(champion_id: ChampionId) -> MayFail<String> {
        Self::create_from_raw(&format!("{champion_id:?}"))
    }

    /// Creates a new generator file in the [`GENERATOR_FOLDER`] directory. The
    /// cache file is read to generate the new file with fairly accurate offsets
    /// and good function bindings
    pub fn create_from_raw(entity_id: &str) -> MayFail<String> {
        if let Ok(data) = std::fs::read_to_string(
            SaveTo::Generator(Tag::Champions, &entity_id.to_lowercase()).path(),
        ) && (data.contains("#![stable]") || data.contains("#![preserve]"))
        {
            return Ok(data);
        }

        let bind_function = |ability_char: char, meraki_offsets: &[MerakiOffset]| -> String {
            let offsets = meraki_offsets
                .into_iter()
                .map(|meraki_offset| {
                    format!(
                        "({effect_index}, {leveling_index}, {enum_binding:?})",
                        effect_index = meraki_offset.effect,
                        leveling_index = meraki_offset.leveling,
                        enum_binding = meraki_offset.binding
                    )
                })
                .collect::<Vec<String>>();
            format!(
                "self.ability({ability_char}, [{offsets}]);",
                offsets = offsets.join(","),
            )
        };

        let mut generated_content = format!(
            "use super::*;

            impl Generator<Champion> for {entity_id} {{
                fn generate(mut self: Box<Self>) -> MayFail<Champion> {{"
        );

        let meraki_champion = MerakiChampion::from_file(format!(
            "cache/meraki/champions/{entity_id}.json"
        ))
        .map_err(|e| format!("Error calling MerakiChampion::from_file for {entity_id:?}: {e:?}"))?;
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

        ChampionId::ARRAY.into_par_iter().for_each(|champion_id| {
            let Ok(data) = Self::create(champion_id) else {
                return println!("Unable to create generator file for {champion_id:?}");
            };
            let file_name = format!("{champion_id:?}").to_lowercase();
            std::fs::write(
                SaveTo::Generator(Tag::Champions, &file_name).path(),
                data.as_bytes(),
            )
            .unwrap();
        });

        Ok(())
    }

    /// Runs all generator files. It means that several `.json` files will be created
    /// in the internal cache folder
    pub fn run_all() -> MayFail {
        ChampionId::ARRAY.into_par_iter().for_each(|champion_id| {
            if let Err(e) = Self::run(champion_id) {
                println!("Failed to run generator file for {champion_id:?}: {e:?}");
            }
        });

        Ok(())
    }

    /// Runs a generator file, given the entity name and its offset in the
    /// [`Self::GENERATOR_FUNCTIONS`] array
    pub fn run_from_raw(entity_id: &str, offset: usize) -> MayFail {
        let data = MerakiChampion::from_file(format!("cache/meraki/champions/{entity_id}.json"))?;
        let function = Self::GENERATOR_FUNCTIONS[offset];
        let generator = function(data);
        match generator.generate() {
            Ok(champion) => champion.into_file(format!("internal/champions/{entity_id}.json")),
            Err(e) => {
                println!("Error generating {entity_id:?}: {e:?}. Performing offset check.");
                match Self::check_offsets_raw(entity_id)? {
                    true => println!("{entity_id:?} have an issue unrelated to offsets"),
                    false => println!("{entity_id:?} likely has incorrect offsets"),
                }
                Ok(())
            }
        }
    }

    /// Runs a generator file, given the [`ChampionId`]
    pub fn run(champion_id: ChampionId) -> MayFail {
        Self::run_from_raw(&format!("{champion_id:?}"), champion_id as usize)
    }

    /// Receives the raw string of some generator file generates a HashMap with
    /// the extracted offsets being used in that file
    pub fn parse_offsets(src: &str) -> MayFail<HashMap<char, Vec<(usize, usize)>>> {
        let mut out = HashMap::<char, Vec<(usize, usize)>>::new();

        for caps in RE_MACRO.captures_iter(src) {
            let ability = caps[1].chars().next().ok_or("First capture has no chars")?;

            if !matches!(ability, 'Q' | 'W' | 'E' | 'R') {
                continue;
            }

            let body = &caps[2];

            let mut tuples = Vec::new();
            for t in RE_TUPLE.captures_iter(body) {
                let e = t[1].parse::<usize>()?;
                let l = t[2].parse::<usize>()?;
                tuples.push((e, l));
            }
            if !tuples.is_empty() {
                out.entry(ability).or_default().extend(tuples);
            }
        }

        Ok(out)
    }

    /// Creates a BtreeMap whose values are sorted vectors by offset tuple
    fn to_sorted_btree(
        m: &HashMap<char, Vec<(usize, usize)>>,
    ) -> BTreeMap<char, Vec<(usize, usize)>> {
        let mut norm = BTreeMap::new();
        for (&k, v) in m {
            if v.is_empty() {
                continue;
            }
            let mut vv = v.clone();
            vv.sort_unstable();
            norm.insert(k, vv);
        }
        norm
    }

    /// Prints the differences of old and new offsets to the console. It is used
    /// to warn if there might have been some changes in the current patch that could
    /// potentially cause the generator file to fail or produce invalid outputs
    fn print_diffs(
        old: &BTreeMap<char, Vec<(usize, usize)>>,
        new: &BTreeMap<char, Vec<(usize, usize)>>,
    ) {
        let keys = old
            .keys()
            .chain(new.keys())
            .copied()
            .collect::<BTreeSet<char>>();

        println!("Offsets are not equal (diff by ability):");
        for k in keys {
            let old_values = old.get(&k).cloned().unwrap_or_default();
            let new_values = new.get(&k).cloned().unwrap_or_default();
            if old_values == new_values {
                continue;
            }

            let counts = |v: &[(usize, usize)]| -> BTreeMap<(usize, usize), usize> {
                let mut c = BTreeMap::new();
                for &p in v {
                    *c.entry(p).or_insert(0) += 1;
                }
                c
            };

            let old_count = counts(&old_values);
            let new_count = counts(&new_values);

            let mut missing = Vec::new();
            let mut extra = Vec::new();

            for (p, &oq) in &old_count {
                let nq = *new_count.get(p).unwrap_or(&0);
                if nq < oq {
                    for _ in 0..(oq - nq) {
                        missing.push(*p);
                    }
                }
            }
            for (p, &nq) in &new_count {
                let oq = *old_count.get(p).unwrap_or(&0);
                if nq > oq {
                    for _ in 0..(nq - oq) {
                        extra.push(*p);
                    }
                }
            }

            #[derive(Debug)]
            #[allow(non_snake_case, unused)]
            struct OffsetCheck {
                Ability: char,
                Found: Vec<(usize, usize)>,
                Expected: Vec<(usize, usize)>,
                Missing_In_New: Vec<(usize, usize)>,
                Extra_In_New: Vec<(usize, usize)>,
            }

            println!(
                "{:#?}",
                OffsetCheck {
                    Ability: k,
                    Found: old_values,
                    Expected: new_values,
                    Missing_In_New: missing,
                    Extra_In_New: extra,
                }
            );
        }
    }

    /// Compares old and new offsets, and returns if they're absolutely equal or not.
    /// If they're not equal, likely the generator file will fail
    pub fn compare_offsets(src: &str, new: &HashMap<char, Vec<(usize, usize)>>) -> MayFail<bool> {
        let old_raw = Self::parse_offsets(src)?;
        let old = Self::to_sorted_btree(&old_raw);
        let new = Self::to_sorted_btree(new);
        if old == new {
            Ok(true)
        } else {
            Self::print_diffs(&old, &new);
            Ok(false)
        }
    }

    /// Verifies if the offsets used in the `.ability()` or `.passive()` functions
    /// inside all generators are likely to be correct, printing to the console the
    /// collected results
    pub fn check_all_offsets() {
        for champion_id in ChampionId::ARRAY {
            match Self::check_offsets(champion_id) {
                Err(e) => println!("Error checking {champion_id:?} offsets: {e:?}"),
                Ok(false) => println!("{champion_id:?} has incorrect offsets"),
                Ok(true) => println!("{champion_id:?} passed offsets check"),
            };
        }
    }

    /// Verifies the offsets used in the [`ChampionData::ability`] and [`ChampionData::passive`]
    /// methods for some [`ChampionId`]
    pub fn check_offsets(champion_id: ChampionId) -> MayFail<bool> {
        Self::check_offsets_raw(&format!("{champion_id:?}"))
    }

    /// Verifies the correctness of offsets for some champion, search by its normalized name
    pub fn check_offsets_raw(name: &str) -> MayFail<bool> {
        let meraki_champion =
            MerakiChampion::from_file(format!("cache/meraki/champions/{name}.json"))?;

        let mut new_offsets = HashMap::<char, Vec<(usize, usize)>>::new();

        for (ability_char, ability_vec) in meraki_champion.abilities.into_iter() {
            let meraki_offsets = ChampionData::get_ability_offsets(ability_vec);
            new_offsets.insert(
                ability_char,
                meraki_offsets
                    .iter()
                    .map(|o| (o.effect, o.leveling))
                    .collect(),
            );
        }

        let old_content = std::fs::read_to_string(SaveTo::Generator(Tag::Champions, name).path())?;

        if !Self::compare_offsets(&old_content, &new_offsets)? {
            return Ok(false);
        }

        Ok(true)
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
            map: BTreeMap::new(),
            mergevec: BTreeSet::new(),
        }
    }

    /// Converts the [`ChampionData`] into a [`Champion`], ending
    /// the work of the generator
    pub fn finish(self) -> Champion {
        Champion {
            name: self.data.name,
            adaptative_type: AdaptativeType::from_str(&self.data.adaptive_type).unwrap_or_default(),
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
        }
    }

    /// Returns the [`MerakiAbility`] for some [`AbilityId`] and its offset
    pub fn get_meraki_ability<'a>(
        &'a self,
        ability: AbilityId,
        ability_offset: usize,
    ) -> &'a MerakiAbility {
        let abilities = &self.data.abilities;
        let meraki_abilities = match ability {
            AbilityId::P(_) => &abilities.p,
            AbilityId::Q(_) => &abilities.q,
            AbilityId::W(_) => &abilities.w,
            AbilityId::E(_) => &abilities.e,
            AbilityId::R(_) => &abilities.r,
        };
        &meraki_abilities[ability_offset]
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
                            let enum_match = match bindings {
                                ..9 => format!("\"_{bindings}\""),
                                _ => format!("\"_{}Min\"", bindings - 8),
                            };
                            serde_json::from_str::<AbilityName>(&enum_match).unwrap()
                        },
                    };
                    fn_args.push(offset);
                    bindings += 1;
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
                            .map_or("".to_string(), |s| s.trim().to_string());
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
        ability: AbilityId,
        pattern: [(usize, usize, AbilityName); N],
    ) -> [(usize, usize, AbilityId); N] {
        let mut offsets = [(0, 0, AbilityId::P(AbilityName::Void)); N];
        let mut i = 0;
        while i < N {
            let offset = pattern[i];
            let (a, b, c) = offset;
            offsets[i] = (a, b, ability.from_ability_name(c));
            i += 1;
        }
        offsets
    }

    /// Inserts a new ability into [`Self::map`].
    pub fn insert(&mut self, key: impl Into<AbilityId>, ability: Ability) {
        self.map.insert(key.into(), ability);
    }

    /// Returns a mutable reference to some key in [`Self::map`],
    /// with custom error message
    pub fn get_mut(&mut self, key: impl Into<AbilityId>) -> MayFail<&mut Ability> {
        let field = key.into();
        Ok(self
            .map
            .get_mut(&field)
            .ok_or("[get_mut] Failed to find field: {key:?}".to_string())?)
    }

    /// Returns a reference to some key in [`Self::map`], with custom error message
    pub fn get(&self, key: impl Into<AbilityId>) -> MayFail<&Ability> {
        let field = key.into();
        Ok(self
            .map
            .get(&field)
            .ok_or(format!("[get] Failed to find field: {field:?}"))?)
    }

    /// Receives some ability key and a pattern of that helps locate where
    /// in the effects and levelings array some ability of that kind is located,
    /// and the desired name to call it through the application.
    ///
    /// ```rs
    /// self.ability(
    ///     Q,
    ///     [(0, 0, _1), (0, 1, _1Max), (2, 1, _2)]
    /// )
    /// ```
    pub fn ability<const N: usize>(
        &mut self,
        key: AbilityId,
        pattern: [(usize, usize, AbilityName); N],
    ) {
        let offsets = Self::modify_pattern(key, pattern);
        self.extract_ability_damage(key.into(), 0, &offsets);
    }

    /// Adds the attribute to all abilities in the provided array. If any ability in that
    /// array does not exist in [`Self::map`], this function will fail.
    /// If there's an ability with a different [`AbilityId`] kind, you may want to use the
    /// macro [`dynarr`]
    pub fn attr<const N: usize>(&mut self, attr: Attrs, set: [impl Into<AbilityId>; N]) -> MayFail {
        for key in set {
            self.get_mut(key.into())?.attributes = attr;
        }
        Ok(())
    }

    /// Use method [`Self::ability`] instead. It allows the usage of [`AbilityName`] values
    /// in the third value of the pattern tuples instead of a full [`AbilityId`] enum
    pub fn extract_ability_damage(
        &mut self,
        ability: AbilityId,
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

            const MIN_I: u8 = AbilityName::Min as u8;
            const MIN_J: u8 = AbilityName::_8Min as u8;
            const MAX_I: u8 = AbilityName::Max as u8;
            const MAX_J: u8 = AbilityName::_8Max as u8;

            let min_range = MIN_I..=MIN_J;
            const MAX_MATCH: u8 = 1 + MAX_J - MAX_I;

            let make = key.from_fn();

            if min_range.contains(&index) {
                let mut found = false;
                let ability_name =
                    unsafe { std::mem::transmute::<_, AbilityName>(index + MAX_MATCH) };
                let ability_id = make(ability_name);
                let name_alias =
                    unsafe { std::mem::transmute::<_, AbilityName>(index - MAX_MATCH) };
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
    pub fn clone_to(
        &mut self,
        from: impl Into<AbilityId>,
        into: impl Into<AbilityId>,
    ) -> MayFail<&mut Ability> {
        let clone_from = self.get(from.into())?.clone();
        let into_key = into.into();
        self.insert(into_key, clone_from);
        self.get_mut(into_key)
    }

    /// Associates some damage type to a key in [`Self::map`]. If that key is missing,
    /// this function will fail
    pub fn damage_type(&mut self, key: impl Into<AbilityId>, damage_type: DamageType) -> MayFail {
        self.get_mut(key.into())?.damage_type = damage_type;
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
        args: [impl Into<AbilityId> + Copy; N],
    ) -> MayFail<Vec<String>> {
        let mut sizes = Vec::<usize>::new();
        for arg in args {
            let result = self.get(arg.into())?;
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
                closure_args[j] = self.get(arg.into())?.damage[i].clone();
            }
            result.push(closure(closure_args));
        }
        Ok(result)
    }
}
