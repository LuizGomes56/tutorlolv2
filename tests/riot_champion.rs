//! Real champion of a Live Client player despite Neeko's disguise and Gnar's
//! form (spec §8.6), as `realtime()` sees it.

use std::{num::NonZeroU8, path::Path};
use tutorlolv2::{
    ChampionId,
    model::*,
    realtime::realtime,
    riot::*,
};

fn player(champion_name: &'static str, raw_champion_name: &'static str, raw_skin_name: &'static str) -> RiotAllPlayers<'static> {
    RiotAllPlayers {
        champion_name,
        raw_champion_name,
        raw_skin_name,
        items: Box::new([]),
        level: NonZeroU8::new(18).unwrap(),
        position: "MIDDLE",
        riot_id: "self",
        team: "ORDER",
        scores: RiotScoreboard { kills: 0, deaths: 0, assists: 0, creep_score: 0 },
    }
}

fn enemy(champion_name: &'static str, raw_champion_name: &'static str, raw_skin_name: &'static str) -> RiotAllPlayers<'static> {
    RiotAllPlayers { riot_id: "enemy", team: "CHAOS", ..player(champion_name, raw_champion_name, raw_skin_name) }
}

fn game(me: RiotAllPlayers<'static>, foe: RiotAllPlayers<'static>, level: u8, points: [u8; 4]) -> RiotRealtime<'static> {
    let ability = |ability_level| RiotAbility { ability_level };
    RiotRealtime {
        active_player: RiotActivePlayer {
            abilities: RiotAbilities { q: ability(points[0]), w: ability(points[1]), e: ability(points[2]), r: ability(points[3]) },
            champion_stats: PlayerStats {
                max_health: 2608.0,
                current_health: 2608.0,
                attack_damage: 90.0,
                crit_damage: 175.0,
                attack_speed: 1.0,
                ..PlayerStats::default()
            },
            full_runes: RiotFullRunes { general_runes: None },
            level: NonZeroU8::new(level).unwrap(),
            riot_id: "self",
        },
        all_players: vec![me, foe].into_boxed_slice(),
        events: RiotRealtimeEvents { events: Box::new([]) },
        game_data: RiotRealtimeGameData { game_time: 900.0, map_number: 11 },
    }
}

#[test]
fn champion_id_uses_raw_skin_name() {
    assert_eq!(player("Renata Glasc", "game_character_displayname_Renata", "game_character_skin_displayname_Neeko_22").champion_id(), ChampionId::Neeko);
    assert_eq!(player("Blue Sentinel", "game_character_displayname_SRUBlue", "game_character_skin_displayname_Neeko_1").champion_id(), ChampionId::Neeko);
    assert_eq!(player("Maokai", "game_character_displayname_Maokai", "game_character_displayname_Neeko").champion_id(), ChampionId::Neeko);
    assert_eq!(player("Wukong", "game_character_displayname_MonkeyKing", "game_character_displayname_MonkeyKing").champion_id(), ChampionId::MonkeyKing);
    assert_eq!(player("Fiddlesticks", "game_character_displayname_Fiddlesticks", "game_character_displayname_FiddleSticks").champion_id(), ChampionId::Fiddlesticks);
    assert_eq!(player("Ahri", "", "").champion_id(), ChampionId::Ahri);
}

#[test]
fn mega_gnar_is_language_independent() {
    let p = player("메가 나르", "game_character_displayname_GnarBig", "game_character_displayname_Gnar");
    assert_eq!(p.champion_id(), ChampionId::Gnar);
    assert!(p.is_mega_gnar(ChampionId::Gnar));
    assert!(!player("Gnar", "game_character_displayname_Gnar", "game_character_displayname_Gnar").is_mega_gnar(ChampionId::Gnar));
}

#[test]
fn non_transforming_champions_take_the_name_path() {
    // Raw fields deliberately inconsistent: the name wins when the skin isn't Neeko's.
    assert_eq!(player("Ahri", "game_character_displayname_Lux", "game_character_displayname_Lux").champion_id(), ChampionId::Ahri);
}

#[test]
fn disguised_neeko_detection() {
    let disguised = player("Blue Sentinel", "game_character_displayname_SRUBlue", "game_character_skin_displayname_Neeko_1");
    assert!(disguised.is_disguised_neeko(ChampionId::Neeko));
    let plain = player("Neeko", "game_character_displayname_Neeko", "game_character_skin_displayname_Neeko_1");
    assert!(!plain.is_disguised_neeko(ChampionId::Neeko));
}

#[test]
fn realtime_computes_disguised_neeko_as_neeko_at_true_level() {
    let me = player("Blue Sentinel", "game_character_displayname_SRUBlue", "game_character_skin_displayname_Neeko_1");
    let foe = enemy("Ahri", "game_character_displayname_Ahri", "game_character_displayname_Ahri");
    let input = game(me, foe, 1, [5, 5, 5, 3]);
    let out = realtime(&input).unwrap();
    assert_eq!(out.current_player.champion_id, ChampionId::Neeko);
    assert_eq!(out.current_player.level.get(), 18);
}

#[test]
fn realtime_scoreboard_resolves_disguised_neeko() {
    let me = player("Ahri", "game_character_displayname_Ahri", "game_character_displayname_Ahri");
    let foe = enemy("Renata Glasc", "game_character_displayname_Renata", "game_character_skin_displayname_Neeko_22");
    let input = game(me, foe, 18, [5, 5, 5, 3]);
    let out = realtime(&input).unwrap();
    assert_eq!(out.enemies[0].champion_id, ChampionId::Neeko);
    assert!(out.scoreboard.iter().any(|s| s.riot_id == "enemy" && s.champion_id == ChampionId::Neeko));
}

#[test]
fn enemy_mega_gnar_uses_mega_base_stats() {
    let me = || player("Ahri", "game_character_displayname_Ahri", "game_character_displayname_Ahri");
    let mini_input = game(me(), enemy("Gnar", "game_character_displayname_Gnar", "game_character_displayname_Gnar"), 18, [5, 5, 5, 3]);
    let mega_input = game(me(), enemy("Mega Gnar", "game_character_displayname_GnarBig", "game_character_displayname_Gnar"), 18, [5, 5, 5, 3]);
    let mini = realtime(&mini_input).unwrap();
    let mega = realtime(&mega_input).unwrap();
    assert_eq!(mini.enemies[0].champion_id, ChampionId::Gnar);
    assert_eq!(mega.enemies[0].champion_id, ChampionId::Gnar);
    let level = NonZeroU8::new(18).unwrap();
    assert_eq!(mini.enemies[0].base_stats, SimpleStats::base_stats(ChampionId::Gnar, level, false));
    assert_eq!(mega.enemies[0].base_stats, SimpleStats::base_stats(ChampionId::Gnar, level, true));
    assert_ne!(mini.enemies[0].base_stats, mega.enemies[0].base_stats);
}

fn captures(dir: &str) -> Vec<std::path::PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("captures").join(dir);
    let mut files: Vec<_> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("{}: {e}", root.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    files
}

#[test]
#[ignore = "needs local captures"]
fn realtime_on_transformation_captures() {
    for file in captures("neeko") {
        let json = std::fs::read(&file).unwrap();
        let input: RiotRealtime = serde_json::from_slice(&json).unwrap();
        let out = realtime(&input).unwrap();
        assert_eq!(out.current_player.champion_id, ChampionId::Neeko, "{}", file.display());
    }
    for file in captures("gnar") {
        let json = std::fs::read(&file).unwrap();
        let input: RiotRealtime = serde_json::from_slice(&json).unwrap();
        let out = realtime(&input).unwrap();
        let is_mega = file.file_name().unwrap().to_string_lossy().starts_with("mega");
        let expected = BasicStats::base_stats(ChampionId::Gnar, out.current_player.level, is_mega);
        assert_eq!(out.current_player.champion_id, ChampionId::Gnar, "{}", file.display());
        assert_eq!(out.current_player.base_stats, expected, "{}", file.display());
    }
}
