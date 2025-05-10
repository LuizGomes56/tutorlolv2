use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GameData {
    pub summoner_name: String,
}
