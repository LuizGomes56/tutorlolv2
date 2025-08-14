use super::schemas::APIResponse;
use crate::{
    AppState,
    model::{calculator::InputGame, riot::RiotRealtime},
    send_response,
    services::{CalculationError, calculator::calculator, realtime::realtime},
};
use actix_web::{HttpResponse, Responder, get, post, web::Data};
use bincode::{Decode, Encode};
use rand::random_range;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[get("/create")]
pub async fn create_game_handler(state: Data<AppState>) -> impl Responder {
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut game_code_bytes = [0; 6];
    for i in 0..6 {
        game_code_bytes[i] = chars[random_range(0..chars.len())];
    }
    let game_code = unsafe { std::str::from_utf8_unchecked(&game_code_bytes) };
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
            data: format!(r#"{{"game_code": {game_code},"game_id": {game_id}}}"#),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            success: false,
            message: format!("Error when inserting game_data: {}", e),
            data: (),
        }),
    }
}

#[inline(always)]
fn get_calculation_error(e: CalculationError) -> &'static str {
    match e {
        CalculationError::CurrentPlayerNotFound => "Current player not found in allPlayers",
        CalculationError::ChampionNameNotFound => {
            "Could not convert champion name to its corresponding id"
        }
        CalculationError::ChampionCacheNotFound => "Current champion cache not found",
    }
}

#[derive(Decode)]
struct GetByCodeBody {
    game_code: String,
}

#[post("/get_by_code")]
pub async fn get_by_code_handler(
    state: Data<AppState>,
    body: actix_web::web::Bytes,
) -> impl Responder {
    #[cfg(not(feature = "dev"))]
    return HttpResponse::ServiceUnavailable();
    #[cfg(feature = "dev")]
    match bincode::decode_from_slice::<GetByCodeBody, _>(&body, bincode::config::standard()) {
        Ok(decoded) => {
            #[derive(Encode, FromRow)]
            struct RealtimeSqlxQuery {
                game: String,
            }

            match sqlx::query_as::<_, RealtimeSqlxQuery>(
                "SELECT g.game_id, gd.game_data AS game 
                FROM games g
                JOIN game_data gd ON gd.game_id = g.game_id 
                WHERE g.game_code = $1
                ORDER BY gd.game_time DESC 
                LIMIT 1",
            )
            .bind(decoded.0.game_code)
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
        Err(e) => {
            eprintln!("Error deserializing bincode: {}", e);
            HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: format!("Error deserializing data: {:#?}", e),
                data: (),
            })
        }
    }
}

#[post("/realtime")]
pub async fn realtime_handler(
    // state: Data<AppState>,
    body: actix_web::web::Bytes,
) -> impl Responder {
    #[derive(Decode)]
    struct RealtimeBody {
        game_id: String,
        game_code: String,
        game_data: Vec<u8>,
    }

    match bincode::decode_from_slice::<RealtimeBody, _>(&body, bincode::config::standard()) {
        Ok(decoded) => {
            let RealtimeBody {
                game_id,
                game_code,
                game_data,
            } = decoded.0;
            // let game_time = game_data.game_data.game_time;
            // let summoner_name = game_data.active_player.riot_id.clone();
            // let champion_name = game_data
            //     .all_players
            //     .iter()
            //     .find(|p| p.riot_id == summoner_name)
            //     .map(|p| p.champion_name.clone())
            //     .unwrap_or_default();

            // println!(
            //     "Found champion_name: {}, and summoner_name: {}",
            //     champion_name, summoner_name
            // );

            match serde_json::from_slice(&game_data) {
                Ok(game_data) => match realtime(&game_data) {
                    Ok(data) => send_response!(data),
                    Err(e) => {
                        let message = get_calculation_error(e);
                        println!("Error on realtime response: {:#?}", message);
                        HttpResponse::InternalServerError().json(APIResponse {
                            success: false,
                            message,
                            data: (),
                        })
                    }
                },
                Err(e) => {
                    eprintln!("Error deserializing bincode: {}", e);
                    HttpResponse::InternalServerError().json(APIResponse {
                        success: false,
                        message: format!("Error deserializing data: {:#?}", e),
                        data: (),
                    })
                }
            }

            // ! Database should have a BYTEA field (current = JSONB)

            /*
            match sqlx::query(
                "INSERT INTO game_data (game_id, game_data, champion_name, game_time, summoner_name)
                VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&game_id)
            .bind(&body.to_vec())
            .bind(champion_name)
            .bind(game_time)
            .bind(summoner_name)
            .execute(&state.db)
            .await {
                Ok(_) => {
                    match realtime(&game_data) {
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
            */
        }
        Err(e) => {
            eprintln!("Error deserializing bincode: {}", e);
            return HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: format!("Error deserializing data: {:#?}", e),
                data: (),
            });
        }
    }
}

#[post("/calculator")]
pub async fn calculator_handler(body: actix_web::web::Bytes) -> impl Responder {
    match bincode::decode_from_slice::<InputGame, _>(&body, bincode::config::standard()) {
        Ok(decoded) => match calculator(decoded.0) {
            Ok(data) => {
                send_response!(&data)
            }
            Err(e) => HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: get_calculation_error(e),
                data: (),
            }),
        },
        Err(e) => {
            eprintln!("Error deserializing bincode: {}", e);
            HttpResponse::InternalServerError().json(APIResponse {
                success: false,
                message: format!("Error deserializing data: {:#?}", e),
                data: (),
            })
        }
    }
}
