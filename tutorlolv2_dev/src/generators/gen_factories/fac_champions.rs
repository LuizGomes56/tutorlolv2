use crate::{
    champions::{Ability, CdnAbility, CdnChampion, Champion, Modifiers},
    essentials::ext::FilePathExt,
    generators::{
        Generator, MayFail,
        gen_decl::decl_champions::*,
        gen_utils::{F64Ext, RegExtractor},
    },
};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::Path,
    str::FromStr,
};
use tutorlolv2_fmt::invoke_rustfmt;
use tutorlolv2_gen::{
    AbilityLike, AbilityName, ChampionId, EvalIdent, INTERNAL_CHAMPIONS, Position,
};

const GENERATOR_FOLDER: &str = "tutorlolv2_dev/src/generators_v2/gen_champions";

pub struct ChampionData {
    pub data: CdnChampion,
    pub hashmap: HashMap<AbilityLike, Ability>,
    pub mergevec: Vec<(AbilityLike, AbilityLike)>,
}

pub struct ChampionFactory;

impl ChampionFactory {
    pub const NUMBER_OF_CHAMPIONS: usize = INTERNAL_CHAMPIONS.len();
    pub const GENERATOR_FUNCTIONS: [fn(CdnChampion) -> Box<dyn Generator<Champion>>;
        Self::NUMBER_OF_CHAMPIONS] =
        tutorlolv2_macros::expand_dir!("../internal/champions", |[Name]| Name::new);

    pub fn create(champion_id: ChampionId) -> MayFail<String> {
        let file_name = format!("{champion_id:?}").to_lowercase();
        let path = format!("{GENERATOR_FOLDER}/{file_name}.rs");

        let bind_macro = |ability_char: char, cdn_offsets: &[CdnOffset]| -> String {
            let mut offsets = Vec::new();
            for cdn_offset in cdn_offsets {
                offsets.push(format!(
                    "({effect_index}, {leveling_index}, {enum_binding:?})",
                    effect_index = cdn_offset.effect,
                    leveling_index = cdn_offset.leveling,
                    enum_binding = cdn_offset.binding
                ));
            }
            format!(
                "ability![{ability_char}, {offsets}];",
                offsets = offsets.join(","),
            )
        };

        let mut generated_content = format!(
            "use super::*;

                impl Generator<Champion> for {champion_id:?} {{
                    #[generator_v2]
                    fn generate(mut self: Box<Self>) -> MayFail<Champion> {{"
        );

        if let Ok(data) = std::fs::read_to_string(&path) {
            if data.contains("#![stable]") {
                return Ok(data);
            }
        }

        let cdn_champion =
            format!("cache/cdn/champions/{champion_id:?}.json").read_json::<CdnChampion>()?;

        for (ability_char, ability_vec) in cdn_champion.abilities.into_iter() {
            let cdn_offsets = ChampionData::get_ability_offsets(ability_vec);
            if cdn_offsets.len() > 0 {
                generated_content.push_str(&bind_macro(ability_char, &cdn_offsets));
            }
        }

        generated_content.push_str("}}");
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
            format!("{GENERATOR_FOLDER}/{file_name}.rs").write_to_file(data.as_bytes())?;
        }

        Ok(())
    }

    pub fn run_all() {
        for i in 0..Self::NUMBER_OF_CHAMPIONS {
            let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
            let result = Self::run(champion_id);
            match result {
                Ok(champion) => {
                    let json_string = serde_json::to_string_pretty(&champion).unwrap();
                    format!("internal/champions/{champion_id:?}.json")
                        .write_to_file(json_string.as_bytes())
                        .unwrap();
                }
                Err(e) => {
                    println!("Error generating {champion_id:?}: {e:?}. Performing offset check.");
                    match Self::check_offsets(champion_id) {
                        Ok(true) => println!("{champion_id:?} have an issue unrelated to offsets"),
                        Ok(false) => println!("{champion_id:?} likely has incorrect offsets"),
                        Err(e) => println!("Error checking offsets for {champion_id:?}: {e:?}"),
                    }
                }
            };
        }
    }

    pub fn run(champion_id: ChampionId) -> MayFail<Champion> {
        let data =
            format!("cache/cdn/champions/{champion_id:?}.json").read_json::<CdnChampion>()?;
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

            println!("  Ability '{}':", k);
            println!("      expected(old): {old_values:?}");
            println!("      found(new):    {new_values:?}");
            if !missing.is_empty() {
                println!("      missing in new:   {missing:?}");
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
        let cdn_champion =
            format!("cache/cdn/champions/{name:?}.json").read_json::<CdnChampion>()?;

        let mut new_offsets = HashMap::<char, Vec<(usize, usize)>>::new();

        for (ability_char, ability_vec) in cdn_champion.abilities.into_iter() {
            let cdn_offsets = ChampionData::get_ability_offsets(ability_vec);
            new_offsets.insert(
                ability_char,
                cdn_offsets.iter().map(|o| (o.effect, o.leveling)).collect(),
            );
        }

        let old_content = std::fs::read_to_string(format!("{GENERATOR_FOLDER}/{name:?}.rs"))?;

        if !Self::compare_offsets(&old_content, &new_offsets)? {
            return Ok(false);
        }

        Ok(true)
    }
}

pub struct CdnOffset {
    pub effect: usize,
    pub leveling: usize,
    pub binding: AbilityName,
}

impl ChampionData {
    pub fn new(data: CdnChampion) -> Self {
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
            abilities: self.hashmap,
            merge_data: self.mergevec,
        }
    }

    pub fn get_cdn_ability<'a>(
        &'a self,
        ability: AbilityLike,
        ability_offset: usize,
    ) -> &'a CdnAbility {
        let abilities = &self.data.abilities;
        let cdn_abilities = match ability {
            AbilityLike::P(_) => &abilities.p,
            AbilityLike::Q(_) => &abilities.q,
            AbilityLike::W(_) => &abilities.w,
            AbilityLike::E(_) => &abilities.e,
            AbilityLike::R(_) => &abilities.r,
        };
        &cdn_abilities[ability_offset]
    }

    pub fn get_ability_offsets(abilities: Vec<CdnAbility>) -> Vec<CdnOffset> {
        let mut macro_offsets = Vec::<CdnOffset>::new();
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

                    let offset = CdnOffset {
                        effect: effect_index,
                        leveling: leveling_index,
                        binding: {
                            let enum_match = match bindings {
                                ..9 => format!("_{}", bindings.to_string()),
                                _ => format!("_{}Min", bindings - 8),
                            };
                            AbilityName::from_str(&enum_match).unwrap()
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
                    let raw_unit = modifier.units[i].trim();
                    let scallings = raw_unit.get_scalings();
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
                            format!("({} * {})", coef.trim(), suffix)
                        } else {
                            format!("{}", coef.trim())
                        }
                    } else if unit.is_empty() {
                        value.trim()
                    } else {
                        format!("{}{}", value.trim(), unit)
                    };
                    let formatted_string = cleaned_string.replace_keys();
                    let final_string = if scallings.is_empty() {
                        formatted_string
                    } else {
                        format!("{} + {}", formatted_string, scallings)
                    };
                    parts.push(final_string);
                }
            }
            result.push(parts.join(" + "));
        }
        result
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
                let cdn_ability = self.get_cdn_ability(ability, ability_offset);
                if let Some(effects) = cdn_ability.effects.get(effect_index) {
                    if let Some(level_entry) = effects.leveling.get(leveling_index) {
                        let modifiers = &level_entry.modifiers;
                        self.hashmap.insert(
                            *keyname,
                            cdn_ability.format(Self::extract_ability(&modifiers)),
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

    pub fn extract_passive_bounds(&self, offsets: (usize, usize)) -> (&CdnAbility, (f64, f64)) {
        let (ability_index, effect_index) = offsets;

        let passive = self
            .data
            .abilities
            .p
            .get(ability_index)
            .expect("Self::extract_passive_bounds: ability_index is invalid.");

        let passive_effects = passive
            .effects
            .get(effect_index)
            .expect("Self::extract_passive_bounds: effect_index is invalid.")
            .description
            .clone();

        let passive_bounds = passive_effects
            .get_interval()
            .expect("Couldn't extract numeric values for passive.");

        (passive, passive_bounds)
    }

    pub fn extract_passive_damage(
        &mut self,
        ability: AbilityLike,
        offsets: (usize, usize),
        postfix: Option<EvalIdent>,
        scalings: Option<usize>,
    ) {
        let (passive, passive_bounds) = self.extract_passive_bounds(offsets);
        let mut description = &String::new();
        if let Some(scalings) = scalings {
            description = &passive.effects[scalings].description;
        }

        let mut damage =
            <str as RegExtractor>::process_linear_scalings(passive_bounds, 18, postfix);
        if description.is_empty() {
            return;
        }

        let scalings = description.get_scalings();
        if scalings.len() > 0 {
            damage.iter_mut().for_each(|dmg: &mut String| {
                *dmg = format!("{} + {}", dmg, scalings);
            });
        }

        self.hashmap.insert(ability, passive.format(damage));
    }
}
