use lazy_regex::regex;
use tutorlolv2_build_dep::generators::{constants::Constants, utils::RegExtractor};

#[test]
fn capture_returns_each_group_in_order() {
    let text = "Deal 5% increased damage while below 60% maximum health";

    let [damage, health] = text
        .capture(regex!(r"Deal (\d+)% increased damage while below (\d+)%"))
        .unwrap();

    assert_eq!((damage, health), (5.0, 60.0));
}

#[test]
fn capture_fails_when_the_wiki_text_changes() {
    let text = "Reduces all incoming basic damage by 10%";

    assert!(
        text.capture::<1>(regex!(r"critical strikes by (\d+)%"))
            .is_err()
    );
    // A pattern with a different number of groups than expected is an error
    assert!(text.capture::<2>(regex!(r"by (\d+)%")).is_err());
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

#[test]
fn constants_that_modes_change_get_a_function_of_the_map() {
    use tutorlolv2_build_dep::generators::constants::Constant;
    use tutorlolv2_types::MapMode;

    let mut constants = Constants::default();

    constants.insert("RATIO", 0.1).unwrap();
    constants.insert("STEADY", 3.0).unwrap();
    constants
        .insert_map("RATIO", vec![(MapMode::Arena, Constant::Scalar(0.2))])
        .unwrap();
    // A mode with the same value has no arm, and no mode still gets a function
    constants
        .insert_map("STEADY", vec![(MapMode::Aram, Constant::Scalar(3.0))])
        .unwrap();

    let baked = constants.bake();

    assert!(baked.contains("pub const fn ratio(map: tutorlolv2_types::MapMode) -> f32"));
    assert!(baked.contains("tutorlolv2_types::MapMode::Arena => { core::hint::cold_path(); 0.2 }"));
    assert!(baked.contains("_ => RATIO"));
    assert!(
        baked.contains("pub const fn steady(_map: tutorlolv2_types::MapMode) -> f32 { STEADY }")
    );
}

#[test]
fn map_values_need_their_constant_and_its_shape() {
    use tutorlolv2_build_dep::generators::constants::Constant;
    use tutorlolv2_types::MapMode;

    let mut constants = Constants::default();

    assert!(constants.insert_map("MISSING", vec![]).is_err());

    constants.insert("RANKED", [1.0, 2.0]).unwrap();
    assert!(
        constants
            .insert_map("RANKED", vec![(MapMode::Aram, Constant::Scalar(1.0))])
            .is_err()
    );
    assert!(
        constants
            .insert_map(
                "RANKED",
                vec![(MapMode::Default, Constant::Ranked(vec![1.0, 2.0]))]
            )
            .is_err()
    );
}
