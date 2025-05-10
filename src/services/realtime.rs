use crate::model::realtime::GameData;

pub async fn calculate(_game: &GameData, _rec: bool, _item: String) -> GameData {
    GameData {
        summoner_name: "testing".to_string(),
    }
}
