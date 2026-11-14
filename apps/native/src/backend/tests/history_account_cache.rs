use crate::backend::{history::*, store::Store};
#[test]
fn rebinding_owner_cancels_invalidated_refresh_even_when_identity_is_unchanged() {
    let dir = std::env::temp_dir().join(format!("m4-cache-rebind-{}", std::process::id()));
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    assert!(history.begin(false));
    history.bind_owner(Some([1; 16])).unwrap();
    assert!(
        history.begin(false),
        "replacement client must be able to refresh after rollback"
    );
    let _ = std::fs::remove_dir_all(dir);
}
#[test]
fn owner_changes_drop_remote_metadata_but_preserve_local_hide_and_progress() {
    let dir = std::env::temp_dir().join(format!("m4-cache-{}", std::process::id()));
    let history = History::new(Store::new(dir.clone()), Some([1; 16]));
    history.local([2; 16], Some([3; 16]), 4, true, 2).unwrap();
    history.hide([3; 16]).unwrap();
    {
        let mut state = history.state.lock().unwrap();
        let mut remote = state.index.entries[0].clone();
        remote.local_id = None;
        remote.recording_id = Some([8; 16]);
        remote.hidden = false;
        state.index.entries.push(remote);
    }
    history.bind_owner(Some([9; 16])).unwrap();
    assert!(history.check_device(Some([1; 16])).is_err());
    assert!(history.list(0, 50).entries.is_empty());
    {
        let state = history.state.lock().unwrap();
        assert_eq!(state.index.entries.len(), 1);
        let row = &state.index.entries[0];
        assert!(row.hidden);
        assert_eq!(row.pending_chunks, 4);
        assert_eq!(row.lost_chunks, 2);
    }
    let restored = History::new(Store::new(dir.clone()), Some([9; 16]));
    assert!(restored.list(0, 50).entries.is_empty());
    assert_eq!(restored.state.lock().unwrap().index.entries.len(), 1);
    std::fs::create_dir(dir.join("history.bin.tmp")).unwrap();
    assert!(history.bind_owner(Some([7; 16])).is_err());
    assert!(history.check_device(Some([9; 16])).is_err());
    assert!(history.check_device(Some([7; 16])).is_ok());
    let _ = std::fs::remove_dir_all(dir);
}
