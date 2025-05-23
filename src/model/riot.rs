#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct RiotCDNStandard {
    pub data: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct RiotCDNItem {
    pub description: String,
}

#[derive(Deserialize)]
pub struct RiotCDNImage {
    pub full: String,
}

#[derive(Deserialize)]
pub struct RiotCDNInstance {
    pub image: RiotCDNImage,
}

#[derive(Deserialize)]
pub struct RiotCDNRuneTree {
    pub icon: String,
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize)]
pub struct RiotCDNRuneSlot {
    pub runes: Vec<RiotCDNRuneTree>,
}

#[derive(Deserialize)]
pub struct RiotCDNRune {
    pub icon: String,
    pub id: usize,
    pub name: String,
    pub slots: Vec<RiotCDNRuneSlot>,
}

#[derive(Deserialize)]
pub struct RiotCDNSkin {
    pub num: usize,
}

#[derive(Deserialize)]
pub struct RiotCDNChampion {
    pub id: String,
    pub image: RiotCDNImage,
    pub passive: RiotCDNInstance,
    pub spells: Vec<RiotCDNInstance>,
    pub skins: Vec<RiotCDNSkin>,
}

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

#[derive(Deserialize, Default)]
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
#[serde(rename_all = "camelCase")]
pub struct RiotScoreboard {
    pub kills: usize,
    pub deaths: usize,
    pub assists: usize,
    pub creep_score: usize,
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
    pub scores: RiotScoreboard,
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
    pub dragon_type: Option<String>,
    pub killer_name: Option<String>,
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
