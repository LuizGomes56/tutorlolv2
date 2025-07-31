use crate::model::base::AbilityLevels;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAbility {
    pub ability_level: u8,
}

#[derive(Deserialize)]
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

#[derive(Deserialize, Clone, Default)]
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
    pub id: u32,
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
    pub level: u8,
    pub riot_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotScoreboard {
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    pub creep_score: u16,
}

#[derive(Deserialize)]
pub struct RiotItems {
    #[serde(rename = "itemID")]
    pub item_id: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotAllPlayers {
    pub champion_name: String,
    pub items: Vec<RiotItems>,
    pub level: u8,
    pub position: String,
    pub riot_id: String,
    pub team: String,
    pub scores: RiotScoreboard,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtimeGameData {
    pub game_time: f32,
    pub map_number: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RealtimeEvent {
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
