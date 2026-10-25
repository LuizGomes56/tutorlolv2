use actix_web::{
    HttpResponse,
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::Bytes,
};
use bincode::{Decode, Encode, config::Config};
use tutorlolv2::{calculator, model::OwnedInputGame, riot::RiotRealtime};

type Response = actix_web::Result<HttpResponse>;
const OCTET_STREAM: &str = "application/octet-stream";

fn request_config() -> impl Config {
    bincode::config::standard().with_limit::<1_048_576>()
}

fn respond(data: impl Encode) -> Response {
    let bytes = bincode::encode_to_vec(data, bincode::config::standard())
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type(OCTET_STREAM).body(bytes))
}

fn decode_request<T: Decode<()>>(body: &[u8]) -> actix_web::Result<T> {
    let (value, consumed) =
        bincode::decode_from_slice(body, request_config()).map_err(ErrorBadRequest)?;

    if consumed != body.len() {
        return Err(ErrorBadRequest("Unexpected trailing request bytes"));
    }

    Ok(value)
}

#[post("/realtime")]
pub async fn realtime_handler(body: Bytes) -> Response {
    let (game, consumed): (RiotRealtime<'_>, _) =
        bincode::borrow_decode_from_slice(&body, request_config()).map_err(ErrorBadRequest)?;

    if consumed != body.len() {
        return Err(ErrorBadRequest("Unexpected trailing request bytes"));
    }

    let data = tutorlolv2::realtime::realtime(&game).map_err(|e| ErrorBadRequest(e.to_string()))?;

    respond(data)
}

#[post("/calculator")]
pub async fn calculator_handler(body: Bytes) -> Response {
    let request: OwnedInputGame = decode_request(&body)?;
    let prepared = request.prepare();

    respond(calculator::calculator(prepared.as_input()))
}

/*
const CODE_LENGTH: usize = 6;
const BODY_LENGTH: usize = 16 + CODE_LENGTH;
static CODE_CHARS: &[u8; 62] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

#[get("/create")]
pub async fn create_game_handler(state: Data<AppState>) -> Response {
    let mut game_code_bytes = [0; CODE_LENGTH];

    unsafe {
        for i in 0..CODE_LENGTH {
            *game_code_bytes.get_unchecked_mut(i) = CODE_CHARS[random_range(0..CODE_CHARS.len())];
        }

        let game_code = std::str::from_utf8_unchecked(&game_code_bytes);
        let uuid_v4 = Uuid::new_v4();
        let game_id = uuid_v4.into_bytes();

        let mut buffer = [0; BODY_LENGTH];
        buffer.get_unchecked_mut(..16).copy_from_slice(&game_id);
        buffer
            .get_unchecked_mut(16..)
            .copy_from_slice(game_code_bytes.get_unchecked(..));

        sqlx::query("INSERT INTO games (game_id, game_code) VALUES ($1, $2)")
            .bind(game_id)
            .bind(game_code)
            .execute(&state.db)
            .await?;

        Ok(HttpResponse::Ok()
            .insert_header(OCTET_STREAM)
            .body(Bytes::from(Box::<[u8]>::from(buffer))))
    }
}

#[post("/get_by_code")]
pub async fn get_by_code_handler(state: Data<AppState>, body: actix_web::web::Bytes) -> Response {
    let game_code = std::str::from_utf8(body.as_ref())?;
    let (data,) = sqlx::query_as::<_, (String,)>(
        "SELECT g.game_id, gd.game_data AS game
        FROM games g
        JOIN game_data gd ON gd.game_id = g.game_id
        WHERE g.game_code = $1
        ORDER BY gd.game_time DESC
        LIMIT 1",
    )
    .bind(game_code)
    .fetch_one(&state.db)
    .await?;

    let riot_realtime = serde_json::from_str(&data)?;
    let realtime_data =
        realtime(&riot_realtime).ok_or("Error calling function `realtime`. Invalid input")?;
    respond(realtime_data)
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, http::StatusCode, test};
    use std::num::NonZeroU8;
    use tutorlolv2::{ChampionId, model::*, riot::*};

    fn live_game() -> RiotRealtime<'static> {
        let scores = RiotScoreboard {
            kills: 0,
            deaths: 0,
            assists: 0,
            creep_score: 0,
        };

        let player = |champion_name, riot_id, team| RiotAllPlayers {
            champion_name,
            riot_id,
            team,
            position: "TOP",
            level: NonZeroU8::MIN,
            items: Box::new([]),
            scores,
        };

        RiotRealtime {
            active_player: RiotActivePlayer {
                abilities: RiotAbilities {
                    q: RiotAbility { ability_level: 1 },
                    w: RiotAbility { ability_level: 1 },
                    e: RiotAbility { ability_level: 1 },
                    r: RiotAbility { ability_level: 1 },
                },
                champion_stats: PlayerStats {
                    max_health: 1000.0,
                    current_health: 1000.0,
                    attack_damage: 100.0,
                    crit_damage: 200.0,
                    attack_speed: 1.0,
                    armor_penetration_percent: 1.0,
                    magic_penetration_percent: 1.0,
                    ..PlayerStats::default()
                },
                full_runes: RiotFullRunes {
                    general_runes: None,
                },
                level: NonZeroU8::MIN,
                riot_id: "self",
            },
            all_players: vec![
                player("Gnar", "self", "ORDER"),
                player("Aatrox", "enemy", "CHAOS"),
            ]
            .into_boxed_slice(),
            events: RiotRealtimeEvents {
                events: Box::new([]),
            },
            game_data: RiotRealtimeGameData {
                game_time: 1800.0,
                map_number: 11,
            },
        }
    }

    #[actix_web::test]
    async fn calculator_accepts_bincode_borrowed_input_and_returns_decodable_output() {
        let app = test::init_service(App::new().service(calculator_handler)).await;
        let enemies = [InputMinData {
            champion_id: ChampionId::Aatrox,
            ..Default::default()
        }];
        let input = InputGame {
            enemy_players: &enemies,
            ..Default::default()
        };
        let bytes = bincode::encode_to_vec(input, bincode::config::standard()).unwrap();
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/calculator")
                .set_payload(bytes)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            OCTET_STREAM
        );
        let body = test::read_body(response).await;
        let (output, used): (OutputGame, _) =
            bincode::decode_from_slice(&body, bincode::config::standard()).unwrap();
        assert_eq!(used, body.len());
        assert_eq!(output.enemies.len(), 1);
        assert_eq!(output.enemies[0].champion_id, ChampionId::Aatrox);
    }

    #[actix_web::test]
    async fn realtime_accepts_bincode_and_returns_borrowable_output() {
        let app = test::init_service(App::new().service(realtime_handler)).await;
        let bytes = bincode::encode_to_vec(live_game(), bincode::config::standard()).unwrap();
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/realtime")
                .set_payload(bytes)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = test::read_body(response).await;
        let (output, used): (Realtime<'_>, _) =
            bincode::borrow_decode_from_slice(&body, bincode::config::standard()).unwrap();
        assert_eq!(used, body.len());
        assert_eq!(output.enemies.len(), 1);
        assert_eq!(output.current_player.riot_id, "self");
        assert_eq!(output.enemies[0].siml_items[0].items.len(), 2);
    }

    #[actix_web::test]
    async fn malformed_and_trailing_bytes_return_bad_request_for_both_endpoints() {
        let app = test::init_service(
            App::new()
                .service(calculator_handler)
                .service(realtime_handler),
        )
        .await;
        let mut calc =
            bincode::encode_to_vec(InputGame::default(), bincode::config::standard()).unwrap();
        calc.push(0);
        let mut live = bincode::encode_to_vec(live_game(), bincode::config::standard()).unwrap();
        live.push(0);
        for (path, bytes) in [
            ("/calculator", vec![255]),
            ("/calculator", calc),
            ("/realtime", vec![255]),
            ("/realtime", live),
        ] {
            let response = test::call_service(
                &app,
                test::TestRequest::post()
                    .uri(path)
                    .set_payload(bytes)
                    .to_request(),
            )
            .await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
    }
}
