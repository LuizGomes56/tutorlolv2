use super::{formulas::*, helpers::*, model::*, riot::*};
use smallvec::SmallVec;
use tinyset::SetU32;
use tutorlolv2_gen::{
    CHAMPION_NAME_TO_ID, DAMAGING_ITEMS, DAMAGING_RUNES, INTERNAL_CHAMPIONS, ItemId, RuneId,
};

pub fn realtime(game: &RiotRealtime) -> Option<()> {
    let mut size_counter = 0;

    let RiotRealtime {
        active_player:
            RiotActivePlayer {
                abilities,
                champion_stats,
                full_runes: RiotFullRunes { general_runes },
                level,
                riot_id,
            },
        all_players,
        events: RiotRealtimeEvents { events },
        game_data: RiotRealtimeGameData {
            game_time,
            map_number,
        },
    } = game;

    let current_player = all_players
        .iter()
        .find(|player| player.riot_id == *riot_id)?;

    let current_player_champion_id = CHAMPION_NAME_TO_ID.get(current_player.champion_name)?;
    let current_player_cache =
        unsafe { INTERNAL_CHAMPIONS.get_unchecked(*current_player_champion_id as usize) };

    let current_player_base_stats = {
        BasicStatsF32 {
            armor: get_base_stats(&current_player_cache.stats.armor, *level),
            health: get_base_stats(&current_player_cache.stats.health, *level),
            attack_damage: get_base_stats(&current_player_cache.stats.attack_damage, *level),
            magic_resist: get_base_stats(&current_player_cache.stats.magic_resistance, *level),
            mana: get_base_stats(&current_player_cache.stats.mana, *level),
        }
    };

    let current_playe_bonus_stats = {
        BasicStatsF32 {
            armor: champion_stats.armor - current_player_base_stats.armor,
            health: champion_stats.health - current_player_base_stats.health,
            attack_damage: champion_stats.attack_damage - current_player_base_stats.attack_damage,
            magic_resist: champion_stats.magic_resist - current_player_base_stats.magic_resist,
            mana: champion_stats.mana - current_player_base_stats.mana,
        }
    };

    let current_player_runes = general_runes
        .as_ref()
        .and_then(|gr| {
            Some(
                gr.into_iter()
                    .filter_map(|riot_rune| {
                        let rune_id = riot_rune.id;
                        DAMAGING_RUNES
                            .contains(&rune_id)
                            .then_some(RuneId::from_riot_id(rune_id) as u32)
                    })
                    .collect::<SetU32>(),
            )
        })
        .unwrap_or_default();

    let current_player_items = current_player
        .items
        .iter()
        .filter_map(|riot_item| {
            let item_id = riot_item.item_id;
            DAMAGING_ITEMS
                .contains(&item_id)
                .then_some(ItemId::from_riot_id(item_id) as u32)
        })
        .collect::<SetU32>();

    let (ally_dragons, enemy_earth_dragons) = get_dragons(&events, &all_players);

    let simulated_stats = get_simulated_stats(&champion_stats, ally_dragons);

    Some(())
}

fn get_dragons(events: &[RealtimeEvent], players_map: &[RiotAllPlayers]) -> (Dragons, u8) {
    let mut dragons = Dragons::default();
    let mut enemy_earth_dragons = 0;
    for event in events {
        if let (Some(killer), Some(dragon)) =
            (event.killer_name.as_deref(), event.dragon_type.as_deref())
        {
            match dragon {
                "Earth" => {
                    if players_map.iter().any(|player| player.riot_id == killer) {
                        dragons.earth += 1;
                    } else {
                        enemy_earth_dragons += 1;
                    }
                }
                "Fire" => dragons.fire += 1,
                _ => {}
            }
        }
    }
    (dragons, enemy_earth_dragons)
}
