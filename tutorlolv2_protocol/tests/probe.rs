use serde_json::json;
use tutorlolv2_protocol::*;

const ME: &str = "Me#BR1";

fn entry(team: Team, riot_id: &str, champion: &str) -> RosterEntry {
    RosterEntry { team, riot_id: riot_id.into(), champion: champion.into(), skin_id: 0, keystone: None, is_bot: false }
}

fn roster() -> Vec<RosterEntry> {
    let mut roster = vec![
        entry(Team::Order, ME, "ahri"),
        entry(Team::Order, "A2#1", "lux"),
        entry(Team::Order, "A3#1", "garen"),
        entry(Team::Order, "A4#1", "jinx"),
        entry(Team::Order, "A5#1", "leona"),
        entry(Team::Chaos, "B1#1", "zed"),
        entry(Team::Chaos, "B2#1", "darius"),
        entry(Team::Chaos, "B3#1", "vayne"),
        entry(Team::Chaos, "B4#1", "thresh"),
        entry(Team::Chaos, "B5#1", "lee sin"),
    ];
    roster.sort();
    roster
}

fn events(list: &[(u32, &str, f32)]) -> Vec<EventKey> {
    list.iter()
        .map(|&(id, name, t)| EventKey { id, name_hash: fnv1a32(name.as_bytes()), time_bits: t.to_bits() })
        .collect()
}

fn probe(t: f32, list: &[(u32, &str, f32)]) -> Probe {
    Probe {
        active_riot_id: ME.into(),
        game_mode: GameMode::Classic,
        map_number: 11,
        roster: roster(),
        game_time: t,
        events: events(list),
        riot_game: None,
    }
}

const START: (u32, &str, f32) = (0, "GameStart", 0.0276);
const MINIONS: (u32, &str, f32) = (1, "MinionsSpawning", 30.05);
const KILL: (u32, &str, f32) = (2, "ChampionKill", 95.5);

fn decide(prev: &Probe, prev_seen_ms: i64, next: &Probe, now_ms: i64) -> MatchDecision {
    same_match(prev, prev_seen_ms, next, now_ms)
}

#[test]
fn continuous_game_is_same() {
    let prev = probe(100.0, &[START, MINIONS]);
    let next = probe(101.0, &[START, MINIONS, KILL]);
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Same);
}

#[test]
fn app_restart_after_three_minutes_is_same() {
    let prev = probe(100.0, &[START, MINIONS]);
    let next = probe(280.0, &[START, MINIONS, KILL]);
    assert_eq!(decide(&prev, 0, &next, 180_000), MatchDecision::Same);
}

#[test]
fn pause_with_frozen_game_time_is_same() {
    let prev = probe(100.0, &[START, MINIONS]);
    let next = probe(100.0, &[START, MINIONS]);
    assert_eq!(decide(&prev, 0, &next, 60_000), MatchDecision::Same);
}

#[test]
fn loading_then_first_moving_snapshot_is_same() {
    let prev = probe(0.032, &[]);
    let next = probe(0.99, &[(0, "GameStart", 0.0)]);
    assert_eq!(decide(&prev, 0, &next, 43_000), MatchDecision::Same);
}

#[test]
fn new_game_lower_time_other_champion_is_different() {
    let prev = probe(1500.0, &[START, MINIONS, KILL]);
    let mut next = probe(40.0, &[(0, "GameStart", 0.0312), (1, "MinionsSpawning", 30.01)]);
    next.roster.iter_mut().find(|e| e.riot_id == ME).unwrap().champion = "zoe".into();
    assert_eq!(decide(&prev, 0, &next, 300_000), MatchDecision::Different(DifferentReason::ChampionChanged));
}

#[test]
fn same_roster_diverged_events_is_different() {
    let prev = probe(10.0, &[START]);
    let next = probe(20.0, &[(0, "GameStart", 0.0312)]);
    assert_eq!(decide(&prev, 0, &next, 10_000), MatchDecision::Different(DifferentReason::EventsDiverged));
}

#[test]
fn missing_previous_event_is_different() {
    let prev = probe(100.0, &[START, MINIONS]);
    let next = probe(101.0, &[START]);
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Different(DifferentReason::EventsDiverged));
}

#[test]
fn time_went_backwards_is_different() {
    let prev = probe(600.0, &[START]);
    let next = probe(100.0, &[START]);
    assert_eq!(decide(&prev, 0, &next, 5_000), MatchDecision::Different(DifferentReason::TimeWentBackwards));
}

#[test]
fn small_backwards_jitter_is_same() {
    let prev = probe(600.0, &[START]);
    let next = probe(598.5, &[START]);
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Same);
}

#[test]
fn time_faster_than_wall_clock_is_different() {
    let prev = probe(100.0, &[START]);
    let next = probe(400.0, &[START]);
    assert_eq!(decide(&prev, 0, &next, 60_000), MatchDecision::Different(DifferentReason::TimeTooFast));
}

#[test]
fn account_switch_is_different() {
    let prev = probe(100.0, &[START]);
    let mut next = probe(101.0, &[START]);
    next.active_riot_id = "Other#BR1".into();
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Different(DifferentReason::AccountChanged));
}

#[test]
fn mode_change_is_different() {
    let prev = probe(100.0, &[START]);
    let mut next = probe(101.0, &[START]);
    next.game_mode = GameMode::Aram;
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Different(DifferentReason::ModeChanged));
}

#[test]
fn leaver_disappearing_is_same() {
    let prev = probe(100.0, &[START]);
    let mut next = probe(101.0, &[START]);
    next.roster.retain(|e| e.riot_id != "B5#1");
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Same);
}

#[test]
fn player_appearing_is_different() {
    let mut prev = probe(100.0, &[START]);
    prev.roster.retain(|e| e.riot_id != "B5#1");
    let next = probe(101.0, &[START]);
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Different(DifferentReason::RosterChanged));
}

#[test]
fn duplicate_bot_riot_ids_on_both_teams_are_same() {
    let mut prev = probe(100.0, &[START]);
    prev.roster.push(entry(Team::Order, "Poppy#BOT", "poppy"));
    prev.roster.push(entry(Team::Chaos, "Poppy#BOT", "poppy"));
    prev.roster.sort();
    let mut next = prev.clone();
    next.game_time = 101.0;
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Same);

    // a third Poppy is a player appearing
    next.roster.push(entry(Team::Chaos, "Poppy#BOT", "poppy"));
    next.roster.sort();
    assert_eq!(decide(&prev, 0, &next, 1_000), MatchDecision::Different(DifferentReason::RosterChanged));
}

fn inspected(champion_name: &str, raw_champion: &str, raw_skin: &str, t: f32) -> Probe {
    let body = json!({
        "activePlayer": { "riotId": ME, "level": 14, "abilities": {} },
        "allPlayers": [{
            "riotId": ME, "championName": champion_name, "rawChampionName": raw_champion,
            "rawSkinName": raw_skin, "skinID": 1, "team": "ORDER", "isBot": false, "scores": {}
        }],
        "events": { "Events": [{ "EventID": 0, "EventName": "GameStart", "EventTime": 0.0276 }] },
        "gameData": { "gameMode": "CLASSIC", "gameTime": t, "mapNumber": 11 }
    });
    inspect(&serde_json::to_vec(&body).unwrap()).unwrap().probe
}

#[test]
fn neeko_disguise_and_mega_gnar_keep_same() {
    let neeko = inspected("Neeko", "game_character_displayname_Neeko", "game_character_skin_displayname_Neeko_1", 100.0);
    let disguised = inspected("Blue Sentinel", "game_character_displayname_SRUBlue", "game_character_skin_displayname_Neeko_1", 101.0);
    assert_eq!(decide(&neeko, 0, &disguised, 1_000), MatchDecision::Same);

    let gnar = inspected("Gnar", "game_character_displayname_Gnar", "game_character_skin_displayname_Gnar_1", 100.0);
    let mega = inspected("Mega Gnar", "game_character_displayname_GnarBig", "game_character_skin_displayname_Gnar_1", 101.0);
    assert_eq!(decide(&gnar, 0, &mega, 1_000), MatchDecision::Same);
}

#[test]
fn riot_game_is_decisive() {
    let game = |id| Some(RiotGameRef { region: Region::Na, game_id: id });
    let mut prev = probe(10.0, &[START]);
    let mut next = probe(20.0, &[(0, "GameStart", 0.0312)]);
    prev.riot_game = game(5669550383);
    next.riot_game = game(5669550383);
    assert_eq!(decide(&prev, 0, &next, 10_000), MatchDecision::Same);

    next.riot_game = game(5669550384);
    assert_eq!(decide(&prev, 0, &next, 10_000), MatchDecision::Different(DifferentReason::RiotGameDiffers));
}

#[test]
fn consistent_accepts_older_chunk_after_newer_probe() {
    let newer = probe(600.0, &[START, MINIONS, KILL]);
    let older = probe(300.0, &[START, MINIONS]);
    assert_eq!(consistent(&newer, &older), Ok(()));
    assert_eq!(consistent(&older, &newer), Ok(()));

    let mut with_leaver = newer.clone();
    with_leaver.roster.retain(|e| e.riot_id != "B5#1");
    assert_eq!(consistent(&with_leaver, &older), Ok(()));
    assert_eq!(consistent(&older, &with_leaver), Ok(()));
}

#[test]
fn consistent_rejects_other_match() {
    let a = probe(600.0, &[START, MINIONS]);
    let b = probe(300.0, &[(0, "GameStart", 0.0312)]);
    assert_eq!(consistent(&a, &b), Err(DifferentReason::EventsDiverged));

    let mut c = probe(300.0, &[START]);
    c.roster.iter_mut().find(|e| e.riot_id == "B1#1").unwrap().champion = "yasuo".into();
    assert_eq!(consistent(&a, &c), Err(DifferentReason::ChampionChanged));
}

#[test]
fn fingerprint_ignores_time_and_events_and_order() {
    let a = probe(100.0, &[START]);
    let mut b = probe(900.0, &[START, MINIONS, KILL]);
    b.roster.reverse();
    assert_eq!(fingerprint(&a), fingerprint(&b));

    let mut c = probe(100.0, &[START]);
    c.roster[0].skin_id = 7;
    assert_ne!(fingerprint(&a), fingerprint(&c));
}
