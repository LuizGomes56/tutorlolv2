use crate::{
    CdnChampion,
    champions::{CdnAbility, Modifiers},
    essentials::ext::FilePathExt,
    generators::*,
    setup::generators::decl_v2::*,
};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::Deref,
    path::Path,
    str::FromStr,
};
use tutorlolv2_fmt::invoke_rustfmt;
use tutorlolv2_gen::{ChampionId, INTERNAL_CHAMPIONS, Position};

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub const NUMBER_OF_CHAMPIONS: usize = INTERNAL_CHAMPIONS.len();
pub const CHAMPION_NAMES: [&str; NUMBER_OF_CHAMPIONS] = {
    let mut i = 0;
    let mut output = [""; NUMBER_OF_CHAMPIONS];
    while i < NUMBER_OF_CHAMPIONS {
        let data = INTERNAL_CHAMPIONS[i];
        output[i] = data.name;
        i += 1;
    }
    output
};

const GENERATOR_FOLDER: &str = "tutorlolv2_dev/src/generators_v2";

trait F64Ext {
    fn trim(&self) -> String;
}

impl F64Ext for f64 {
    fn trim(&self) -> String {
        if self.fract() == 0.0 {
            format!("{:.0}", self)
        } else {
            format!("{}", self)
        }
    }
}

pub trait RegExtractor {
    fn replace_keys(&self) -> String;
    fn remove_parenthesis(&self) -> String;
    fn get_scalings(&self) -> String;
    fn get_interval(&self) -> Option<(f64, f64)>;
    fn get_damage(&self) -> String;
    fn from_scaled_string(&self) -> String;
    fn process_linear_scalings(
        bounds: (f64, f64),
        size: usize,
        postfix: Option<&str>,
    ) -> Vec<String>;
}

impl RegExtractor for str {
    fn replace_keys(&self) -> String {
        let replacements = [
            ("per 100", "0.01 *"),
            ("of damage dealt", "100.0"),
            ("of damage stored", "100.0"),
            ("per Soul collected", " * THRESH_STACKS"),
            ("of expended Grit", "0.0"),
            ("of primary target's bonus health", "ENEMY_BONUS_HEALTH"),
            ("of his bonus health", "BONUS_HEALTH"),
            ("Pantheon's bonus health", "BONUS_HEALTH"),
            ("critical strike chance", "CRIT_CHANCE"),
            ("of the original damage", "100.0"),
            ("per Overwhelm stack on the target", "1.0"),
            ("of Ivern's AP", "AP"),
            ("of Sona's AP", "AP"),
            ("per Feast stack", "CHOGATH_STACKS"),
            ("of Siphoning Strike stacks", "NASUS_STACKS"),
            ("Stardust", "AURELION_SOL_STACKS"),
            ("per mark", "KINDRED_STACKS"),
            ("bonus armor", "BONUS_ARMOR"),
            ("bonus mana", "BONUS_MANA"),
            ("bonus AD", "BONUS_AD"),
            ("bonus armor", "BONUS_ARMOR"),
            ("bonus magic resistance", "BONUS_MAGIC_RESIST"),
            ("bonus health", "BONUS_HEALTH"),
            ("bonus movement speed", "BONUS_MOVE_SPEED"),
            ("armor", "ARMOR"),
            ("of the target's maximum health", "ENEMY_MAX_HEALTH"),
            ("of target's maximum health", "ENEMY_MAX_HEALTH"),
            ("of Zac's maximum health", "MAX_HEALTH"),
            ("of Braum's maximum health", "MAX_HEALTH"),
            ("of her maximum health", "MAX_HEALTH"),
            ("of his maximum health", "MAX_HEALTH"),
            ("of maximum health", "MAX_HEALTH"),
            ("maximum health", "MAX_HEALTH"),
            ("of the target's current health", "ENEMY_CURRENT_HEALTH"),
            ("of target's current health", "ENEMY_CURRENT_HEALTH"),
            ("target's current health", "ENEMY_CURRENT_HEALTH"),
            ("of the target's missing health", "ENEMY_MISSING_HEALTH"),
            ("of target's missing health", "ENEMY_MISSING_HEALTH"),
            ("target's missing health", "ENEMY_MISSING_HEALTH"),
            ("maximum mana", "MAX_MANA"),
        ];

        replacements
            .iter()
            .fold(self.to_string(), |acc, (old, new)| acc.replace(old, new))
    }

    fn remove_parenthesis(&self) -> String {
        Regex::new(r"\(\+\s*[^)]*\)")
            .unwrap()
            .replace_all(self, "")
            .to_string()
    }

    fn get_scalings(&self) -> String {
        let re = Regex::new(r"\(([^)]+)\)").unwrap();
        let mut result = Vec::<String>::new();
        for cap in re.captures_iter(self) {
            let content = cap[1].trim();
            if content.to_lowercase().contains("based on level") {
                continue;
            }
            let cleaned = content.trim_start_matches('+').trim();
            let parts = cleaned.split_whitespace().collect::<Vec<&str>>();
            if parts.len() >= 2 && parts[0].ends_with('%') {
                if let Ok(percent) = parts[0].trim_end_matches('%').parse::<f64>() {
                    let decimal = percent / 100.0;
                    let rest = parts[1..].join(" ");
                    result.push(format!("({} * {})", decimal, rest));
                    continue;
                }
            }
            result.push(format!("({})", cleaned));
        }
        result
            .into_iter()
            .map(|value| value.replace_keys())
            .collect::<Vec<String>>()
            .join(" + ")
    }

    fn get_interval(&self) -> Option<(f64, f64)> {
        let re = Regex::new(r"(\d+(?:\.\d+)?)(%)?\s*[:\-–]\s*(\d+(?:\.\d+)?)(%)?").ok()?;
        let caps = re.captures(self)?;

        let first = caps.get(1)?.as_str().parse::<f64>().ok()?;
        let second = caps.get(3)?.as_str().parse::<f64>().ok()?;

        let first_is_percent = caps.get(2).is_some();
        let second_is_percent = caps.get(4).is_some();

        let denom1 = if first_is_percent { 100.0 } else { 1.0 };
        let denom2 = if second_is_percent { 100.0 } else { 1.0 };

        Some((first / denom1, second / denom2))
    }

    fn get_damage(&self) -> String {
        let re = Regex::new(r"\{\{as\|([^\}]+)\}\}").unwrap();
        let mut results = Vec::<String>::new();
        for cap in re.captures_iter(self) {
            let mut content = cap[1].to_string();
            let nested = Regex::new(r"\{\{[^}]+\|([^}]+)\}\}").unwrap();
            content = nested.replace_all(&content, "$1").to_string();
            results.push(content);
        }
        let scaled_input = results.join(" ").replace("{{as|", "");
        Self::from_scaled_string(&scaled_input)
    }

    fn from_scaled_string(&self) -> String {
        let re = Regex::new(r"\([^\)]*\)").unwrap();
        let paren_part = re.find(self).map(|m| m.as_str()).unwrap_or("");
        let base = self.replace(paren_part, "").trim().to_string();
        let scaled = paren_part.get_scalings();
        if !scaled.is_empty() {
            format!("{} + {}", base, scaled)
        } else {
            base
        }
    }

    fn process_linear_scalings(
        bounds: (f64, f64),
        size: usize,
        postfix: Option<&str>,
    ) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let (start, end) = bounds;
        for i in 0..size {
            let value = start + (((end - start) * (i as f64)) / (size as f64 - 1.0));
            if let Some(postfix) = postfix {
                result.push(format!("({value} + {postfix})"));
                continue;
            }
            result.push(value.to_string());
        }
        result
    }
}

pub struct GeneratorData {
    pub data: CdnChampion,
    pub hashmap: HashMap<AbilityLike, Ability>,
    pub mergevec: Vec<(AbilityLike, AbilityLike)>,
}

pub struct GeneratorFactory(HashMap<ChampionId, fn(CdnChampion) -> Box<dyn Generator>>);

impl GeneratorFactory {
    pub fn new() -> Self {
        let mut inner = HashMap::new();

        for i in 0..NUMBER_OF_CHAMPIONS {
            let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };

            macro_rules! match_arm {
                ($($name:ident),+$(,)?) => {
                    match champion_id {
                        $(ChampionId::$name => $name::new,)+
                    }
                };
            }

            let function = match_arm!(
                Aatrox,
                Ahri,
                Akali,
                Akshan,
                Alistar,
                Ambessa,
                Amumu,
                Anivia,
                Annie,
                Aphelios,
                Ashe,
                AurelionSol,
                Aurora,
                Azir,
                Bard,
                Belveth,
                Blitzcrank,
                Brand,
                Braum,
                Briar,
                Caitlyn,
                Camille,
                Cassiopeia,
                Chogath,
                Corki,
                Darius,
                Diana,
                Draven,
                DrMundo,
                Ekko,
                Elise,
                Evelynn,
                Ezreal,
                Fiddlesticks,
                Fiora,
                Fizz,
                Galio,
                Gangplank,
                Garen,
                Gnar,
                Gragas,
                Graves,
                Gwen,
                Hecarim,
                Heimerdinger,
                Hwei,
                Illaoi,
                Irelia,
                Ivern,
                Janna,
                JarvanIV,
                Jax,
                Jayce,
                Jhin,
                Jinx,
                Kaisa,
                Kalista,
                Karma,
                Karthus,
                Kassadin,
                Katarina,
                Kayle,
                Kayn,
                Kennen,
                Khazix,
                Kindred,
                Kled,
                KogMaw,
                KSante,
                Leblanc,
                LeeSin,
                Leona,
                Lillia,
                Lissandra,
                Lucian,
                Lulu,
                Lux,
                Malphite,
                Malzahar,
                Maokai,
                MasterYi,
                Mel,
                Milio,
                MissFortune,
                MonkeyKing,
                Mordekaiser,
                Morgana,
                Naafiri,
                Nami,
                Nasus,
                Nautilus,
                Neeko,
                Nidalee,
                Nilah,
                Nocturne,
                Nunu,
                Olaf,
                Orianna,
                Ornn,
                Pantheon,
                Poppy,
                Pyke,
                Qiyana,
                Quinn,
                Rakan,
                Rammus,
                RekSai,
                Rell,
                Renata,
                Renekton,
                Rengar,
                Riven,
                Rumble,
                Ryze,
                Samira,
                Sejuani,
                Senna,
                Seraphine,
                Sett,
                Shaco,
                Shen,
                Shyvana,
                Singed,
                Sion,
                Sivir,
                Skarner,
                Smolder,
                Sona,
                Soraka,
                Swain,
                Sylas,
                Syndra,
                TahmKench,
                Taliyah,
                Talon,
                Taric,
                Teemo,
                Thresh,
                Tristana,
                Trundle,
                Tryndamere,
                TwistedFate,
                Twitch,
                Udyr,
                Urgot,
                Varus,
                Vayne,
                Veigar,
                Velkoz,
                Vex,
                Vi,
                Viego,
                Viktor,
                Vladimir,
                Volibear,
                Warwick,
                Xayah,
                Xerath,
                XinZhao,
                Yasuo,
                Yone,
                Yorick,
                Yunara,
                Yuumi,
                Zac,
                Zed,
                Zeri,
                Ziggs,
                Zilean,
                Zoe,
                Zyra
            );
            inner.insert(champion_id, function);
        }

        Self(inner)
    }

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

                impl Generator for {champion_id:?} {{
                    #[generator_v2]
                    fn generate(self: Box<Self>) -> MayFail<Champion> {{"
        );

        if let Ok(data) = std::fs::read_to_string(&path) {
            if data.contains("#![stable]") {
                return Ok(data);
            }
        }

        let cdn_champion =
            format!("cache/cdn/champions/{champion_id:?}.json").read_json::<CdnChampion>()?;

        for (ability_char, ability_vec) in cdn_champion.abilities.into_iter() {
            let cdn_offsets = GeneratorData::get_ability_offsets(ability_vec);
            if cdn_offsets.len() > 0 {
                generated_content.push_str(&bind_macro(ability_char, &cdn_offsets));
            }
        }

        generated_content.push_str("}}");
        Ok(invoke_rustfmt(&generated_content, 80))
    }

    pub async fn create_all() -> MayFail {
        if !Path::new(GENERATOR_FOLDER).exists() {
            std::fs::create_dir(GENERATOR_FOLDER).unwrap();
        }

        let mut futures = Vec::new();

        for i in 0..NUMBER_OF_CHAMPIONS as u8 {
            let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i) };
            futures.push(async move { Self::create(champion_id) });
        }

        for (i, future) in futures.into_iter().enumerate() {
            if let Ok(data) = future.await {
                let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
                let file_name = format!("{champion_id:?}").to_lowercase();
                format!("{GENERATOR_FOLDER}/{file_name}.rs").write_to_file(data.as_bytes())?;
            };
        }

        Ok(())
    }

    pub async fn run_all(&self) {
        let mut futures = Vec::new();
        for i in 0..NUMBER_OF_CHAMPIONS {
            futures.push(async move {
                let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
                let result = self.run(champion_id);
                if let Ok(champion) = result {
                    let json_string = serde_json::to_string_pretty(&champion).unwrap();
                    format!("internal/champions/{champion_id:?}.json")
                        .write_to_file(json_string.as_bytes())
                        .unwrap();
                }
            });
        }
        for future in futures {
            let _ = future.await;
        }
    }

    pub fn run(&self, name: ChampionId) -> MayFail<Champion> {
        let data = format!("cache/cdn/champions/{name:?}.json").read_json::<CdnChampion>()?;
        let function = self[&name];
        let generator = function(data);
        Ok(generator.generate()?)
    }

    pub fn parse_offsets(src: &str) -> MayFail<HashMap<char, Vec<(usize, usize)>>> {
        let macro_re = Regex::new(r"(?s)ability!\s*\[\s*([A-Za-z])\s*,(.*?)\]")?;
        let tuple_re = Regex::new(r"\(\s*(\d+)\s*,\s*(\d+)\s*,")?;

        let mut out = HashMap::<char, Vec<(usize, usize)>>::new();

        for caps in macro_re.captures_iter(src) {
            let ability = caps[1].chars().next().ok_or("First capture has no chars")?;
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
        for i in 0..NUMBER_OF_CHAMPIONS {
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
            let cdn_offsets = GeneratorData::get_ability_offsets(ability_vec);
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

impl Deref for GeneratorFactory {
    type Target = HashMap<ChampionId, fn(CdnChampion) -> Box<dyn Generator>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CdnOffset {
    pub effect: usize,
    pub leveling: usize,
    pub binding: AbilityName,
}

impl GeneratorData {
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
        postfix: Option<&str>,
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
