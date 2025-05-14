#![allow(dead_code)]

use crate::model::application::GlobalCache;
use crate::model::champions::{CdnChampion, Modifiers};
use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io::Write, path::Path};

use super::*;

pub async fn load_cache() -> GlobalCache {
    let champion_files: Vec<PathBuf> = fs::read_dir("src/internal/champions")
        .expect("Failed to read champions")
        .map(|e| e.unwrap().path())
        .collect();

    let champion_names =
        read_from_file::<HashMap<String, String>>("src/internal/champion_names.json");

    // let item_files: Vec<PathBuf> = fs::read_dir("src/internal/items")
    //     .expect("Failed to read items")
    //     .map(|e| e.unwrap().path())
    //     .collect();

    let mut champions = HashMap::with_capacity(champion_files.len());
    for path in champion_files {
        let data =
            read_from_file::<Champion>(path.to_str().expect("Failed to convert path to string"));
        let name = Path::new(path.to_str().expect("Failed to convert path to string"))
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or_default()
            .trim_end_matches(".json");
        champions.insert(String::from(name), data);
    }

    GlobalCache {
        champions,
        items: HashMap::new(),
        champion_names,
    }
}

// Helper function to fetch data from the CDN. ReturnTypes are not strongly typed.
async fn fetch_api(path_name: &str) -> HashMap<String, Value> {
    let url = std::env::var("CDN_ENDPOINT").expect("CDN_ENDPOINT is not set");
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/{}", url.trim_end_matches('/'), path_name))
        .send()
        .await
        .expect("Failed to send request");

    let data: Value = res.json().await.expect("Failed to parse JSON");

    let result = data.as_object().expect("Failed to convert JSON to object");

    result.clone().into_iter().collect()
}

// Recovers all champions from CDN and create a separate file for each of them.
pub async fn write_champions() {
    let result = fetch_api("champions.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/champions/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

// Recovers all items from CDN and create a separate file for each of them.
pub async fn write_items() {
    let result = fetch_api("items.json").await;

    for (key, value) in result {
        let path_name = format!("cache/cdn/items/{}.json", key);
        let bytes = value.to_string();
        write_to_file(&path_name, bytes.as_bytes());
    }
}

// Creates basic folders necessary to run the program. If one of these folders are not found,
// The program is likely to panic when an update is called.
pub fn setup_folders() {
    for dir in &[
        "cache",
        "cache/cdn",
        "cache/cdn/champions",
        "cache/cdn/items",
        "internal",
    ] {
        let path = Path::new(dir);
        if !path.exists() {
            let error_msg = format!("Unable to create directory '{}'", dir);

            fs::create_dir_all(path).expect(&error_msg);
        }
    }
}

// Helper function to write files
fn write_to_file(path_name: &str, bytes: &[u8]) {
    let mut file = std::fs::File::create(path_name).expect("Unable to create file");
    file.write_all(bytes).expect("Unable to write data");
}

// Helper to read from files and parse the value to a struct
fn read_from_file<T: DeserializeOwned>(path_name: &str) -> T {
    let data = fs::read_to_string(path_name).expect("Unable to read file");
    serde_json::from_str(&data).expect("Failed to parse JSON")
}

// Read every file in cache/cdn/champions folder and delegates the processing to generate_champion_file
pub fn setup_champion_cache() {
    let files =
        fs::read_dir("cache/cdn/champions").expect("Unable to read directory cache/cdn/champions");

    for file in files {
        let path_name = file.unwrap().path();
        tokio::task::spawn_blocking(move || {
            generate_champion_file(
                path_name
                    .to_str()
                    .expect("Failed to convert path to string"),
            );
        });
    }
}

// Takes a string with the match "{number} : {number}" and returns the numeric values
// Might return nothing if no values are found, or a tuple is malformed
fn extract_range_values(input: &str) -> Option<(f64, f64)> {
    let re = Regex::new(r"(\d+(?:\.\d+)?)(%)?\s*[:\-–]\s*(\d+(?:\.\d+)?)(%)?").ok()?;
    let caps = re.captures(input)?;

    let first = caps.get(1)?.as_str().parse::<f64>().ok()?;
    let second = caps.get(3)?.as_str().parse::<f64>().ok()?;

    let first_is_percent = caps.get(2).is_some();
    let second_is_percent = caps.get(4).is_some();

    let denom1 = if first_is_percent { 100.0 } else { 1.0 };
    let denom2 = if second_is_percent { 100.0 } else { 1.0 };

    Some((first / denom1, second / denom2))
}

// Lots of passive strings match with a pattern of (number) : (number) ... (+ Scalings)
// This function returns the first two values it found, assuming there will always be two.
fn extract_passive_bounds(
    data: &CdnChampion,
    indexes: (usize, usize),
) -> (&CdnAbility, (f64, f64)) {
    let (ability_index, effect_index) = indexes;

    let passive = data
        .abilities
        .p
        .get(ability_index)
        .expect("ability_index is invalid.");

    let passive_effects = passive
        .effects
        .get(effect_index)
        .expect("effect_index is invalid.")
        .description
        .clone();

    let passive_bounds = extract_range_values(&passive_effects)
        .expect("Couldn't extract numeric values for passive.");

    (passive, passive_bounds)
}

// Gets the tuples that are in pattern (+ Scalling) and formats the string to the internal format.
fn extract_scaled_values(input: &str) -> String {
    let re = Regex::new(r"\(([^)]+)\)").unwrap();
    let mut result = Vec::new();
    for cap in re.captures_iter(input) {
        let content = cap[1].trim();
        if content.to_lowercase().contains("based on level") {
            continue;
        }
        let cleaned = content.trim_start_matches('+').trim();
        let parts: Vec<&str> = cleaned.split_whitespace().collect();
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
        .iter()
        .map(|value| replace_keys(value))
        .collect::<Vec<String>>()
        .join(" + ")
}

// Useful for passives where scalling is linear over all 18 levels.
// Returns the array with the values for each level adjusted
fn assign_as_linear_range(bounds: (f64, f64), size: usize, postfix: Option<&str>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let (start, end) = bounds;
    for i in 0..size {
        let value = start + (((end - start) * (i as f64)) / (size as f64 - 1.0));
        if let Some(postfix) = postfix {
            result.push(format!("({} + {})", value, postfix));
            continue;
        }
        result.push(format!("{}", value));
    }
    result
}

fn remove_parenthesized_additions(input: &str) -> String {
    let re = Regex::new(r"\(\+\s*[^)]*\)").unwrap();
    re.replace_all(input, "").to_string()
}

// Takes the default format of the API and assigns to target_vec the correct format
// Used internally.
fn extract_ability(modifiers: &Vec<Modifiers>, target_vec: &mut Vec<String>) {
    if modifiers.is_empty() {
        return;
    }
    let length = modifiers[0].values.len();
    for i in 0..length {
        let mut parts = Vec::new();
        for modifier in modifiers {
            let value = modifier.values[i];
            let raw_unit = modifier.units[i].trim();
            let scallings = extract_scaled_values(&raw_unit);
            let unit = remove_parenthesized_additions(&raw_unit);
            let cleaned_string = if unit.contains('%') {
                let parts: Vec<&str> = unit.split('%').collect();
                let suffix = parts
                    .get(1)
                    .map_or("".to_string(), |s| s.trim().to_string());
                let coef = value / 100.0;
                if coef == 1.0 && !suffix.is_empty() {
                    suffix
                } else if !suffix.is_empty() {
                    format!("({} * {})", trim_f64(coef), suffix)
                } else {
                    format!("{}", trim_f64(coef))
                }
            } else if unit.is_empty() {
                trim_f64(value)
            } else {
                format!("{}{}", trim_f64(value), unit)
            };
            let formatted_string = replace_keys(&cleaned_string);
            let final_string = if scallings.is_empty() {
                formatted_string
            } else {
                format!("{} + {}", formatted_string, scallings)
            };
            parts.push(final_string);
        }
        target_vec.push(parts.join(" + "));
    }
}

pub(super) enum IterationTarget {
    MINIMUM,
    MAXIMUM,
}

type IteratorExtractor<'a> = HashMap<usize, HashMap<usize, (String, &'a IterationTarget)>>;

// Helper function to remove the decimal point if it's not needed, or expand floats.
fn trim_f64(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{:.0}", val)
    } else {
        format!("{}", val)
    }
}

// Takes a pattern of [Index on Vec<Effect>], [Index on Vec<Leveling>], [(Keyname, Max/Min)]
// And assigns to the map the correct format that will be used internally.
pub(super) fn get_from_pattern(
    data: &CdnAbility,
    map: &mut HashMap<String, Ability>,
    pattern: &[(usize, usize, &str, IterationTarget)],
) {
    let mut indexes: IteratorExtractor = HashMap::new();

    for (effect_index, leveling_index, keyname, target_vector) in pattern.into_iter() {
        indexes
            .entry(*effect_index)
            .or_insert(HashMap::new())
            .insert(*leveling_index, (keyname.to_string(), target_vector));
    }

    for (effect_index, leveling) in indexes {
        for (leveling_index, (keyname, target_vector)) in leveling {
            let mut minimum_damage = Vec::<String>::new();
            let mut maximum_damage = Vec::<String>::new();

            let effects = data
                .effects
                .get(effect_index)
                .expect("Effect index passed is wrong.");

            let modifiers = &effects
                .leveling
                .get(leveling_index)
                .expect("Leveling index passed is wrong.")
                .modifiers;

            match target_vector {
                IterationTarget::MINIMUM => extract_ability(modifiers, &mut minimum_damage),
                IterationTarget::MAXIMUM => extract_ability(modifiers, &mut maximum_damage),
            }

            map.insert(keyname, data.format(minimum_damage, maximum_damage));
        }
    }
}

// Takes the reference of the description of one ability, the reference vector
// where data will be written at, and adds the tuples of scalling found.
fn assign_scalings(description: &String, ref_vec: &mut Vec<String>) {
    if description.is_empty() {
        return;
    }
    let scalings = extract_scaled_values(&description);
    ref_vec.iter_mut().for_each(|dmg| {
        *dmg = format!("{} + {}", dmg, scalings);
    });
}

// Easier way to get passive damage when the standard format matches.
// data -> the reference to the data passed to the caller function.
// indexes -> the (ability_index, effect_index) of the description string to be extracted.
// postfix -> optional hardcoded string to be added after each matching in final Vec<String>
// scalings -> optional index where the description string can be found to get passive's scallings
// target_vec -> determines if final ocurrence will be written in Minimum or Maximum vector
// keyname -> name of the key to be added in the map after final Vec<String> is created
// map -> reference to the map created internally by the caller function. (Must be created)
pub(super) fn get_passive_damage(
    data: &CdnChampion,
    indexes: (usize, usize),
    postfix: Option<&str>,
    scalings: Option<usize>,
    target_vec: &IterationTarget,
    keyname: &str,
    map: &mut HashMap<String, Ability>,
) {
    let mut minimum_damage = Vec::<String>::new();
    let mut maximum_damage = Vec::<String>::new();

    let (passive, passive_bounds) = extract_passive_bounds(&data, indexes);

    let mut description = &String::new();

    if let Some(scalings) = scalings {
        description = &passive.effects[scalings].description;
    }

    match target_vec {
        IterationTarget::MINIMUM => {
            minimum_damage = assign_as_linear_range(passive_bounds, 18, postfix);
            assign_scalings(&description, &mut minimum_damage);
        }
        IterationTarget::MAXIMUM => {
            maximum_damage = assign_as_linear_range(passive_bounds, 18, postfix);
            assign_scalings(&description, &mut minimum_damage);
        }
    };

    map.insert(
        String::from(keyname),
        passive.format(minimum_damage, maximum_damage),
    );
}

// Replaces common keys found in the API with the corresponding ones used internally
pub(super) fn replace_keys(s: &str) -> String {
    let replacements = [
        ("of target's maximum health", "ENEMY_MAX_HEALTH"),
        ("target's current health", "ENEMY_CURRENT_HEALTH"),
        ("target's missing health", "ENEMY_MISSING_HEALTH"),
        ("per Feast stack", "CHOGATH_STACKS"),
        ("bonus AD", "BONUS_AD"),
        ("bonus health", "BONUS_HEALTH"),
    ];

    replacements
        .iter()
        .fold(s.to_string(), |acc, (old, new)| acc.replace(old, new))
}

pub fn rewrite_champion_names() {
    let files =
        fs::read_dir("cache/cdn/champions").expect("Unable to read directory cache/cdn/champions");

    let mut map = HashMap::<String, String>::new();

    for file in files {
        let path_buf = file.unwrap().path();

        let path_name = path_buf.to_str().unwrap();

        let result = read_from_file::<CdnChampion>(path_name);

        let name = Path::new(path_name)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or_default()
            .trim_end_matches(".json");

        map.insert(result.name, name.to_string());
    }

    write_to_file(
        "src/internal/champion_names.json",
        serde_json::to_string(&map).unwrap().as_bytes(),
    );
}

// Automatically updates every champion in the game. New champions, or big updates to existing
// champions will need to be rewritten over time. If an error occurs while trying to update a
// champion, it will be skipped. Writes the resulting json to internal/{champion_name}.json
fn generate_champion_file(path_name: &str) {
    let result = read_from_file::<CdnChampion>(path_name);

    let name = Path::new(path_name)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or_default()
        .trim_end_matches(".json");

    let champion: Option<Champion> = match name {
        "Aatrox" => Some(aatrox::transform(result)),
        "Ahri" => Some(ahri::transform(result)),
        "Akali" => Some(akali::transform(result)),
        // "Akshan" => Some(akshan::transform(result)),
        // "Alistar" => Some(alistar::transform(result)),
        // "Ambessa" => Some(ambessa::transform(result)),
        // "Amumu" => Some(amumu::transform(result)),
        // "Anivia" => Some(anivia::transform(result)),
        // "Annie" => Some(annie::transform(result)),
        // "Aphelios" => Some(aphelios::transform(result)),
        // "Ashe" => Some(ashe::transform(result)),
        // "AurelionSol" => Some(aurelionsol::transform(result)),
        // "Aurora" => Some(aurora::transform(result)),
        // "Azir" => Some(azir::transform(result)),
        // "Bard" => Some(bard::transform(result)),
        // "Belveth" => Some(belveth::transform(result)),
        // "Blitzcrank" => Some(blitzcrank::transform(result)),
        // "Brand" => Some(brand::transform(result)),
        // "Braum" => Some(braum::transform(result)),
        // "Briar" => Some(briar::transform(result)),
        // "Caitlyn" => Some(caitlyn::transform(result)),
        // "Camille" => Some(camille::transform(result)),
        // "Cassiopeia" => Some(cassiopeia::transform(result)),
        "Chogath" => Some(chogath::transform(result)),
        // "Corki" => Some(corki::transform(result)),
        // "Darius" => Some(darius::transform(result)),
        // "Diana" => Some(diana::transform(result)),
        // "Draven" => Some(draven::transform(result)),
        // "DrMundo" => Some(drmundo::transform(result)),
        // "Ekko" => Some(ekko::transform(result)),
        // "Elise" => Some(elise::transform(result)),
        // "Evelynn" => Some(evelynn::transform(result)),
        // "Ezreal" => Some(ezreal::transform(result)),
        // "Fiddlesticks" => Some(fiddlesticks::transform(result)),
        // "Fiora" => Some(fiora::transform(result)),
        // "Fizz" => Some(fizz::transform(result)),
        // "Galio" => Some(galio::transform(result)),
        // "Gangplank" => Some(gangplank::transform(result)),
        // "Garen" => Some(garen::transform(result)),
        // "Gnar" => Some(gnar::transform(result)),
        // "Gragas" => Some(gragas::transform(result)),
        // "Graves" => Some(graves::transform(result)),
        // "Gwen" => Some(gwen::transform(result)),
        // "Hecarim" => Some(hecarim::transform(result)),
        // "Heimerdinger" => Some(heimerdinger::transform(result)),
        // "Hwei" => Some(hwei::transform(result)),
        // "Illaoi" => Some(illaoi::transform(result)),
        // "Irelia" => Some(irelia::transform(result)),
        // "Ivern" => Some(ivern::transform(result)),
        // "Janna" => Some(janna::transform(result)),
        // "JarvanIV" => Some(jarvaniv::transform(result)),
        // "Jax" => Some(jax::transform(result)),
        // "Jayce" => Some(jayce::transform(result)),
        // "Jhin" => Some(jhin::transform(result)),
        // "Jinx" => Some(jinx::transform(result)),
        // "Kaisa" => Some(kaisa::transform(result)),
        // "Kalista" => Some(kalista::transform(result)),
        // "Karma" => Some(karma::transform(result)),
        // "Karthus" => Some(karthus::transform(result)),
        // "Kassadin" => Some(kassadin::transform(result)),
        // "Katarina" => Some(katarina::transform(result)),
        // "Kayle" => Some(kayle::transform(result)),
        // "Kayn" => Some(kayn::transform(result)),
        // "Kennen" => Some(kennen::transform(result)),
        // "Khazix" => Some(khazix::transform(result)),
        // "Kindred" => Some(kindred::transform(result)),
        // "Kled" => Some(kled::transform(result)),
        // "KogMaw" => Some(kogmaw::transform(result)),
        // "KSante" => Some(ksante::transform(result)),
        // "Leblanc" => Some(leblanc::transform(result)),
        // "LeeSin" => Some(leesin::transform(result)),
        // "Leona" => Some(leona::transform(result)),
        // "Lillia" => Some(lillia::transform(result)),
        // "Lissandra" => Some(lissandra::transform(result)),
        // "Lucian" => Some(lucian::transform(result)),
        // "Lulu" => Some(lulu::transform(result)),
        // "Lux" => Some(lux::transform(result)),
        // "Malphite" => Some(malphite::transform(result)),
        // "Malzahar" => Some(malzahar::transform(result)),
        // "Maokai" => Some(maokai::transform(result)),
        // "MasterYi" => Some(masteryi::transform(result)),
        // "Mel" => Some(mel::transform(result)),
        // "Milio" => Some(milio::transform(result)),
        // "MissFortune" => Some(missfortune::transform(result)),
        // "MonkeyKing" => Some(monkeyking::transform(result)),
        // "Mordekaiser" => Some(mordekaiser::transform(result)),
        // "Morgana" => Some(morgana::transform(result)),
        // "Naafiri" => Some(naafiri::transform(result)),
        // "Nami" => Some(nami::transform(result)),
        // "Nasus" => Some(nasus::transform(result)),
        // "Nautilus" => Some(nautilus::transform(result)),
        "Neeko" => Some(neeko::transform(result)),
        // "Nidalee" => Some(nidalee::transform(result)),
        // "Nilah" => Some(nilah::transform(result)),
        // "Nocturne" => Some(nocturne::transform(result)),
        // "Nunu" => Some(nunu::transform(result)),
        // "Olaf" => Some(olaf::transform(result)),
        // "Orianna" => Some(orianna::transform(result)),
        // "Ornn" => Some(ornn::transform(result)),
        // "Pantheon" => Some(pantheon::transform(result)),
        // "Poppy" => Some(poppy::transform(result)),
        // "Pyke" => Some(pyke::transform(result)),
        // "Qiyana" => Some(qiyana::transform(result)),
        // "Quinn" => Some(quinn::transform(result)),
        // "Rakan" => Some(rakan::transform(result)),
        // "Rammus" => Some(rammus::transform(result)),
        // "RekSai" => Some(reksai::transform(result)),
        // "Rell" => Some(rell::transform(result)),
        // "Renata" => Some(renata::transform(result)),
        // "Renekton" => Some(renekton::transform(result)),
        // "Rengar" => Some(rengar::transform(result)),
        // "Riven" => Some(riven::transform(result)),
        // "Rumble" => Some(rumble::transform(result)),
        // "Ryze" => Some(ryze::transform(result)),
        // "Samira" => Some(samira::transform(result)),
        // "Sejuani" => Some(sejuani::transform(result)),
        // "Senna" => Some(senna::transform(result)),
        // "Seraphine" => Some(seraphine::transform(result)),
        // "Sett" => Some(sett::transform(result)),
        // "Shaco" => Some(shaco::transform(result)),
        // "Shen" => Some(shen::transform(result)),
        // "Shyvana" => Some(shyvana::transform(result)),
        // "Singed" => Some(singed::transform(result)),
        // "Sion" => Some(sion::transform(result)),
        // "Sivir" => Some(sivir::transform(result)),
        // "Skarner" => Some(skarner::transform(result)),
        // "Smolder" => Some(smolder::transform(result)),
        // "Sona" => Some(sona::transform(result)),
        // "Soraka" => Some(soraka::transform(result)),
        // "Swain" => Some(swain::transform(result)),
        // "Sylas" => Some(sylas::transform(result)),
        // "Syndra" => Some(syndra::transform(result)),
        // "TahmKench" => Some(tahmkench::transform(result)),
        // "Taliyah" => Some(taliyah::transform(result)),
        // "Talon" => Some(talon::transform(result)),
        // "Taric" => Some(taric::transform(result)),
        // "Teemo" => Some(teemo::transform(result)),
        // "Thresh" => Some(thresh::transform(result)),
        // "Tristana" => Some(tristana::transform(result)),
        // "Trundle" => Some(trundle::transform(result)),
        // "Tryndamere" => Some(tryndamere::transform(result)),
        // "TwistedFate" => Some(twistedfate::transform(result)),
        // "Twitch" => Some(twitch::transform(result)),
        // "Udyr" => Some(udyr::transform(result)),
        // "Urgot" => Some(urgot::transform(result)),
        // "Varus" => Some(varus::transform(result)),
        // "Vayne" => Some(vayne::transform(result)),
        // "Veigar" => Some(veigar::transform(result)),
        // "Velkoz" => Some(velkoz::transform(result)),
        // "Vex" => Some(vex::transform(result)),
        // "Vi" => Some(vi::transform(result)),
        // "Viego" => Some(viego::transform(result)),
        // "Viktor" => Some(viktor::transform(result)),
        // "Vladimir" => Some(vladimir::transform(result)),
        // "Volibear" => Some(volibear::transform(result)),
        // "Warwick" => Some(warwick::transform(result)),
        // "Xayah" => Some(xayah::transform(result)),
        // "Xerath" => Some(xerath::transform(result)),
        // "XinZhao" => Some(xinzhao::transform(result)),
        // "Yasuo" => Some(yasuo::transform(result)),
        // "Yone" => Some(yone::transform(result)),
        // "Yorick" => Some(yorick::transform(result)),
        // "Yuumi" => Some(yuumi::transform(result)),
        // "Zac" => Some(zac::transform(result)),
        // "Zed" => Some(zed::transform(result)),
        // "Zeri" => Some(zeri::transform(result)),
        // "Ziggs" => Some(ziggs::transform(result)),
        // "Zilean" => Some(zilean::transform(result)),
        // "Zoe" => Some(zoe::transform(result)),
        // "Zyra" => Some(zyra::transform(result)),
        _ => None,
    };

    if champion.is_none() {
        return;
    }

    let bytes = serde_json::to_string(&champion).unwrap();

    write_to_file(
        &format!("src/internal/champions/{}.json", name),
        bytes.as_bytes(),
    );
}
