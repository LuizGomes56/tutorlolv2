use tutorlolv2_perf::{Settings, StatsMode, Workload, generate, run, summarize};

#[test]
fn seeded_workload_roundtrips_with_exact_enemy_count() {
    for enemies in [0, 1, 5, 12] {
        let workload = generate(42, 4, enemies, StatsMode::Mixed).unwrap();
        assert_eq!(workload.games.len(), 4);
        assert!(
            workload
                .games
                .iter()
                .all(|game| game.enemy_players.len() == enemies)
        );
        let json = serde_json::to_vec(&workload).unwrap();
        let decoded = serde_json::from_slice::<Workload>(&json).unwrap();
        let prepared = decoded.prepare().unwrap();

        assert_eq!(prepared[0].input().enemy_players.len(), enemies);
        assert_eq!(
            json,
            serde_json::to_vec(&generate(42, 4, enemies, StatsMode::Mixed).unwrap()).unwrap()
        );
        assert_ne!(
            json,
            serde_json::to_vec(&generate(43, 4, enemies, StatsMode::Mixed).unwrap()).unwrap()
        );
    }
}

#[test]
fn explicit_and_inferred_fixtures_exercise_both_paths() {
    let inferred = generate(10, 3, 5, StatsMode::Inferred).unwrap();
    let explicit = generate(10, 3, 5, StatsMode::Explicit).unwrap();
    assert!(
        inferred
            .games
            .iter()
            .all(|game| game.active_player.data.stats.is_none()
                && game.enemy_players.iter().all(|enemy| enemy.stats.is_none()))
    );
    assert!(
        explicit
            .games
            .iter()
            .all(|game| game.active_player.data.stats.is_some()
                && game.enemy_players.iter().all(|enemy| enemy.stats.is_some()))
    );
    for fixture in [&inferred, &explicit] {
        for game in fixture.prepare().unwrap() {
            let result = tutorlolv2::calculator::calculator(game.input());
            assert_eq!(result.enemies.len(), 5);
        }
    }
}

#[test]
fn summary_reports_percentiles_of_batch_means() {
    let summary = summarize(&[50.0, 10.0, 30.0, 20.0, 40.0]).unwrap();
    assert_eq!(summary.mean_ns_per_call, 30.0);
    assert_eq!(summary.median_ns_per_call, 30.0);
    assert_eq!(summary.p95_batch_mean_ns_per_call, 50.0);
    assert_eq!(summary.min_ns_per_call, 10.0);
    assert_eq!(summary.max_ns_per_call, 50.0);
}

#[test]
fn real_measurement_records_requested_calls_and_raw_samples() {
    let workload = generate(2, 2, 3, StatsMode::Mixed).unwrap();
    let prepared = workload.prepare().unwrap();
    let measurement = run(
        &prepared,
        Settings {
            iterations: 5,
            samples: 3,
            warmup: 2,
        },
    )
    .unwrap();
    assert_eq!(measurement.measured_calls, 15);
    assert_eq!(measurement.samples_ns_per_call.len(), 3);
    assert!(
        measurement
            .samples_ns_per_call
            .iter()
            .all(|&sample| sample > 0.0)
    );
    assert!(generate(0, 0, 5, StatsMode::Mixed).is_err());
    assert!(
        run(
            &prepared,
            Settings {
                iterations: 0,
                samples: 3,
                warmup: 0
            }
        )
        .is_err()
    );
    assert!(summarize(&[]).is_err());
}

#[test]
fn cli_writes_replayable_fixture_and_results_without_overwriting_baselines() {
    use std::{
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };
    let dir = std::env::temp_dir().join(format!(
        "tutorlolv2-perf-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let fixture = dir.join("input.json");
    let result = dir.join("result.json");
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_tutorlolv2_perf"))
            .args([
                "--enemies",
                "3",
                "--games",
                "2",
                "--seed",
                "42",
                "--iterations",
                "3",
                "--samples",
                "2",
                "--warmup",
                "1",
            ])
            .arg("--input")
            .arg(&fixture)
            .arg("--output")
            .arg(&result)
            .output()
            .unwrap()
    };
    let first = invoke();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&result).unwrap()).unwrap();
    assert_eq!(report["measurement"]["measured_calls"], 6);
    assert_eq!(report["workload"]["enemies_per_game"], 3);
    let original = std::fs::read(&result).unwrap();
    assert!(!invoke().status.success());
    assert_eq!(std::fs::read(&result).unwrap(), original);
    std::fs::remove_dir_all(dir).unwrap();
}
