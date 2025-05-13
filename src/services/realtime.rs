use crate::model::realtime::GameData;

// Return a massive struct object that will be sent to browser. It will have every
// necessary field to make up tables and display the game data.
pub async fn calculate(_game: &GameData, _rec: bool, _item: String) -> GameData {
    GameData {
        summoner_name: "testing".to_string(),
    }
}
