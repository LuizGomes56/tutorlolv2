use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tutorlolv2_protocol::*;

fn player(riot_id: &str, champion_name: &str, raw_champion: &str, raw_skin: &str) -> Value {
    json!({
        "riotId": riot_id,
        "championName": champion_name,
        "rawChampionName": raw_champion,
        "rawSkinName": raw_skin,
        "skinID": 0,
        "team": "ORDER",
        "isBot": false,
        "position": "MIDDLE",
        "level": 1,
        "items": [],
        "runes": { "keystone": { "id": 8005 } },
        "scores": { "kills": 3, "deaths": 1, "assists": 7, "creepScore": 42 }
    })
}

/// A valid `allgamedata` body: `players` are `(riotId, championName, rawChampionName, rawSkinName)`,
/// the first one is the active player, `events` is the JSON array of `events.Events`.
fn snap(active: &str, players: &[(&str, &str, &str, &str)], events: &str) -> Value {
    json!({
        "activePlayer": {
            "riotId": active,
            "level": 1,
            "abilities": {
                "Q": { "abilityLevel": 0 }, "W": { "abilityLevel": 0 },
                "E": { "abilityLevel": 0 }, "R": { "abilityLevel": 0 },
                "Passive": {}
            }
        },
        "allPlayers": players.iter().map(|&(id, name, raw_champion, raw_skin)| player(id, name, raw_champion, raw_skin)).collect::<Vec<_>>(),
        "events": { "Events": serde_json::from_str::<Value>(events).unwrap() },
        "gameData": { "gameMode": "CLASSIC", "gameTime": 120.5, "mapName": "Map11", "mapNumber": 11, "mapTerrain": "Default" }
    })
}

fn one_player() -> Value {
    snap(
        "Me#BR1",
        &[("Me#BR1", "Ahri", "game_character_displayname_Ahri", "game_character_displayname_Ahri")],
        r#"[{"EventID":0,"EventName":"GameStart","EventTime":0.0276}]"#,
    )
}

fn inspect_value(v: &Value) -> Result<Snapshot, SnapshotError> {
    inspect(&serde_json::to_vec(v).unwrap())
}

#[test]
fn champion_key_cases() {
    assert_eq!(champion_key("game_character_skin_displayname_Neeko_22", "Renata Glasc"), "neeko");
    assert_eq!(champion_key("game_character_displayname_Neeko", "Maokai"), "neeko");
    assert_eq!(champion_key("game_character_displayname_Gnar", "Mega Gnar"), "gnar");
    assert_eq!(champion_key("game_character_displayname_FiddleSticks", "Fiddlesticks"), "fiddlesticks");
    assert_eq!(champion_key("game_character_displayname_MonkeyKing", "Wukong"), "monkeyking");
    assert_eq!(champion_key("", "Ahri"), "ahri");
}

#[test]
fn fnv1a32_known_values() {
    assert_eq!(fnv1a32(b""), 0x811C_9DC5);
    assert_eq!(fnv1a32(b"a"), 0xE40C_292C);
}

#[test]
fn inspect_rejects_replay_error_object() {
    let mut v = one_player();
    v["activePlayer"] = json!({ "error": "Spectator mode doesn't currently support this feature" });
    assert_eq!(inspect_value(&v).unwrap_err(), SnapshotError::NoActivePlayer);
}

#[test]
fn inspect_rejects_404_body() {
    let body = br#"{"errorCode":"RESOURCE_NOT_FOUND","httpStatus":404,"message":"Invalid URI format"}"#;
    assert_eq!(inspect(body).unwrap_err(), SnapshotError::NoActivePlayer);
}

#[test]
fn inspect_rejects_non_json() {
    assert_eq!(inspect(b"not json").unwrap_err(), SnapshotError::Json);
}

#[test]
fn inspect_rejects_active_player_missing_from_roster() {
    let mut v = one_player();
    v["activePlayer"]["riotId"] = json!("Someone#Else");
    assert_eq!(inspect_value(&v).unwrap_err(), SnapshotError::ActivePlayerNotInRoster);
}

#[test]
fn inspect_rejects_negative_game_time() {
    let mut v = one_player();
    v["gameData"]["gameTime"] = json!(-1.0);
    assert_eq!(inspect_value(&v).unwrap_err(), SnapshotError::BadGameTime);
}

#[test]
fn inspect_accepts_loading_screen() {
    let mut v = one_player();
    v["gameData"]["gameTime"] = json!(0.032);
    v["events"]["Events"] = json!([]);
    let s = inspect_value(&v).unwrap();
    assert!(s.probe.events.is_empty());
    assert_eq!(s.game_end, None);
    assert_eq!(s.game_time, 0.032);
}

#[test]
fn inspect_builds_probe() {
    let s = inspect_value(&one_player()).unwrap();
    assert_eq!(s.probe.active_riot_id, "Me#BR1");
    assert_eq!(s.probe.game_mode, GameMode::Classic);
    assert_eq!(s.probe.map_number, 11);
    assert_eq!(s.probe.riot_game, None);
    assert_eq!(
        s.probe.roster,
        vec![RosterEntry {
            team: Team::Order,
            riot_id: "Me#BR1".into(),
            champion: "ahri".into(),
            skin_id: 0,
            keystone: Some(8005),
            is_bot: false,
        }]
    );
    assert_eq!(
        s.probe.events,
        vec![EventKey { id: 0, name_hash: fnv1a32(b"GameStart"), time_bits: 0.0276f32.to_bits() }]
    );
    assert_eq!(
        s.summary,
        Summary { champion: "ahri".into(), kills: 3, deaths: 1, assists: 7, creep_score: 42, level: 1 }
    );
}

#[test]
fn inspect_sorts_roster() {
    let v = snap(
        "B#1",
        &[("B#1", "Ahri", "", "game_character_displayname_Ahri"), ("A#1", "Lux", "", "game_character_displayname_Lux")],
        "[]",
    );
    let roster = inspect_value(&v).unwrap().probe.roster;
    assert_eq!(roster.iter().map(|e| e.riot_id.as_str()).collect::<Vec<_>>(), ["A#1", "B#1"]);
}

#[test]
fn inspect_extracts_game_end() {
    let mut v = one_player();
    v["events"]["Events"] = json!([
        {"EventID":0,"EventName":"GameStart","EventTime":0.0276},
        {"EventID":135,"EventName":"GameEnd","EventTime":1020.364501953125,"Result":"Lose"}
    ]);
    let s = inspect_value(&v).unwrap();
    assert_eq!(
        s.game_end,
        Some(GameEnd { event_time: 1020.364501953125f64 as f32, result: Some(GameResult::Lose) })
    );
}

#[test]
fn inspect_accepts_null_runes() {
    let mut v = one_player();
    v["allPlayers"][0]["runes"] = Value::Null;
    v["allPlayers"][0]["position"] = json!("NONE");
    let s = inspect_value(&v).unwrap();
    assert!(s.probe.roster.iter().all(|e| e.keystone.is_none()));
}

#[test]
fn inspect_accepts_escaped_unicode_riot_id() {
    let v = one_player();
    let text = serde_json::to_string(&v).unwrap().replace("Me#BR1", "\\u6df1\\u6d77\\u00e0#002");
    assert!(text.contains(r"\u6df1\u6d77\u00e0"), "the JSON must carry real escapes");
    let s = inspect(text.as_bytes()).unwrap();
    assert_eq!(s.probe.active_riot_id, "深海à#002");
}

#[test]
fn summary_corrects_disguised_neeko_level() {
    let mut v = snap(
        "Me#BR1",
        &[("Me#BR1", "Blue Sentinel", "game_character_displayname_SRUBlue", "game_character_skin_displayname_Neeko_1")],
        "[]",
    );
    for (k, level) in [("Q", 5), ("W", 5), ("E", 5), ("R", 3)] {
        v["activePlayer"]["abilities"][k]["abilityLevel"] = json!(level);
    }
    let s = inspect_value(&v).unwrap();
    assert_eq!(s.summary.champion, "neeko");
    assert_eq!(s.summary.level, 18);
}

#[test]
fn summary_keeps_jayce_level() {
    let mut v = snap(
        "Me#BR1",
        &[("Me#BR1", "Jayce", "game_character_displayname_Jayce", "game_character_displayname_Jayce")],
        "[]",
    );
    v["activePlayer"]["abilities"]["Q"]["abilityLevel"] = json!(1);
    v["activePlayer"]["abilities"]["R"]["abilityLevel"] = json!(1);
    assert_eq!(inspect_value(&v).unwrap().summary.level, 1);
}

#[test]
fn is_practice_from_game_mode() {
    let mut v = one_player();
    v["gameData"]["gameMode"] = json!("PRACTICETOOL");
    assert!(inspect_value(&v).unwrap().is_practice);
    v["gameData"]["gameMode"] = json!("KIWI");
    let s = inspect_value(&v).unwrap();
    assert!(!s.is_practice);
    assert_eq!(s.probe.game_mode, GameMode::Kiwi);
}

fn captures(dir: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../captures").join(dir);
    let mut files: Vec<_> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("{}: {e}", root.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no captures in {}", root.display());
    files
}

#[test]
#[ignore = "needs local captures"]
fn champion_keys_of_all_transformation_captures() {
    for (dir, key) in [("neeko", "neeko"), ("gnar", "gnar"), ("kayn", "kayn"), ("wukong", "monkeyking")] {
        for file in captures(dir) {
            let s = inspect(&std::fs::read(&file).unwrap()).unwrap();
            assert_eq!(s.summary.champion, key, "{}", file.display());
        }
    }
    let kayn = inspect(&std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../captures/kayn/regular.json")).unwrap()).unwrap();
    assert!(kayn.probe.roster.iter().any(|e| e.champion == "fiddlesticks"));
}

#[test]
#[ignore = "needs local captures"]
fn replay_capture_is_rejected_and_aram_capture_accepted() {
    for file in captures("replay-20261029-103710/live") {
        assert_eq!(inspect(&std::fs::read(&file).unwrap()).unwrap_err(), SnapshotError::NoActivePlayer, "{}", file.display());
    }
    for file in captures("aram-20261029-101315/live") {
        assert!(inspect(&std::fs::read(&file).unwrap()).is_ok(), "{}", file.display());
    }
}

#[test]
fn inspect_tolerates_unknown_game_end_result() {
    let mut v = one_player();
    v["events"]["Events"] = json!([
        {"EventID":0,"EventName":"GameStart","EventTime":0.0276},
        {"EventID":9,"EventName":"GameEnd","EventTime":900.5,"Result":"Draw"}
    ]);
    let s = inspect_value(&v).expect("an unknown result must not reject the snapshot");
    assert_eq!(s.game_end, Some(GameEnd { event_time: 900.5, result: None }));
}
