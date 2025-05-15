#![allow(dead_code)]

use crate::model::application::GlobalCache;
use crate::model::champions::CdnChampion;
use crate::model::items::{CdnItem, Item, PartialStats};
use regex::Regex;
use scraper::{Html, Selector};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tokio::task;

use super::*;

pub(super) fn extract_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.trim_end_matches(".json"))
        .unwrap_or_default()
}

pub async fn load_cache() -> GlobalCache {
    let champion_files: Vec<PathBuf> = fs::read_dir("src/internal/champions")
        .expect("Failed to read champions")
        .map(|e| e.unwrap().path())
        .collect();

    let champion_names =
        read_from_file::<HashMap<String, String>>("src/internal/champion_names.json");

    let item_files: Vec<PathBuf> = fs::read_dir("src/internal/items")
        .expect("Failed to read items")
        .map(|e| e.unwrap().path())
        .collect();

    let mut items = HashMap::<String, Item>::new();

    for path_name in item_files {
        let data = read_from_file::<Item>(
            path_name
                .to_str()
                .expect("Failed to convert path to string"),
        );
        let name = extract_file_name(&path_name);
        items.insert(String::from(name), data);
    }

    let mut champions = HashMap::with_capacity(champion_files.len());
    for path_name in champion_files {
        let data = read_from_file::<Champion>(
            path_name
                .to_str()
                .expect("Failed to convert path to string"),
        );
        let name = extract_file_name(&path_name);
        champions.insert(String::from(name), data);
    }

    GlobalCache {
        champions,
        items,
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

// Recovers all the common builds for the current patch so the app can recommend builds to the user
// Average time to update is 2m30s. Making the outer loop a new task overloads the target website
// causing requests to timeout.
pub async fn get_meta_items() {
    let client_arc = reqwest::Client::new();

    let champion_names =
        read_from_file::<HashMap<String, String>>("src/internal/champion_names.json");

    let positions = ["top", "jungle", "mid", "adc", "support"];
    let mut collected_results = HashMap::<String, HashMap<String, Vec<String>>>::new();

    for (_, name) in champion_names {
        let mut second_future = Vec::new();
        for position in positions {
            let champion_name = name.to_lowercase().clone();
            let client = client_arc.clone();
            second_future.push(tokio::spawn(async move {
                let endpoint = std::env::var("META_ENDPOINT").expect("META_ENDPOINT is not set");
                let url = format!("{}/{}/build/{}", endpoint, champion_name, position);

                let res = client
                    .get(url)
                    .send()
                    .await
                    .expect("Failed to send request");

                let mut result = HashMap::<String, Vec<String>>::new();

                let html = res.text().await.expect("Could not read response text");
                let document = Html::parse_document(&html);
                let full_build = Selector::parse(".m-1q4a7cx:nth-of-type(4) > div > div img")
                    .expect("Failed to parse nested selector");
                let situational_build = Selector::parse(".m-s76v8c > div > div img")
                    .expect("Failed to parse selector .m-s76v8c");

                let mut items = Vec::<String>::new();
                let mut push_items = |selector: &Selector| {
                    for img in document.select(selector) {
                        if let Some(alt) = img.value().attr("alt") {
                            items.push(alt.to_string());
                        }
                    }
                };
                push_items(&full_build);
                push_items(&situational_build);
                result.insert(String::from(position), items);
                result
            }));
        }

        let mut collected_result = HashMap::new();
        for result in second_future {
            println!("Fetching meta items for {}", name);
            collected_result.extend(result.await.unwrap());
        }
        collected_results.insert(name, collected_result);
    }

    write_to_file(
        "src/internal/meta_items.json",
        serde_json::to_string(&collected_results)
            .unwrap()
            .as_bytes(),
    );
}

pub async fn update_instances(instance: &str) {
    let result = fetch_api(&format!("{}.json", instance)).await;

    for (key, value) in result {
        let folder_name = format!("cache/cdn/{}", instance);
        task::spawn_blocking(move || {
            let path_name = format!("{}/{}.json", folder_name, key);
            let strval = value.to_string();
            write_to_file(&path_name, strval.as_bytes());
        });
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
        "src/internal/items",
        "src/internal/champions",
        "src/internal/runes",
    ] {
        let path = Path::new(dir);
        if !path.exists() {
            let error_msg = format!("Unable to create directory '{}'", dir);

            fs::create_dir_all(path).expect(&error_msg);
        }
    }
}

// Helper function to write files
pub(super) fn write_to_file(path_name: &str, bytes: &[u8]) {
    let mut file = std::fs::File::create(path_name).expect("Unable to create file");
    file.write_all(bytes).expect("Unable to write data");
}

// Helper to read from files and parse the value to a struct
pub(super) fn read_from_file<T: DeserializeOwned>(path_name: &str) -> T {
    let data = fs::read_to_string(path_name).expect("Unable to read file");
    serde_json::from_str(&data).expect("Failed to parse JSON")
}

// Read every file in cache/cdn/champions folder and delegates the processing to generate_champion_file
pub fn setup_champion_cache() {
    let files =
        fs::read_dir("cache/cdn/champions").expect("Unable to read directory cache/cdn/champions");

    for file in files {
        let path_name = file.unwrap().path();
        task::spawn_blocking(move || {
            generate_champion_file(
                path_name
                    .to_str()
                    .expect("Failed to convert path to string"),
            );
        });
    }
}

// Not meant to be used frequently. Just a quick check for every
// patch to identify if a new damaging item was added
pub fn identify_damaging_items() {
    let contains_damage_outside_template = |text: &str| -> bool {
        let re = Regex::new(r"\{\{[^}]*\}\}").unwrap();
        let cleaned = re.replace_all(text, "");
        cleaned.contains("damage")
    };
    let files = fs::read_dir("cache/cdn/items").expect("Unable to read directory cache/cdn/items");
    let mut is_damaging = Vec::new();
    for file in files {
        let path_buf = file.unwrap().path();
        let path_name = path_buf.to_str().unwrap();
        let result = read_from_file::<CdnItem>(path_name);
        if !result.shop.purchasable {
            continue;
        }
        let mut found_match = false;
        if !result.passives.is_empty() {
            for passive in &result.passives {
                if contains_damage_outside_template(&passive.effects) {
                    found_match = true;
                }
            }
        }
        if !result.active.is_empty() {
            for active in &result.active {
                if contains_damage_outside_template(&active.effects) {
                    found_match = true;
                }
            }
        }
        if found_match {
            is_damaging.push(result.id);
        }
    }
    is_damaging.sort();
    write_to_file(
        "src/internal/damaging_items.json",
        serde_json::to_string_pretty(&is_damaging)
            .unwrap()
            .as_bytes(),
    );
}

// Replaces the content found in the files to a shorter and adapted version,
// initializes items as default, and Damaging stats must be added separately.
pub fn initialize_items() {
    let non_zero = |val: f64| -> Option<f64> { if val == 0.0 { None } else { Some(val) } };

    let files = fs::read_dir("cache/cdn/items").expect("Unable to read directory cache/cdn/items");
    for file in files {
        task::spawn_blocking(move || {
            let path_buf = file.unwrap().path();
            let path_name = path_buf.to_str().unwrap();
            let cdn_item = read_from_file::<CdnItem>(path_name);

            let stats = &cdn_item.stats;
            let mut item_stats = PartialStats::default();

            item_stats.ability_power = non_zero(stats.ability_power.flat);
            item_stats.armor = non_zero(stats.armor.flat);
            item_stats.attack_damage = non_zero(stats.attack_damage.flat);
            item_stats.attack_speed = non_zero(stats.attack_speed.flat);
            item_stats.critical_strike_chance = non_zero(stats.critical_strike_chance.flat);
            item_stats.critical_strike_damage = non_zero(stats.critical_strike_damage.flat);
            item_stats.health = non_zero(stats.health.flat);
            item_stats.lifesteal = non_zero(stats.lifesteal.flat);
            item_stats.magic_resistance = non_zero(stats.magic_resistance.flat);
            item_stats.mana = non_zero(stats.mana.flat);
            item_stats.movespeed = non_zero(stats.movespeed.flat);
            item_stats.omnivamp = non_zero(stats.omnivamp.flat);

            item_stats.armor_penetration_flat = non_zero(stats.armor_penetration.flat);
            item_stats.armor_penetration_percent = non_zero(stats.armor_penetration.percent);

            item_stats.magic_penetration_flat = non_zero(stats.magic_penetration.flat);
            item_stats.magic_penetration_percent = non_zero(stats.magic_penetration.percent);

            let result = Item {
                name: cdn_item.name,
                levelings: None,
                damage_type: None,
                stats: item_stats,
                builds_from: cdn_item.builds_from,
                ranged: None,
                melee: None,
            };

            write_to_file(
                format!("src/internal/items/{}.json", cdn_item.id).as_str(),
                serde_json::to_string(&result).unwrap().as_bytes(),
            );
        });
    }
}

// Uses champion display name and converts to their respective ids, saving to internal
pub fn rewrite_champion_names() {
    let files =
        fs::read_dir("cache/cdn/champions").expect("Unable to read directory cache/cdn/champions");

    let mut map = HashMap::<String, String>::new();

    for file in files {
        let path_buf = file.unwrap().path();
        let path_name = path_buf.to_str().unwrap();
        let result = read_from_file::<CdnChampion>(path_name);
        let name = extract_file_name(&path_buf);
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
    let name = extract_file_name(Path::new(path_name));

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
