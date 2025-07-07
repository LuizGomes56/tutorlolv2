use super::schemas::APIResponse;
use crate::{
    AppState,
    model::{calculator::InputGame, riot::RiotRealtime},
    send_response,
    services::{CalculationError, calculator::calculator, realtime::realtime},
};
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
    game_code: String,
    game_id: String,
}

#[get("/create")]
pub async fn create_game_handler(state: Data<AppState>) -> impl Responder {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"
        .chars()
        .collect::<Vec<char>>();
    let game_code = (0..6)
        .map(|_| chars[random_range(0..chars.len())])
        .collect::<String>();
    let game_id = Uuid::new_v4().to_string();
    match sqlx::query("INSERT INTO games (game_id, game_code) VALUES ($1, $2)")
        .bind(&game_id)
        .bind(&game_code)
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
    game_code: String,
}

#[inline(always)]
fn get_calculation_error(e: CalculationError) -> String {
    match e {
        CalculationError::CurrentPlayerNotFound => {
            String::from("Current player not found in allPlayers")
        }
        CalculationError::ChampionNameNotFound(n) => format!(
            "Could not convert champion name to its corresponding id: {}",
            n
        ),
        CalculationError::ChampionCacheNotFound(n) => {
            format!("Current champion cache not found: {}", n)
        }
        CalculationError::ItemCacheNotFound(n) => {
            format!("One item's cache was not found: {}", n)
        }
    }
}

#[post("/get_by_code")]
pub async fn get_by_code_handler(
    state: Data<AppState>,
    body: Json<GetByCodeBody>,
) -> impl Responder {
    let GetByCodeBody { game_code } = body.into_inner();

    match sqlx::query_as::<_, RealtimeSqlxQuery>(
        "SELECT g.game_id, gd.game_data AS game 
        FROM games g
        JOIN game_data gd ON gd.game_id = g.game_id 
        WHERE g.game_code = $1
        ORDER BY gd.game_time DESC 
        LIMIT 1",
    )
    .bind(game_code)
    .fetch_one(&state.db)
    .await
    {
        Ok(data) => match serde_json::from_str::<RiotRealtime>(&data.game) {
            Ok(riot_realtime) => match realtime(&riot_realtime) {
                Ok(realtime_data) => {
                    send_response!(&realtime_data)
                }
                Err(e) => {
                    let message = get_calculation_error(e);
                    HttpResponse::InternalServerError().json(APIResponse {
                        success: false,
                        message,
                        data: (),
                    })
                }
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
    game_code: String,
    game_data: String,
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
                    match realtime(&realtime_data) {
                        Ok(data) => {
                            println!(
                                "fn[realtime_handler] Success on request for game_code: {}, game_id: {}",
                                game_code,
                                game_id
                            );
                            send_response!(&data)
                        }
                        Err(e) => {
                            let message = get_calculation_error(e);
                            println!("Error on realtime response: {:#?}", message);
                            HttpResponse::InternalServerError().json(APIResponse {
                                success: false,
                                message,
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
    game: InputGame,
}

#[post("/calculator")]
pub async fn calculator_handler(body: Json<CalculatorBody>) -> impl Responder {
    let CalculatorBody { game } = body.into_inner();
    match calculator(game) {
        Ok(data) => {
            send_response!(&data)
        }
        Err(e) => HttpResponse::InternalServerError().json({
            let message = get_calculation_error(e);
            APIResponse {
                success: false,
                message,
                data: (),
            }
        }),
    }
}
