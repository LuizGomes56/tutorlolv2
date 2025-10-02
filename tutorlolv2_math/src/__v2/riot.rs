use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use tinyset::SetU32;
use tutorlolv2_gen::{DAMAGING_ITEMS, ItemId};

use crate::__v2::model::{AbilityLevels, L_ITEM, L_PLYR};

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
    #[inline(always)]
    pub fn get_levelings(&self) -> AbilityLevels {
        AbilityLevels {
            q: self.q.ability_level,
            w: self.w.ability_level,
            e: self.e.ability_level,
            r: self.r.ability_level,
        }
    }
}

#[derive(Serialize, Default, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiotChampionStats {
    pub ability_power: f32,
    pub armor: f32,
    #[serde(rename = "physicalLethality")]
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub current_health: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_resist: f32,
    #[serde(rename = "maxHealth")]
    pub health: f32,
    #[serde(rename = "resourceMax")]
    pub mana: f32,
    #[serde(rename = "resourceValue")]
    pub current_mana: f32,
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
    pub kills: u8,
    pub deaths: u8,
    pub assists: u8,
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
    pub items: SmallVec<[RiotItems; L_ITEM]>,
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
    pub events: Vec<RealtimeEvent<'a>>,
}

#[derive(Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotRealtime<'a> {
    #[serde(borrow)]
    pub active_player: RiotActivePlayer<'a>,
    #[serde(borrow)]
    pub all_players: SmallVec<[RiotAllPlayers<'a>; L_PLYR]>,
    #[serde(borrow)]
    pub events: RiotRealtimeEvents<'a>,
    pub game_data: RiotRealtimeGameData,
}
