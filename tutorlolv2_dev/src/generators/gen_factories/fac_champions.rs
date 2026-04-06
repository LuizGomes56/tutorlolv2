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
    AbilityId, AbilityName, AdaptiveType, AttackType, Attrs, CastId, ChampionId, ComboElement,
    DamageType, DevMergeData, Key, Position,
};

pub struct ChampionData {
    pub data: MerakiChampion,
    pub map: BTreeMap<AbilityId, Ability>,
    pub mergevec: BTreeSet<DevMergeData>,
    pub combo: Vec<Vec<ComboElement>>,
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
    const fn function(
        champion_id: ChampionId,
    ) -> fn(MerakiChampion) -> Box<dyn Generator<Champion>> {
        match champion_id {
            ChampionId::Aatrox => Aatrox::new,
            ChampionId::Ahri => Ahri::new,
            ChampionId::Akali => Akali::new,
            ChampionId::Akshan => Akshan::new,
            ChampionId::Alistar => Alistar::new,
            ChampionId::Ambessa => Ambessa::new,
            ChampionId::Amumu => Amumu::new,
            ChampionId::Anivia => Anivia::new,
            ChampionId::Annie => Annie::new,
            ChampionId::Aphelios => Aphelios::new,
            ChampionId::Ashe => Ashe::new,
            ChampionId::AurelionSol => AurelionSol::new,
            ChampionId::Aurora => Aurora::new,
            ChampionId::Azir => Azir::new,
            ChampionId::Bard => Bard::new,
            ChampionId::Belveth => Belveth::new,
            ChampionId::Blitzcrank => Blitzcrank::new,
            ChampionId::Brand => Brand::new,
            ChampionId::Braum => Braum::new,
            ChampionId::Briar => Briar::new,
            ChampionId::Caitlyn => Caitlyn::new,
            ChampionId::Camille => Camille::new,
            ChampionId::Cassiopeia => Cassiopeia::new,
            ChampionId::Chogath => Chogath::new,
            ChampionId::Corki => Corki::new,
            ChampionId::Darius => Darius::new,
            ChampionId::Diana => Diana::new,
            ChampionId::Draven => Draven::new,
            ChampionId::DrMundo => DrMundo::new,
            ChampionId::Ekko => Ekko::new,
            ChampionId::Elise => Elise::new,
            ChampionId::Evelynn => Evelynn::new,
            ChampionId::Ezreal => Ezreal::new,
            ChampionId::Fiddlesticks => Fiddlesticks::new,
            ChampionId::Fiora => Fiora::new,
            ChampionId::Fizz => Fizz::new,
            ChampionId::Galio => Galio::new,
            ChampionId::Gangplank => Gangplank::new,
            ChampionId::Garen => Garen::new,
            ChampionId::Gnar => Gnar::new,
            ChampionId::Gragas => Gragas::new,
            ChampionId::Graves => Graves::new,
            ChampionId::Gwen => Gwen::new,
            ChampionId::Hecarim => Hecarim::new,
            ChampionId::Heimerdinger => Heimerdinger::new,
            ChampionId::Hwei => Hwei::new,
            ChampionId::Illaoi => Illaoi::new,
            ChampionId::Irelia => Irelia::new,
            ChampionId::Ivern => Ivern::new,
            ChampionId::Janna => Janna::new,
            ChampionId::JarvanIV => JarvanIV::new,
            ChampionId::Jax => Jax::new,
            ChampionId::Jayce => Jayce::new,
            ChampionId::Jhin => Jhin::new,
            ChampionId::Jinx => Jinx::new,
            ChampionId::Kaisa => Kaisa::new,
            ChampionId::Kalista => Kalista::new,
            ChampionId::Karma => Karma::new,
            ChampionId::Karthus => Karthus::new,
            ChampionId::Kassadin => Kassadin::new,
            ChampionId::Katarina => Katarina::new,
            ChampionId::Kayle => Kayle::new,
            ChampionId::Kayn => Kayn::new,
            ChampionId::Kennen => Kennen::new,
            ChampionId::Khazix => Khazix::new,
            ChampionId::Kindred => Kindred::new,
            ChampionId::Kled => Kled::new,
            ChampionId::KogMaw => KogMaw::new,
            ChampionId::KSante => KSante::new,
            ChampionId::Leblanc => Leblanc::new,
            ChampionId::LeeSin => LeeSin::new,
            ChampionId::Leona => Leona::new,
            ChampionId::Lillia => Lillia::new,
            ChampionId::Lissandra => Lissandra::new,
            ChampionId::Lucian => Lucian::new,
            ChampionId::Lulu => Lulu::new,
            ChampionId::Lux => Lux::new,
            ChampionId::Malphite => Malphite::new,
            ChampionId::Malzahar => Malzahar::new,
            ChampionId::Maokai => Maokai::new,
            ChampionId::MasterYi => MasterYi::new,
            ChampionId::Mel => Mel::new,
            ChampionId::Milio => Milio::new,
            ChampionId::MissFortune => MissFortune::new,
            ChampionId::MonkeyKing => MonkeyKing::new,
            ChampionId::Mordekaiser => Mordekaiser::new,
            ChampionId::Morgana => Morgana::new,
            ChampionId::Naafiri => Naafiri::new,
            ChampionId::Nami => Nami::new,
            ChampionId::Nasus => Nasus::new,
            ChampionId::Nautilus => Nautilus::new,
            ChampionId::Neeko => Neeko::new,
            ChampionId::Nidalee => Nidalee::new,
            ChampionId::Nilah => Nilah::new,
            ChampionId::Nocturne => Nocturne::new,
            ChampionId::Nunu => Nunu::new,
            ChampionId::Olaf => Olaf::new,
            ChampionId::Orianna => Orianna::new,
            ChampionId::Ornn => Ornn::new,
            ChampionId::Pantheon => Pantheon::new,
            ChampionId::Poppy => Poppy::new,
            ChampionId::Pyke => Pyke::new,
            ChampionId::Qiyana => Qiyana::new,
            ChampionId::Quinn => Quinn::new,
            ChampionId::Rakan => Rakan::new,
            ChampionId::Rammus => Rammus::new,
            ChampionId::RekSai => RekSai::new,
            ChampionId::Rell => Rell::new,
            ChampionId::Renata => Renata::new,
            ChampionId::Renekton => Renekton::new,
            ChampionId::Rengar => Rengar::new,
            ChampionId::Riven => Riven::new,
            ChampionId::Rumble => Rumble::new,
            ChampionId::Ryze => Ryze::new,
            ChampionId::Samira => Samira::new,
            ChampionId::Sejuani => Sejuani::new,
            ChampionId::Senna => Senna::new,
            ChampionId::Seraphine => Seraphine::new,
            ChampionId::Sett => Sett::new,
            ChampionId::Shaco => Shaco::new,
            ChampionId::Shen => Shen::new,
            ChampionId::Shyvana => Shyvana::new,
            ChampionId::Singed => Singed::new,
            ChampionId::Sion => Sion::new,
            ChampionId::Sivir => Sivir::new,
            ChampionId::Skarner => Skarner::new,
            ChampionId::Smolder => Smolder::new,
            ChampionId::Sona => Sona::new,
            ChampionId::Soraka => Soraka::new,
            ChampionId::Swain => Swain::new,
            ChampionId::Sylas => Sylas::new,
            ChampionId::Syndra => Syndra::new,
            ChampionId::TahmKench => TahmKench::new,
            ChampionId::Taliyah => Taliyah::new,
            ChampionId::Talon => Talon::new,
            ChampionId::Taric => Taric::new,
            ChampionId::Teemo => Teemo::new,
            ChampionId::Thresh => Thresh::new,
            ChampionId::Tristana => Tristana::new,
            ChampionId::Trundle => Trundle::new,
            ChampionId::Tryndamere => Tryndamere::new,
            ChampionId::TwistedFate => TwistedFate::new,
            ChampionId::Twitch => Twitch::new,
            ChampionId::Udyr => Udyr::new,
            ChampionId::Urgot => Urgot::new,
            ChampionId::Varus => Varus::new,
            ChampionId::Vayne => Vayne::new,
            ChampionId::Veigar => Veigar::new,
            ChampionId::Velkoz => Velkoz::new,
            ChampionId::Vex => Vex::new,
            ChampionId::Vi => Vi::new,
            ChampionId::Viego => Viego::new,
            ChampionId::Viktor => Viktor::new,
            ChampionId::Vladimir => Vladimir::new,
            ChampionId::Volibear => Volibear::new,
            ChampionId::Warwick => Warwick::new,
            ChampionId::Xayah => Xayah::new,
            ChampionId::Xerath => Xerath::new,
            ChampionId::XinZhao => XinZhao::new,
            ChampionId::Yasuo => Yasuo::new,
            ChampionId::Yone => Yone::new,
            ChampionId::Yorick => Yorick::new,
            ChampionId::Yunara => Yunara::new,
            ChampionId::Yuumi => Yuumi::new,
            ChampionId::Zaahen => Zaahen::new,
            ChampionId::Zac => Zac::new,
            ChampionId::Zed => Zed::new,
            ChampionId::Zeri => Zeri::new,
            ChampionId::Ziggs => Ziggs::new,
            ChampionId::Zilean => Zilean::new,
            ChampionId::Zoe => Zoe::new,
            ChampionId::Zyra => Zyra::new,
        }
    }

    pub fn progress() {
        let stables = ChampionId::VALUES
            .map(|c| {
                if let Ok(data) = std::fs::read_to_string(SaveTo::Generator(c.entity()).path())
                    && (data.contains("Stable") || data.contains("Preserve"))
                {
                    return 1;
                }
                0
            })
            .into_iter()
            .sum::<u32>();

        let total = ChampionId::VARIANTS;

        println!("[Champion Generators]: {stables} / {total} stable generators");
    }

    /// Creates a new generator file, given a [`ChampionId`]
    pub fn create(champion_id: ChampionId) -> MayFail<String> {
        if let Ok(data) = std::fs::read_to_string(SaveTo::Generator(champion_id.entity()).path())
            && (data.contains("Stable") || data.contains("Preserve"))
        {
            println!("[stable] Skipping generator for {champion_id:?}");
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

        let entity_id = &format!("{champion_id:?}");

        let mut generated_content = format!(
            "use super::*;

            impl Generator<Champion> for {entity_id} {{
                fn generate(mut self: Box<Self>) -> MayFail<Champion> {{"
        );

        let meraki_champion =
            MerakiChampion::from_file(SaveTo::MerakiCache(Tag::Champions, entity_id).path())
                .map_err(|e| {
                    format!("Error calling MerakiChampion::from_file for {entity_id:?}: {e:?}")
                })?;
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

        ChampionId::VALUES.into_par_iter().for_each(|champion_id| {
            let Ok(data) = Self::create(champion_id) else {
                return println!("Unable to create generator file for {champion_id:?}");
            };
            std::fs::write(
                SaveTo::Generator(champion_id.entity()).path(),
                data.as_bytes(),
            )
            .unwrap();
        });

        Ok(())
    }

    /// Runs all generator files. It means that several `.json` files will be created
    /// in the internal cache folder
    pub fn run_all() -> MayFail {
        ChampionId::VALUES.into_par_iter().for_each(|champion_id| {
            if let Err(e) = Self::run(champion_id) {
                println!("Failed to run generator file for {champion_id:?}: {e:?}");
            }
        });

        Ok(())
    }

    pub fn run(champion_id: ChampionId) -> MayFail {
        let entity_id = &format!("{champion_id:?}");
        let data =
            MerakiChampion::from_file(SaveTo::MerakiCache(Tag::Champions, entity_id).path())?;
        let function = Self::function(champion_id);
        let generator = function(data);
        match generator.generate() {
            Ok(champion) => champion.into_file(SaveTo::Internal(champion_id.entity()).path()),
            Err(e) => {
                println!("Error generating {entity_id:?}: {e:?}. Performing offset check.");
                match Self::check_offsets(champion_id)? {
                    true => println!("{entity_id:?} have an issue unrelated to offsets"),
                    false => println!("{entity_id:?} likely has incorrect offsets"),
                }
                Ok(())
            }
        }
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
        for champion_id in ChampionId::VALUES {
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
        let name = &format!("{champion_id:?}");
        let meraki_champion =
            MerakiChampion::from_file(SaveTo::MerakiCache(Tag::Champions, name).path())?;

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

        let old_content = std::fs::read_to_string(SaveTo::Generator(champion_id.entity()).path())?;

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
            map: Default::default(),
            mergevec: Default::default(),
            combo: Default::default(),
        }
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
        key: Key,
        pattern: [(usize, usize, AbilityName); N],
    ) -> [(usize, usize, AbilityId); N] {
        let mut offsets = [(0, 0, AbilityId::P(AbilityName::Void)); N];
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

            let make = match key {
                AbilityId::P(_) => AbilityId::P,
                AbilityId::Q(_) => AbilityId::Q,
                AbilityId::W(_) => AbilityId::W,
                AbilityId::E(_) => AbilityId::E,
                AbilityId::R(_) => AbilityId::R,
            };

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
