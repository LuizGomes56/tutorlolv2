use super::schemas::APIResponse;
use crate::model::riot::RiotRealtime;
use crate::services::calculator::calculator;
use crate::services::realtime::realtime;
use crate::{AppState, model::calculator::GameX};
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
    simulated_items: Vec<usize>,
}

#[derive(Deserialize)]
struct CalculatorBody {
    game: GameX,
    simulated_items: Vec<usize>,
}

#[post("/api/calculator")]
pub async fn calculator_handler(
    state: Data<AppState>,
    body: Json<CalculatorBody>,
) -> impl Responder {
    let CalculatorBody {
        game,
        simulated_items,
    } = body.into_inner();
    match calculator(&state.cache, &game, &simulated_items) {
        Ok(data) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: (),
            data,
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: e,
            data: (),
        }),
    }
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

            match realtime(&state.cache, &game, &body.simulated_items) {
                Ok(data) => HttpResponse::Ok().json(APIResponse {
                    success: true,
                    message: (),
                    data,
                }),
                Err(e) => HttpResponse::InternalServerError().json(APIResponse {
                    success: false,
                    message: e,
                    data: (),
                }),
            }
        }
        Err(e) => HttpResponse::NotFound().json(APIResponse {
            success: false,
            message: "No registers were found in the database.".to_string(),
            data: e.to_string(),
        }),
    }
}
