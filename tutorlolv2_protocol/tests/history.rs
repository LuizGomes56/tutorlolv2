use tutorlolv2_protocol::*;

#[test]
fn provisional_history_does_not_fabricate_a_server_summary() {
    let entry = HistoryEntry {
        local_id: Some([1; 16]),
        recording_id: None,
        summary: None,
        share: ShareState {
            mode: ShareMode::Private,
            code: None,
            url: None,
        },
        pending_chunks: 2,
        pending_control: true,
        lost_chunks: 0,
        last_error: None,
        hidden: false,
    };
    let bytes = bincode::encode_to_vec(&entry, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    let (decoded, used): (HistoryEntry, _) =
        bincode::decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(decoded, entry);
    assert_eq!(used, bytes.len());
    assert!(decoded.summary.is_none());
}

#[test]
fn history_modes_convert_to_existing_wire_tags() {
    let mode: ShareMode = HistoryShareMode::Public.into();
    assert_eq!(mode, ShareMode::Public);
    assert_ne!(
        bincode::encode_to_vec(SetShareMode { mode }, tutorlolv2_types::BINCODE_CONFIG).unwrap(),
        bincode::encode_to_vec(
            SetShareMode {
                mode: ShareMode::Live
            },
            tutorlolv2_types::BINCODE_CONFIG
        )
        .unwrap()
    );
    assert_eq!(
        ShareMode::from(HistoryShareMode::Private),
        ShareMode::Private
    );
}

#[test]
fn history_page_roundtrip_keeps_cursor_optional() {
    let page = HistoryPage {
        items: vec![],
        next_before: None,
    };
    let bytes = bincode::encode_to_vec(&page, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    let (decoded, _): (HistoryPage, _) =
        bincode::decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(page, decoded);
}

fn summary() -> HistorySummary {
    HistorySummary {
        recording: RecordingSummary {
            id: [3; 16],
            champion: "monkeyking".into(),
            game_mode: GameMode::Other("NEW_MODE".into()),
            queue_id: Some(420),
            active_riot_id: "Host#TEST".into(),
            started_at_ms: 1700000000123,
            first_game_time: 30.0,
            last_game_time: 600.0,
            game_length_s: Some(700),
            result: Some(GameResult::Win),
            is_custom: Some(false),
            is_practice: false,
        },
        ended_at_ms: Some(1700000700123),
        end_reason: Some(EndReason::GameEndEvent),
        coverage_s: 400,
        kills: 3,
        deaths: 2,
        assists: 300,
        creep_score: 220,
        level: 18,
    }
}

#[test]
fn owner_details_roundtrip_preserves_wide_scores_unknown_modes_and_coverage() {
    let details = RecordingDetails {
        summary: summary(),
        chunks: vec![ChunkInfo {
            seq: 7,
            first_game_time: 30.0,
            last_game_time: 90.0,
            snapshot_count: 61,
            frames_format: FRAMES_FORMAT,
        }],
        share: ShareState {
            mode: ShareMode::Private,
            code: None,
            url: None,
        },
        frames_format: FRAMES_FORMAT,
    };
    let bytes = bincode::encode_to_vec(&details, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    let (decoded, used): (RecordingDetails, _) =
        bincode::decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(decoded, details);
    let entry = HistoryEntry {
        local_id: Some([2; 16]),
        recording_id: Some([3; 16]),
        summary: Some(summary()),
        share: details.share.clone(),
        pending_chunks: 2,
        pending_control: true,
        lost_chunks: 1,
        last_error: Some("retrying".into()),
        hidden: true,
    };
    let native = HistoryDetails {
        recording: details,
        entry,
    };
    let json = serde_json::to_vec(&native).unwrap();
    assert_eq!(
        serde_json::from_slice::<HistoryDetails>(&json).unwrap(),
        native
    );
}

#[test]
fn history_ipc_modes_and_errors_have_typed_external_names() {
    assert_eq!(
        serde_json::to_string(&HistoryShareMode::Public).unwrap(),
        "\"public\""
    );
    assert!(serde_json::from_str::<HistoryShareMode>("\"live\"").is_err());
    let error = HistoryError {
        kind: HistoryErrorKind::UnsupportedFormat,
        message: "Update to view this recording".into(),
        retryable: false,
    };
    let json = serde_json::to_value(&error).unwrap();
    assert_eq!(json["kind"], "unsupported_format");
    assert_eq!(serde_json::from_value::<HistoryError>(json).unwrap(), error);
}
