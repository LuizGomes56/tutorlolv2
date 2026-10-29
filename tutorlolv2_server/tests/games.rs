use actix_web::{App, http::StatusCode, test};
use std::num::NonZeroU8;
use tutorlolv2::{ChampionId, model::*, riot::*};
use tutorlolv2_server::server::{codec::OCTET_STREAM, games::*};

fn live_game() -> RiotRealtime<'static> {
    let scores = RiotScoreboard {
        kills: 0,
        deaths: 0,
        assists: 0,
        creep_score: 0,
    };

    let player = |champion_name, riot_id, team| RiotAllPlayers {
        champion_name,
        raw_champion_name: "",
        raw_skin_name: "",
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
