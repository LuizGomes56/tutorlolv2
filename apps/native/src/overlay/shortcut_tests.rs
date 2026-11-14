use super::shortcut::*;

#[test]
fn escape_still_closes_after_a_missed_release() {
    let mut state = KeyboardState::new();
    // A game or another hook may swallow key-up before focus switches.
    assert_eq!(state.process(0x1b, true, 0x24b, false, false).action, None);
    assert_eq!(
        state.process(0x1b, true, 0x24b, false, true).action,
        Some(Action::Close)
    );
    assert_eq!(state.process(0x1b, true, 0x24b, true, true).action, None);
    assert!(state.process(0x1b, false, 0x24b, false, false).consume);
}

#[test]
fn physical_binding_survives_non_us_layouts_and_distinguishes_numpad() {
    // On AZERTY the physical KeyQ position reports virtual A.
    assert_eq!(physical_key(0x41, 0x10, false), 0x51);
    assert_eq!(physical_key(0x51, 0x1e, false), 0x41);
    assert_eq!(physical_key(0x23, 0x4f, false), 0xff);
    assert_eq!(physical_key(0x23, 0x4f, true), 0x23);
    assert_eq!(physical_key(0x0d, 0x1c, true), 0xff);
}

fn single(code: &str) -> Shortcut {
    Shortcut {
        code: code.into(),
        ctrl: false,
        ..Shortcut::default()
    }
}

#[test]
fn validates_physical_keys_and_reserves_close_and_cycle() {
    assert_eq!(Shortcut::default().validate().unwrap(), 0x24b);
    assert_eq!(single("F8").validate().unwrap(), 0x77);
    assert_eq!(single("Digit0").validate().unwrap(), 0x30);
    assert_eq!(single("Backquote").validate().unwrap(), 0xc0);
    for code in [
        "Escape",
        "ControlLeft",
        "Key",
        "F0",
        "F25",
        "",
        "Unidentified",
    ] {
        assert!(single(code).validate().is_err(), "{code}");
    }
    assert!(
        Shortcut {
            code: "KeyO".into(),
            ..Shortcut::default()
        }
        .validate()
        .is_err()
    );
}

#[test]
fn toggles_once_per_press_and_consumes_the_whole_trigger() {
    let mut state = KeyboardState::new();
    state.process(0xa2, true, 0x24b, false, false);
    assert_eq!(
        state.process(0x4b, true, 0x24b, false, false),
        Decision {
            action: Some(Action::Toggle),
            consume: true
        }
    );
    assert_eq!(
        state.process(0x4b, true, 0x24b, false, true),
        Decision {
            action: None,
            consume: true
        }
    );
    state.process(0xa2, false, 0x24b, false, true);
    assert_eq!(
        state.process(0x4b, false, 0x77, false, true),
        Decision {
            action: None,
            consume: true
        }
    );
    assert_eq!(
        state.process(0x4b, true, 0x24b, false, true),
        Decision::default()
    );
}

#[test]
fn ignores_extra_modifiers_and_rebinding_a_held_key() {
    let mut state = KeyboardState::new();
    state.process(0xa2, true, 0x24b, false, false);
    state.process(0xa0, true, 0x24b, false, false);
    assert_eq!(
        state.process(0x4b, true, 0x24b, false, false),
        Decision::default()
    );
    state.process(0xa0, false, 0x24b, false, false);
    assert_eq!(
        state.process(0x4b, true, 0x24b, false, false),
        Decision::default()
    );
    state.process(0x4b, false, 0x24b, false, false);
    assert_eq!(
        state.process(0x4b, true, 0x24b, false, false).action,
        Some(Action::Toggle)
    );
}

#[test]
fn closes_only_when_open_and_suspends_all_actions_during_recording() {
    let mut state = KeyboardState::new();
    assert_eq!(
        state.process(0x1b, true, 0x77, false, false),
        Decision::default()
    );
    state.process(0x1b, false, 0x77, false, false);
    assert_eq!(
        state.process(0x1b, true, 0x77, false, true).action,
        Some(Action::Close)
    );
    state.process(0x1b, false, 0x77, false, false);
    assert_eq!(
        state.process(0x77, true, 0x77, true, false),
        Decision::default()
    );
    assert_eq!(
        state.process(0x77, true, 0x77, false, false),
        Decision::default()
    );
    state.process(0x77, false, 0x77, false, false);
    assert_eq!(
        state.process(0x77, true, 0x77, false, false).action,
        Some(Action::Toggle)
    );
}

#[test]
fn preserves_cycle_and_handles_both_control_keys() {
    let mut state = KeyboardState::new();
    state.process(0xa2, true, 0x77, false, false);
    state.process(0xa3, true, 0x77, false, false);
    state.process(0xa2, false, 0x77, false, false);
    assert_eq!(
        state.process(0x4f, true, 0x77, false, false).action,
        Some(Action::Change)
    );
}

#[test]
fn saves_and_loads_shortcut_and_rejects_corrupt_settings() {
    let directory =
        std::env::temp_dir().join(format!("tutorlol-shortcut-test-{}", std::process::id()));
    let path = directory.join("shortcut.json");
    assert_eq!(Shortcut::load(&path).unwrap(), Shortcut::default());
    let binding = single("F8");
    binding.save(&path).unwrap();
    assert_eq!(Shortcut::load(&path).unwrap(), binding);
    assert!(single("Escape").save(&path).is_err());
    assert_eq!(Shortcut::load(&path).unwrap(), binding);
    std::fs::write(&path, "broken").unwrap();
    assert!(Shortcut::load(&path).is_err());
    std::fs::remove_dir_all(directory).unwrap();
}
