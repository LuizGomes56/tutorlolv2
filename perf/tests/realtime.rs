use tutorlolv2::{ItemId, model::RiotFormulas, realtime::realtime};
use tutorlolv2_perf::{
    Settings,
    realtime::{RealtimeWorkload, generate_realtime, run_realtime},
};

#[test]
fn generated_raw_names_preserve_champion_identity_and_mega_form() {
    let bytes = generate_realtime(42, 128, 10, 4).unwrap();
    let corpus: RealtimeWorkload<'_> = serde_json::from_slice(&bytes).unwrap();
    let mut saw_mega = false;
    for game in &corpus.games {
        for player in &game.all_players {
            assert!(
                player
                    .raw_skin_name
                    .starts_with("game_character_skin_displayname_")
            );
            assert!(
                player
                    .raw_champion_name
                    .starts_with("game_character_displayname_")
            );
            let champion = player.champion_id();
            if player.champion_name == "Mega Gnar" {
                saw_mega = true;
                assert_eq!(champion, tutorlolv2::ChampionId::Gnar);
                assert!(player.is_mega_gnar(champion));
                assert_eq!(
                    player.raw_skin_name,
                    "game_character_skin_displayname_Gnar_0"
                );
            }
            if champion == tutorlolv2::ChampionId::Neeko {
                assert!(!player.is_disguised_neeko(champion));
            }
        }
    }
    assert!(saw_mega, "seeded corpus must cover Mega Gnar");
}

#[test]
fn seeded_live_workloads_have_exact_teams_and_roundtrip_ids() {
    for enemies in [0, 1, 5, 12] {
        let bytes = generate_realtime(42, 2, enemies, 4).unwrap();
        assert_eq!(bytes, generate_realtime(42, 2, enemies, 4).unwrap());
        assert_ne!(bytes, generate_realtime(43, 2, enemies, 4).unwrap());
        let corpus: RealtimeWorkload<'_> = serde_json::from_slice(&bytes).unwrap();
        corpus.validate().unwrap();
        assert_eq!(corpus.games.len(), 2);
        for game in &corpus.games {
            let result = realtime(game).unwrap();
            assert_eq!(result.enemies.len(), enemies);
            assert_eq!(result.scoreboard.len(), enemies + 5);
            assert_eq!(result.current_player.riot_id, game.active_player.riot_id);
            assert!(
                result
                    .enemies
                    .iter()
                    .all(|enemy| enemy.team != result.current_player.team)
            );
            for player in &game.all_players {
                assert!(
                    player
                        .champion_name
                        .parse::<tutorlolv2::ChampionId>()
                        .is_ok()
                );
                assert!(player.position.parse::<tutorlolv2::Position>().is_ok());
                assert!(
                    player
                        .items
                        .iter()
                        .all(|item| ItemId::from_riot_id(item.item_id).is_some())
                );
            }
        }
    }
}

#[test]
fn live_stats_use_remaining_fraction_penetration() {
    let bytes = generate_realtime(8, 1, 2, 4).unwrap();
    let mut corpus: RealtimeWorkload<'_> = serde_json::from_slice(&bytes).unwrap();
    let game = &mut corpus.games[0];
    game.active_player.champion_stats.armor_penetration_percent = 0.75;
    game.active_player.champion_stats.magic_penetration_percent = 0.5;
    let result = realtime(game).unwrap();
    assert_eq!(
        result
            .current_player
            .current_stats
            .armor_penetration_percent,
        25.0
    );
    assert_eq!(
        result
            .current_player
            .current_stats
            .magic_penetration_percent,
        50.0
    );
    assert!(game.game_data.game_time > 0.0);
    for enemy in &result.enemies {
        assert_eq!(enemy.siml_items.len(), ItemId::L_SIML);
        assert_eq!(
            enemy.damages.ctx.missing_health,
            RiotFormulas::missing_health(
                game.active_player.champion_stats.current_health,
                game.active_player.champion_stats.max_health
            )
        );
    }
}

#[test]
fn live_measurements_record_only_requested_calls_and_reject_bad_corpora() {
    let bytes = generate_realtime(4, 2, 2, 1).unwrap();
    let mut corpus: RealtimeWorkload<'_> = serde_json::from_slice(&bytes).unwrap();
    let result = run_realtime(
        &corpus.games,
        Settings {
            iterations: 3,
            samples: 2,
            warmup: 1,
        },
    )
    .unwrap();
    assert_eq!(result.measured_calls, 6);
    assert_eq!(result.samples_ns_per_call.len(), 2);
    corpus.enemies_per_game += 1;
    assert!(corpus.validate().is_err());
    assert!(generate_realtime(1, 0, 5, 4).is_err());
}

#[test]
fn missing_rune_page_and_zero_allies_use_supported_live_paths() {
    let bytes = generate_realtime(9, 4, 1, 0).unwrap();
    let corpus: RealtimeWorkload<'_> = serde_json::from_slice(&bytes).unwrap();
    corpus.validate().unwrap();
    let game = &corpus.games[3];
    assert!(game.active_player.full_runes.general_runes.is_none());
    let output = realtime(game).unwrap();
    assert_eq!(output.scoreboard.len(), 2);
    assert_eq!(output.enemies.len(), 1);
    assert!(output.runes_meta.is_empty());
    assert!(output.enemies[0].damages.runes.is_empty());
}

#[test]
fn missing_active_player_fails_before_benchmark_timing() {
    let bytes = generate_realtime(9, 1, 1, 4).unwrap();
    let mut corpus: RealtimeWorkload<'_> = serde_json::from_slice(&bytes).unwrap();
    corpus.games[0].active_player.riot_id = "missing#TEST";
    assert!(corpus.validate().is_err());
    assert!(
        run_realtime(
            &corpus.games,
            Settings {
                iterations: 1,
                samples: 1,
                warmup: 0
            }
        )
        .is_err()
    );
}

#[test]
fn realtime_cli_writes_a_separate_report_and_replays_input() {
    use std::{
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };
    let dir = std::env::temp_dir().join(format!(
        "tutorlolv2-realtime-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("live.json");
    let report = dir.join("result.json");
    let run = Command::new(env!("CARGO_BIN_EXE_realtime"))
        .args([
            "--enemies",
            "2",
            "--games",
            "2",
            "--iterations",
            "2",
            "--samples",
            "2",
            "--warmup",
            "1",
        ])
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&report)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(report).unwrap()).unwrap();
    assert_eq!(result["function"], "realtime");
    assert_eq!(result["measurement"]["measured_calls"], 4);
    assert_eq!(result["workload"]["enemies_per_game"], 2);
    assert_eq!(result["workload"]["allies_per_game"], 4);
    assert_eq!(
        result["workload"]["simulated_items_per_enemy"],
        ItemId::L_SIML
    );
    std::fs::remove_dir_all(dir).unwrap();
}
