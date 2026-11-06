use crate::backend::delivery::Delivery;
use crate::backend::{
    recorder::{Recorder, SessionFile},
    spool::Spool,
    store::Store,
    uploader::ApiClient,
};
use tutorlolv2_protocol::{GameMode, Id16, Probe};

fn recorder(id: Option<Id16>) -> Recorder {
    Recorder::new(
        id.map(|id| SessionFile {
            local_id: id,
            server: Some((id, 0)),
            probe: Probe {
                active_riot_id: "test#1".into(),
                game_mode: GameMode::Classic,
                map_number: 11,
                roster: vec![],
                game_time: 60.0,
                events: vec![],
                riot_game: None,
            },
            last_seen_ms: 1000,
            next_local_seq: 0,
            ended: false,
            pending_end: None,
            lcu_game_id: None,
        }),
        None,
        "test".into(),
    )
}

#[test]
fn share_command_error_is_discarded_when_its_recording_is_no_longer_active() {
    for next_id in [None, Some([8; 16])] {
        let path = std::env::temp_dir().join("tutorlolv2-command-error-unit");
        let delivery = Delivery::new(
            recorder(Some([9; 16])),
            ApiClient::new(None),
            Spool::new(path.join("spool"), Spool::DEFAULT_MAX_BYTES),
            Store::new(path),
        );
        *delivery.sharing_error.lock().unwrap() = Some(([9; 16], "command failed".into()));
        assert_eq!(
            delivery.recording_state().upload.last_error.as_deref(),
            Some("command failed")
        );
        *delivery.recorder.lock().unwrap() = recorder(next_id);
        *delivery.live_error.lock().unwrap() = Some("current publication failed".into());
        assert_eq!(
            delivery.recording_state().upload.last_error.as_deref(),
            Some("current publication failed")
        );
        assert!(delivery.sharing_error.lock().unwrap().is_none());
    }
}
