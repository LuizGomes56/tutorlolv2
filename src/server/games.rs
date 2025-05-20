use super::schemas::APIResponse;
use crate::AppState;
use crate::model::riot::RiotRealtime;
use crate::services::realtime::{CalculateError, calculate};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
struct RealtimeResponse {
    game_id: String,
    summoner_name: String,
    game_code: String,
    champion_name: Option<String>,
    game: String,
}

#[derive(Deserialize)]
struct RealtimeBody {
    code: String,
    item: String,
}

#[post("/api/realtime")]
pub async fn realtime_handler(state: Data<AppState>, body: Json<RealtimeBody>) -> impl Responder {
    match sqlx::query_as::<_, RealtimeResponse>(
        "SELECT g.game_id, g.summoner_name, 
        g.game_code, g.champion_name, 
        gd.game_data as game FROM games g
        JOIN game_data gd ON g.game_id = gd.game_id
        WHERE game_code = $1
        ORDER BY game_time DESC LIMIT 1",
    )
    .bind(&body.code)
    .fetch_one(&state.db)
    .await
    {
        Ok(val) => {
            let game: RiotRealtime =
                serde_json::from_str(&val.game).expect("Failed to parse game data");

            let data = calculate(&state.cache, &game, body.item.clone());

            match data {
                Ok(data) => HttpResponse::Ok().json(APIResponse {
                    success: true,
                    message: (),
                    data,
                }),
                Err(e) => match e {
                    CalculateError::ActivePlayerNotFound => HttpResponse::InternalServerError()
                        .json(APIResponse {
                            success: false,
                            message: "Active player not found.".to_string(),
                            data: (),
                        }),
                    CalculateError::ChampionCacheNotFound(id) => {
                        HttpResponse::InternalServerError().json(APIResponse {
                            success: false,
                            message: format!("Champion cache not found for id: {}", id),
                            data: (),
                        })
                    }
                    CalculateError::ChampionNameNotFound(name) => {
                        HttpResponse::InternalServerError().json(APIResponse {
                            success: false,
                            message: format!("Champion name not found for name: {}", name),
                            data: (),
                        })
                    }
                    CalculateError::MetaItemsNotFound(name) => HttpResponse::InternalServerError()
                        .json(APIResponse {
                            success: false,
                            message: format!("Meta items not found for name: {}", name),
                            data: (),
                        }),
                },
            }
        }
        Err(e) => HttpResponse::NotFound().json(APIResponse {
            success: false,
            message: "No registers were found in the database.".to_string(),
            data: e.to_string(),
        }),
    }
}
