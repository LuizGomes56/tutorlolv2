use tutorlolv2_protocol::*;

#[test]
fn open_enum_roundtrips_known_and_unknown() {
    assert_eq!(GameMode::from("KIWI"), GameMode::Kiwi);
    assert_eq!(GameMode::from("NEWMODE").as_str(), "NEWMODE");

    for mode in [GameMode::Kiwi, GameMode::from("NEWMODE")] {
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(serde_json::from_str::<GameMode>(&json).unwrap(), mode);

        let config = tutorlolv2_types::BINCODE_CONFIG;
        let bytes = bincode::encode_to_vec(&mode, config).unwrap();
        let (decoded, used): (GameMode, _) = bincode::decode_from_slice(&bytes, config).unwrap();
        assert_eq!((decoded, used), (mode, bytes.len()));
    }
    assert_eq!(serde_json::to_string(&GameMode::Kiwi).unwrap(), "\"KIWI\"");
}

#[test]
fn open_enum_bincode_is_the_string() {
    let config = tutorlolv2_types::BINCODE_CONFIG;
    assert_eq!(
        bincode::encode_to_vec(GameMode::Kiwi, config).unwrap(),
        bincode::encode_to_vec(String::from("KIWI"), config).unwrap()
    );
}

#[test]
fn open_enum_compares_by_string_form() {
    assert_eq!(Team::Other("ORDER".into()), Team::Order);
    assert!(Team::Chaos < Team::Order);
}

#[test]
fn region_platform_mapping() {
    assert_eq!(Region::Na.platform(), Some(Platform::Na1));
    assert_eq!(Platform::Na1.regional(), Some(Regional::Americas));
    assert_eq!(Platform::Na1.host(), Some("na1"));
    assert_eq!(Region::from("XX").platform(), None);
    assert_eq!(Platform::from("XX1").regional(), None);
}

#[test]
fn game_type_matched() {
    assert!(GameType::from("MATCHED").is_matched());
    assert!(GameType::from("MATCHED_GAME").is_matched());
    assert!(!GameType::from("CUSTOM").is_matched());
}

#[test]
fn game_result_reads_riot_casing() {
    assert_eq!(serde_json::from_str::<GameResult>("\"Win\"").unwrap(), GameResult::Win);
    assert_eq!(serde_json::from_str::<GameResult>("\"Lose\"").unwrap(), GameResult::Lose);
}
