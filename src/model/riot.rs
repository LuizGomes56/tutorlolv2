#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAbility {
    pub ability_level: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RiotAbilities {
    pub q: RiotAbility,
    pub w: RiotAbility,
    pub e: RiotAbility,
    pub r: RiotAbility,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotGeneralRunes {
    pub id: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotFullRunes {
    pub general_runes: Vec<RiotGeneralRunes>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotActivePlayer {
    pub abilities: RiotAbilities,
    pub champion_stats: RiotChampionStats,
    pub full_runes: RiotFullRunes,
    pub level: usize,
    pub riot_id: String,
}

#[derive(Deserialize)]
pub struct RiotItems {
    #[serde(rename = "itemID")]
    pub item_id: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAllPlayers {
    pub champion_name: String,
    pub items: Vec<RiotItems>,
    pub level: usize,
    pub position: String,
    pub riot_id: String,
    pub team: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtimeGameData {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RealtimeEvent {
    pub event_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RiotRealtimeEvents {
    pub events: Vec<RealtimeEvent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtime {
    pub active_player: RiotActivePlayer,
    pub all_players: Vec<RiotAllPlayers>,
    pub events: RiotRealtimeEvents,
    pub game_data: RiotRealtimeGameData,
}
