use std::sync::Arc;

use crate::model::{application::GlobalCache, realtime::CurrentPlayer, riot::RiotRealtime};

// Initialize a global cache that hold the content in src/internal/*.json
// All the required data to perform calculations may be initialized on app start.

// Return a massive struct object that will be sent to browser. It will have every
// necessary field to make up tables and display the game data.
pub async fn calculate(cache: &Arc<GlobalCache>, game: &RiotRealtime, _item: String) {
    let active_player = &game.active_player;
    let all_players = &game.all_players;

    let active_player_level = active_player.level;

    let active_player_expanded = all_players
        .iter()
        .find(|player| player.riot_id == active_player.riot_id)
        .expect("Couldn't find active player in all players list");

    // let current_player = CurrentPlayer {
    //     riot_id: active_player.riot_id.to_string(),
    // }
}
