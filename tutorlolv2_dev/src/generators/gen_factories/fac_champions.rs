use crate::{
    JsonRead, JsonWrite, MayFail,
    champions::{Ability, Champion, MerakiAbility, MerakiChampion, Modifiers},
    generators::{
        Generator,
        gen_decl::decl_champions::*,
        gen_utils::{F64Ext, RegExtractor},
    },
};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::Path,
};
use tutorlolv2_fmt::invoke_rustfmt;
use tutorlolv2_gen::{Attrs, CHAMPION_CACHE, ChampionId, DamageType, Position};
use tutorlolv2_types::{AbilityLike, AbilityName};

const GENERATOR_FOLDER: &str = "tutorlolv2_dev/src/generators/gen_champions";

pub struct ChampionData {
    pub data: MerakiChampion,
    pub hashmap: HashMap<AbilityLike, Ability>,
    pub mergevec: Vec<(AbilityLike, AbilityLike)>,
}

pub struct ChampionFactory;

impl ChampionFactory {
    pub const NUMBER_OF_CHAMPIONS: usize = CHAMPION_CACHE.len();
    pub const GENERATOR_FUNCTIONS: [fn(MerakiChampion) -> Box<dyn Generator<Champion>>;
        Self::NUMBER_OF_CHAMPIONS] =
        tutorlolv2_macros::expand_dir!("../internal/champions", |[Name]| Name::new);

    pub fn create(champion_id: ChampionId) -> MayFail<String> {
        let file_name = format!("{champion_id:?}").to_lowercase();
        let path = format!("{GENERATOR_FOLDER}/{file_name}.rs");

        let bind_macro = |ability_char: char, meraki_offsets: &[MerakiOffset]| -> String {
            let mut offsets = Vec::new();
            for meraki_offset in meraki_offsets {
                offsets.push(format!(
                    "({effect_index}, {leveling_index}, {enum_binding:?})",
                    effect_index = meraki_offset.effect,
                    leveling_index = meraki_offset.leveling,
                    enum_binding = meraki_offset.binding
                ));
            }
            format!(
                "self.ability({ability_char}, [{offsets}]);",
                offsets = offsets.join(","),
            )
        };

        let mut generated_content = format!(
            "use super::*;

            impl Generator<Champion> for {champion_id:?} {{
                fn generate(mut self: Box<Self>) -> MayFail<Champion> {{"
        );

        if let Ok(data) = std::fs::read_to_string(&path) {
            if data.contains("#![stable]") {
                return Ok(data);
            }
        }

        let meraki_champion =
            MerakiChampion::from_file(format!("cache/meraki/champions/{champion_id:?}.json"))?;
        for (ability_char, ability_vec) in meraki_champion.abilities.into_iter() {
            let meraki_offsets = ChampionData::get_ability_offsets(ability_vec);
            if meraki_offsets.len() > 0 {
                generated_content.push_str(&bind_macro(ability_char, &meraki_offsets));
            }
        }

        generated_content.push_str("self.end()}}");
        Ok(invoke_rustfmt(&generated_content, 80))
    }

    pub fn create_all() -> MayFail {
        if !Path::new(GENERATOR_FOLDER).exists() {
            std::fs::create_dir(GENERATOR_FOLDER).unwrap();
        }

        for i in 0..Self::NUMBER_OF_CHAMPIONS as u8 {
            let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i) };
            let data = Self::create(champion_id)?;
            let file_name = format!("{champion_id:?}").to_lowercase();
            std::fs::write(
                format!("{GENERATOR_FOLDER}/{file_name}.rs"),
                data.as_bytes(),
            )?;
        }

        Ok(())
    }

    pub fn run_all() -> MayFail {
        for i in 0..Self::NUMBER_OF_CHAMPIONS {
            let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
            let result = Self::run(champion_id);
            match result {
                Ok(champion) => {
                    champion.into_file(format!("internal/champions/{champion_id:?}.json"))?;
                }
                Err(e) => {
                    println!("Error generating {champion_id:?}: {e:?}. Performing offset check.");
                    match Self::check_offsets(champion_id)? {
                        true => println!("{champion_id:?} have an issue unrelated to offsets"),
                        false => println!("{champion_id:?} likely has incorrect offsets"),
                    }
                }
            };
        }
        Ok(())
    }

    pub fn run(champion_id: ChampionId) -> MayFail<Champion> {
        let data =
            MerakiChampion::from_file(format!("cache/meraki/champions/{champion_id:?}.json"))?;
        let function = Self::GENERATOR_FUNCTIONS[champion_id as usize];
        let generator = function(data);
        Ok(generator.generate()?)
    }

    pub fn parse_offsets(src: &str) -> MayFail<HashMap<char, Vec<(usize, usize)>>> {
        let macro_re = Regex::new(r"(?s)ability!\s*\[\s*([A-Za-z])\s*,(.*?)\]")?;
        let tuple_re = Regex::new(r"\(\s*(\d+)\s*,\s*(\d+)\s*,")?;

        let mut out = HashMap::<char, Vec<(usize, usize)>>::new();

        for caps in macro_re.captures_iter(src) {
            let ability = caps[1].chars().next().ok_or("First capture has no chars")?;

            if !matches!(ability, 'Q' | 'W' | 'E' | 'R') {
                continue;
            }

            let body = &caps[2];

            let mut tuples = Vec::new();
            for t in tuple_re.captures_iter(body) {
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

            println!("  Ability '{k}':");
            println!("      found(old):     {old_values:?}");
            println!("      expected(new):  {new_values:?}");
            if !missing.is_empty() {
                println!("  missing in new: {missing:?}");
            }
            if !extra.is_empty() {
                println!("      extra in new:     {extra:?}");
            }
        }
    }

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

    pub fn check_all_offsets() {
        for i in 0..Self::NUMBER_OF_CHAMPIONS {
            let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
            match Self::check_offsets(champion_id) {
                Err(e) => {
                    println!("Error checking {champion_id:?} offsets: {e:?}");
                }
                Ok(false) => {
                    println!("{champion_id:?} has incorrect offsets");
                }
                Ok(true) => {
                    println!("{champion_id:?} passed offsets check");
                }
            };
        }
    }

    pub fn check_offsets(name: ChampionId) -> MayFail<bool> {
        let meraki_champion =
            MerakiChampion::from_file(format!("cache/meraki/champions/{name:?}.json"))?;

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

        let old_content = std::fs::read_to_string(format!("{GENERATOR_FOLDER}/{name:?}.rs"))?;

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
    pub fn new(data: MerakiChampion) -> Self {
        Self {
            data,
            hashmap: HashMap::new(),
            mergevec: Vec::new(),
        }
    }

    pub fn finish(self) -> Champion {
        Champion {
            name: self.data.name,
            adaptative_type: self.data.adaptive_type.into(),
            attack_type: self.data.attack_type.into(),
            positions: self
                .data
                .positions
                .into_iter()
                .map(|pos| Position::from_raw(&pos).unwrap_or_default())
                .collect(),
            stats: self.data.stats,
            abilities: self.hashmap.into_iter().collect(),
            merge_data: self.mergevec,
        }
    }

    pub fn get_meraki_ability<'a>(
        &'a self,
        ability: AbilityLike,
        ability_offset: usize,
    ) -> &'a MerakiAbility {
        let abilities = &self.data.abilities;
        let meraki_abilities = match ability {
            AbilityLike::P(_) => &abilities.p,
            AbilityLike::Q(_) => &abilities.q,
            AbilityLike::W(_) => &abilities.w,
            AbilityLike::E(_) => &abilities.e,
            AbilityLike::R(_) => &abilities.r,
        };
        &meraki_abilities[ability_offset]
    }

    pub fn get_ability_offsets(abilities: Vec<MerakiAbility>) -> Vec<MerakiOffset> {
        let mut macro_offsets = Vec::<MerakiOffset>::new();
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
                    macro_offsets.push(offset);
                    bindings += 1;
                }
            }
        }

        macro_offsets
    }

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
        ability: AbilityLike,
        pattern: [(usize, usize, AbilityName); N],
    ) -> [(usize, usize, AbilityLike); N] {
        let mut offsets = [(0, 0, AbilityLike::P(AbilityName::Void)); N];
        let mut i = 0;
        while i < N {
            let offset = pattern[i];
            let (a, b, c) = offset;
            offsets[i] = (a, b, ability.from_ability_name(c));
            i += 1;
        }
        offsets
    }

    pub fn insert(&mut self, key: impl Into<AbilityLike>, ability: Ability) {
        self.hashmap.insert(key.into(), ability);
    }

    pub fn get_mut(&mut self, key: impl Into<AbilityLike>) -> MayFail<&mut Ability> {
        let field = key.into();
        Ok(self
            .hashmap
            .get_mut(&field)
            .ok_or("[get_mut] Failed to find field: {key:?}".to_string())?)
    }

    pub fn get(&self, key: impl Into<AbilityLike>) -> MayFail<&Ability> {
        let field = key.into();
        Ok(self
            .hashmap
            .get(&field)
            .ok_or(format!("[get] Failed to find field: {field:?}"))?)
    }

    pub fn ability<const N: usize>(
        &mut self,
        key: AbilityLike,
        pattern: [(usize, usize, AbilityName); N],
    ) {
        let offsets = Self::modify_pattern(key, pattern);
        self.extract_ability_damage(key.into(), 0, &offsets);
    }

    pub fn attr<const N: usize>(
        &mut self,
        attr: Attrs,
        set: [impl Into<AbilityLike>; N],
    ) -> MayFail {
        for key in set {
            self.get_mut(key.into())?.attributes = attr;
        }
        Ok(())
    }

    pub fn extract_ability_damage(
        &mut self,
        ability: AbilityLike,
        ability_offset: usize,
        pattern: &[(usize, usize, AbilityLike)],
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
                        self.hashmap.insert(
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

    pub fn get_passive_description(&self, offsets: (usize, usize)) -> (&MerakiAbility, &str) {
        let (ability_index, effect_index) = offsets;

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

    pub fn end(mut self) -> MayFail<Champion> {
        let name = &self.data.name;

        // Verifies if any ability found has unknown damage and emits a warning
        // to the console so it can be fixed by the next time the generator runs
        self.hashmap
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
        let keys = self.hashmap.keys().cloned().collect::<Vec<_>>();
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
                let ability_like = make(ability_name);
                if self.hashmap.contains_key(&ability_like) {
                    self.mergevec.push((key, ability_like));
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
        if !self
            .mergevec
            .iter()
            .all(|(a, b)| self.hashmap.contains_key(a) && self.hashmap.contains_key(b))
        {
            println!(
                "{name}: inconsistent data inserted in macro `merge!`.\nmerge_vec: {:?},\n`hashmap_keys: {:?}",
                self.mergevec,
                self.hashmap.keys().collect::<Vec<_>>()
            );
            return Err("Found inconsistent merge vec".into());
        }

        Ok(self.finish())
    }

    pub fn passive(
        &mut self,
        name: AbilityName,
        offsets: (usize, usize),
        postfix: Option<String>,
        scalings: Option<usize>,
    ) {
        let (passive, passive_description) = self.get_passive_description(offsets);

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

        self.hashmap
            .insert(AbilityLike::P(name), passive.format(damage));
    }

    /// Takes in two fields and returns a mutable reference to a new cloned value, that was
    /// already inserted to `self.hashmap`. The first enum is from where it is being cloned,
    /// and the second one is the new name it will have, and will be identical.
    pub fn clone_to(
        &mut self,
        from: impl Into<AbilityLike>,
        into: impl Into<AbilityLike>,
    ) -> MayFail<&mut Ability> {
        let clone_from = self.get(from.into())?.clone();
        let into_key = into.into();
        self.insert(into_key, clone_from);
        self.get_mut(into_key)
    }

    pub fn damage_type(&mut self, key: impl Into<AbilityLike>, damage_type: DamageType) -> MayFail {
        self.get_mut(key.into())?.damage_type = damage_type;
        Ok(())
    }

    pub fn merge_damage<const N: usize>(
        &self,
        closure: fn([String; N]) -> String,
        args: [impl Into<AbilityLike> + Copy; N],
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
