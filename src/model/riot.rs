use crate::model::base::AbilityLevels;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAbility {
    pub ability_level: u8,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RiotAbilities {
    pub q: RiotAbility,
    pub w: RiotAbility,
    pub e: RiotAbility,
    pub r: RiotAbility,
}

impl RiotAbilities {
    pub fn get_levelings(&self) -> AbilityLevels {
        AbilityLevels {
            q: self.q.ability_level,
            w: self.w.ability_level,
            e: self.e.ability_level,
            r: self.r.ability_level,
        }
    }
}

#[derive(Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiotChampionStats {
    pub ability_power: f64,
    pub armor: f64,
    pub physical_lethality: f64,
    pub armor_penetration_percent: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub resource_max: f64,
    pub resource_value: f64,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotGeneralRunes {
    pub id: u32,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotFullRunes {
    pub general_runes: Option<SmallVec<[RiotGeneralRunes; 6]>>,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotActivePlayer<'a> {
    pub abilities: RiotAbilities,
    pub champion_stats: RiotChampionStats,
    pub full_runes: RiotFullRunes,
    pub level: u8,
    #[serde(borrow)]
    pub riot_id: &'a str,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotScoreboard {
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    pub creep_score: u16,
}

#[derive(Serialize, Default, Deserialize)]
pub struct RiotItems {
    #[serde(rename = "itemID")]
    pub item_id: u32,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAllPlayers<'a> {
    #[serde(borrow)]
    pub champion_name: &'a str,
    pub items: SmallVec<[RiotItems; 7]>,
    pub level: u8,
    #[serde(borrow)]
    pub position: &'a str,
    #[serde(borrow)]
    pub riot_id: &'a str,
    #[serde(borrow)]
    pub team: &'a str,
    pub scores: RiotScoreboard,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtimeGameData {
    pub game_time: f32,
    pub map_number: u8,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RealtimeEvent<'a> {
    pub dragon_type: Option<&'a str>,
    pub killer_name: Option<&'a str>,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RiotRealtimeEvents<'a> {
    #[serde(borrow)]
    pub events: SmallVec<[RealtimeEvent<'a>; 5]>,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtime<'a> {
    #[serde(borrow)]
    pub active_player: RiotActivePlayer<'a>,
    #[serde(borrow)]
    pub all_players: SmallVec<[RiotAllPlayers<'a>; 10]>,
    #[serde(borrow)]
    pub events: RiotRealtimeEvents<'a>,
    pub game_data: RiotRealtimeGameData,
}

#[test]
fn _test() {
    unsafe {
        std::thread::Builder::new()
            .stack_size(1 << 23)
            .spawn_unchecked(move || {
                let data = std::fs::read_to_string("example.json").unwrap();
                let parsed = serde_json::from_str(&data).unwrap();
                let start_time = std::time::Instant::now();
                let _game = crate::services::realtime::realtime(&parsed).map_err(|e| {
                    println!("{:?}", e);
                });
                println!("{:?}", start_time.elapsed());
            })
    }
    .unwrap()
    .join()
    .unwrap();
}
