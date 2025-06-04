use super::schemas::APIResponse;
use crate::model::riot::RiotRealtime;
use crate::services::calculator::calculator;
use crate::services::realtime::realtime;
use crate::{AppState, model::calculator::GameX};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Json},
};
use rand::random_range;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize)]
struct CreateGameResponse {
    game_code: i32,
    game_id: String,
}

#[get("/create")]
pub async fn create_game_handler(state: Data<AppState>) -> impl Responder {
    let game_code = random_range(100_000..999_999);
    let game_id = Uuid::new_v4().to_string();
    match sqlx::query("INSERT INTO games (game_id, game_code) VALUES ($1, $2)")
        .bind(&game_id)
        .bind(game_code)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            success: true,
            message: "Game created succesfully",
            data: CreateGameResponse { game_code, game_id },
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Error when inserting game_data: {}", e),
            data: (),
        }),
    }
}

#[derive(Deserialize)]
struct GetByCodeBody {
    game_code: usize,
    simulated_items: Vec<usize>,
}

#[post("/get_by_code")]
pub async fn get_by_code_handler(
    state: Data<AppState>,
    body: Json<GetByCodeBody>,
) -> impl Responder {
    let GetByCodeBody {
        game_code,
        simulated_items,
    } = body.into_inner();

    match sqlx::query_as::<_, RealtimeSqlxQuery>(
        "SELECT g.game_id, gd.game_data AS game 
        FROM games g
        JOIN game_data gd ON gd.game_id = g.game_id 
        WHERE g.game_code = $1
        ORDER BY gd.game_time DESC 
        LIMIT 1",
    )
    .bind(game_code.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(data) => match serde_json::from_str::<RiotRealtime>(&data.game) {
            Ok(riot_realtime) => match realtime(&state.cache, &riot_realtime, &simulated_items) {
                Ok(realtime_data) => HttpResponse::Ok().json(APIResponse {
                    success: true,
                    message: "Game data fetched successfully",
                    data: realtime_data,
                }),
                Err(e) => HttpResponse::InternalServerError().json(APIResponse {
                    success: false,
                    message: format!("realtime calculation failed: {}", e),
                    data: (),
                }),
            },
            Err(e) => HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: format!(
                    "Unable to parse database response into RealtimeSqlxQuery type: {}",
                    e
                ),
                data: (),
            }),
        },
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Querying data by code failed: {}", e),
            data: (),
        }),
    }
}

#[derive(Deserialize)]
struct RealtimeBody {
    game_id: String,
    game_code: i32,
    game_data: String,
    simulated_items: Vec<usize>,
}

#[derive(Serialize, FromRow)]
struct RealtimeSqlxQuery {
    game: String,
}

#[post("/realtime")]
pub async fn realtime_handler(state: Data<AppState>, body: Json<RealtimeBody>) -> impl Responder {
    let RealtimeBody {
        game_id,
        game_code,
        game_data,
        simulated_items,
    } = body.into_inner();
    match serde_json::from_str::<RiotRealtime>(&game_data) {
        Ok(realtime_data) => {
            let game_time = realtime_data.game_data.game_time;
            let summoner_name = realtime_data.active_player.riot_id.clone();
            let champion_name = realtime_data
                .all_players
                .iter()
                .find(|p| p.riot_id == summoner_name)
                .map(|p| p.champion_name.clone())
                .unwrap_or_default();

            println!(
                "Found champion_name: {}, and summoner_name: {}",
                champion_name, summoner_name
            );

            match sqlx::query(
                "INSERT INTO game_data (game_id, game_data, champion_name, game_time, summoner_name)
                VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&game_id)
            .bind(game_data)
            .bind(champion_name)
            .bind(game_time)
            .bind(summoner_name)
            .execute(&state.db)
            .await {
                Ok(_) => {
                    match realtime(&state.cache, &realtime_data, &simulated_items) {
                        Ok(data) => {
                            let message = format!("Success on request for game_code: {}, game_id: {}", game_code, game_id);
                            println!("realtime_handler: {}", message);
                            HttpResponse::Ok().json(APIResponse {
                                success: true,
                                message,
                                data,
                            })
                        }
                        Err(e) => {
                            println!("Error on realtime response: {:#?}", e);
                            HttpResponse::InternalServerError().json(APIResponse {
                                success: false,
                                message: e,
                                data: (),
                            })
                        }
                    }
                }
                Err(e) => {
                    HttpResponse::InternalServerError().json(APIResponse {
                        success: false,
                        message: format!("Error when inserting game_data: {}", e),
                        data: (),
                    })
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().json(APIResponse {
            success: false,
            message: format!("Invalid JSON: {}", e),
            data: (),
        }),
    }
}

#[derive(Deserialize)]
struct CalculatorBody {
    game: GameX,
    simulated_items: Vec<usize>,
}

#[post("/calculator")]
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
