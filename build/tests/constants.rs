use tutorlolv2_build_dep::generators::{constants::Constants, utils::RegExtractor};

#[test]
fn capture_returns_each_group_in_order() {
    let text = "Deal 5% increased damage while below 60% maximum health";

    let [damage, health] = text
        .capture(r"Deal (\d+)% increased damage while below (\d+)%")
        .unwrap();

    assert_eq!((damage, health), (5.0, 60.0));
}

#[test]
fn capture_fails_when_the_wiki_text_changes() {
    let text = "Reduces all incoming basic damage by 10%";

    assert!(text.capture::<1>(r"critical strikes by (\d+)%").is_err());
    // A pattern with a different number of groups than expected is an error
    assert!(text.capture::<2>(r"by (\d+)%").is_err());
}

#[test]
fn constants_are_baked_as_f32_declarations() {
    let mut constants = Constants::default();

    constants.insert("SCALAR", 0.1).unwrap();
    constants.insert("RANKED", [80.0, 120.0, 160.0]).unwrap();

    assert_eq!(
        constants.bake(),
        "pub const RANKED: [f32; 3] = [80.0, 120.0, 160.0];\n\
         pub const SCALAR: f32 = 0.1;\n"
    );
}

#[test]
fn constants_reject_invalid_or_repeated_names() {
    let mut constants = Constants::default();

    assert!(constants.insert("lower_case", 1.0).is_err());
    assert!(constants.insert("EMPTY", Vec::new()).is_err());
    assert!(constants.insert("ONCE", 1.0).is_ok());
    assert!(constants.insert("ONCE", 2.0).is_err());
}
