use std::{fs, path};

use rustc_hash::FxHashMap;
use tutorlolv2::{
    services::eval::MathEval,
    writers::{self, CdnChampion, Champion},
};

include!(concat!(env!("OUT_DIR"), "/writers_generated.rs"));

const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn test_string(target_str: &str) -> String {
    let replacements = [
        "CHOGATH_STACKS",
        "VEIGAR_STACKS",
        "NASUS_STACKS",
        "SMOLDER_STACKS",
        "AURELION_SOL_STACKS",
        "THRESH_STACKS",
        "KINDRED_STACKS",
        "BELVETH_STACKS",
        "ADAPTATIVE_DAMAGE",
        "MISSING_HEALTH",
        "LEVEL",
        "PHYSICAL_MULTIPLIER",
        "MAGIC_MULTIPLIER",
        "STEELCAPS_EFFECT",
        "RANDUIN_EFFECT",
        "ROCKSOLID_EFFECT",
        "ENEMY_BONUS_HEALTH",
        "ENEMY_ARMOR",
        "ENEMY_HEALTH",
        "ENEMY_MAX_HEALTH",
        "ENEMY_CURRENT_HEALTH",
        "ENEMY_MISSING_HEALTH",
        "ENEMY_MAGIC_RESIST",
        "BASE_HEALTH",
        "BASE_AD",
        "BASE_ARMOR",
        "BASE_MAGIC_RESIST",
        "BASE_MANA",
        "BONUS_AD",
        "BONUS_ARMOR",
        "BONUS_MAGIC_RESIST",
        "BONUS_HEALTH",
        "BONUS_MANA",
        "BONUS_MOVE_SPEED",
        "AP",
        "AD",
        "ARMOR_PENETRATION_FLAT",
        "ARMOR_PENETRATION_PERCENT",
        "MAGIC_PENETRATION_FLAT",
        "MAGIC_PENETRATION_PERCENT",
        "MAX_MANA",
        "CURRENT_MANA",
        "MAX_HEALTH",
        "CURRENT_HEALTH",
        "ARMOR",
        "MAGIC_RESIST",
        "CRIT_CHANCE",
        "CRIT_DAMAGE",
        "ATTACK_SPEED",
    ];
    replacements
        .iter()
        .fold(target_str.to_string(), |acc: String, old| {
            acc.replace(old, "100.0")
        })
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let fetch_remote_url: Option<String> = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--fetch-remote=").map(|s| s.to_string()));

    let fx_hashmap: FxHashMap<String, CdnChampion> = if let Some(ref url) = fetch_remote_url {
        println!("[FETCHING]");
        reqwest::get(url).await.unwrap().json().await.unwrap()
    } else {
        let mut map = FxHashMap::default();
        for dir_entry in fs::read_dir("cache/cdn/champions").unwrap() {
            let path: path::PathBuf = dir_entry.unwrap().path();
            let file_name: &str = path.to_str().unwrap();
            let content = fs::read_to_string(file_name).unwrap();
            let result: CdnChampion = serde_json::from_str(&content).unwrap();
            map.insert(result.name.clone(), result);
        }
        map
    };

    let mut eval_errors = Vec::new();
    for (id, val) in fx_hashmap.into_iter() {
        if let Some(chn_data) = try_transform(id.to_lowercase().as_str(), val) {
            for (ability_key, ability_val) in chn_data.abilities.into_iter() {
                macro_rules! check_eval {
                    ($field:ident) => {{
                        for damage in ability_val.$field.into_iter() {
                            if test_string(&damage).eval().is_err() {
                                eval_errors.push(format!(
                                    "{}{}{} - {}::{}",
                                    CYAN,
                                    id,
                                    RESET,
                                    ability_key,
                                    stringify!($field)
                                ));
                                break;
                            }
                        }
                    }};
                }
                check_eval!(minimum_damage);
                check_eval!(maximum_damage);
            }
        }
    }

    if eval_errors.len() > 0 {
        println!("{RED}Found {} errors{RESET}", eval_errors.len());
        for err in &eval_errors {
            println!("{CYAN}→ {err}{RESET}");
        }
        panic!(
            "{RED}Validation failed on {} ability fields.{RESET}",
            eval_errors.len()
        );
    } else {
        println!("{GREEN}✓ All validations passed!{RESET}");
    }
}
