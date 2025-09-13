use crate::{AppState, send_response};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Bytes, Data},
};
use bincode::{Decode, Encode};
use rand::random_range;
use sqlx::prelude::FromRow;
use tutorlolv2_math::{
    math::{calculator::calculator, realtime::realtime},
    model::{Sizer, calculator::InputGame, riot::RiotRealtime},
};
use uuid::Uuid;

const CODE_LENGTH: usize = 6;

macro_rules! send_error {
    ($msg:expr) => {{
        HttpResponse::InternalServerError()
            .insert_header((crate::header::CONTENT_TYPE, "text/plain"))
            .body($msg)
    }};
}

#[get("/create")]
pub async fn create_game_handler(state: Data<AppState>) -> impl Responder {
    const BODY_LENGTH: usize = 16 + CODE_LENGTH;
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut game_code_bytes = [0; CODE_LENGTH];
    unsafe {
        for i in 0..CODE_LENGTH {
            *game_code_bytes.get_unchecked_mut(i) = chars[random_range(0..chars.len())];
        }
        let game_code = std::str::from_utf8_unchecked(&game_code_bytes);
        let uuid_v4 = Uuid::new_v4();
        let game_id = uuid_v4.into_bytes();
        let body = Bytes::from({
            let mut out = [0u8; BODY_LENGTH];
            out[..16].copy_from_slice(&game_id);
            out[16..].copy_from_slice(&game_code_bytes[..]);
            out.to_vec()
        });
        match sqlx::query("INSERT INTO games (game_id, game_code) VALUES ($1, $2)")
            .bind(&game_id)
            .bind(game_code)
            .execute(&state.db)
            .await
        {
            Ok(_) => HttpResponse::Ok()
                .insert_header((crate::header::CONTENT_TYPE, "application/octet-stream"))
                .insert_header((crate::header::CONTENT_LENGTH, BODY_LENGTH))
                .body(body),
            Err(e) => send_error!(format!("Error when inserting game_data: {}", e)),
        }
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
                        Ok(realtime_data) => send_response!(&realtime_data),
                        Err(e) => send_error!(e.as_str()),
                    },
                    Err(e) => send_error!(format!(
                        "Unable to parse database response into RealtimeSqlxQuery type: {:#?}",
                        e
                    )),
                },
                Err(e) => send_error!(format!("Querying data by code failed: {:#?}", e)),
            }
        }
        Err(e) => send_error!(format!("Error deserializing data: {:#?}", e)),
    }
}

#[post("/realtime")]
pub async fn realtime_handler(body: actix_web::web::Bytes) -> impl Responder {
    match serde_json::from_slice(&body) {
        Ok(game_data) => match realtime(&game_data) {
            Ok(data) => send_response!(data),
            Err(e) => send_error!(e.as_str()),
        },
        Err(e) => send_error!(format!("Error deserializing json data: {:#?}", e)),
    }
}

// #[post("/realtime")]
pub async fn _realtime_handler(
    // state: Data<AppState>,
    body: actix_web::web::Bytes,
) -> impl Responder {
    #[derive(bincode::BorrowDecode)]
    struct RealtimeBody<'a> {
        game_id: &'a str,
        game_code: &'a str,
        game_data: &'a [u8],
    }

    match bincode::borrow_decode_from_slice::<'_, RealtimeBody, _>(
        &body,
        bincode::config::standard(),
    ) {
        Ok(decoded) => {
            let RealtimeBody {
                game_id,
                game_code,
                game_data,
            } = decoded.0;

            println!("game_id: {game_id}, game_code: {game_code}");
            match serde_json::from_slice(game_data) {
                Ok(game_data) => match realtime(&game_data) {
                    Ok(data) => send_response!(data),
                    Err(e) => send_error!(e.as_str()),
                },
                Err(e) => send_error!(format!("Error deserializing json data: {:#?}", e)),
            }

            // ! Database should have a BYTEA field (current = JSONB)

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
                            let message = e.as_str();
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
        Err(e) => send_error!(format!("Error deserializing bincode: {:#?}", e)),
    }
}

#[post("/calculator")]
pub async fn calculator_handler(body: actix_web::web::Bytes) -> impl Responder {
    match bincode::decode_from_slice::<InputGame, _>(&body, bincode::config::standard()) {
        Ok(decoded) => match calculator(decoded.0) {
            Ok(data) => {
                let size = data.size();
                let mut buf = Box::<[u8]>::new_uninit_slice(size);
                let raw =
                    unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, size) };
                match bincode::encode_into_slice(&data, raw, bincode::config::standard()) {
                    Ok(_) => {
                        let init = unsafe { buf.assume_init() };
                        HttpResponse::Ok()
                            .insert_header((
                                crate::header::CONTENT_TYPE,
                                "application/octet-stream",
                            ))
                            .body(actix_web::web::Bytes::from(init))
                    }
                    Err(e) => send_error!(format!("Error encoding bincode: {:#?}", e)),
                }
            }
            Err(e) => send_error!(e.as_str()),
        },
        Err(e) => send_error!(format!("Error deserializing data: {:#?}", e)),
    }
}
