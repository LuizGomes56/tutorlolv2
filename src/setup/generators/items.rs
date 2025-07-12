#![allow(unused_imports, dead_code)]
//! Generate fields "melee" and "ranged" for each item in "internal/items/*.json".

use crate::{
    essentials::helpers::{read_json_file, write_to_file},
    model::{
        dev::items::CdnItem,
        items::{DamageObject, Item},
    },
    setup::generators::extractors::{extract_damagelike_expr, process_scaled_string},
};
use regex::Regex;

enum DmgType {
    Magic,
    Physicical,
    True,
    Mixed,
    Adaptative,
}

impl DmgType {
    fn stringify(&self) -> &'static str {
        match self {
            DmgType::Magic => "MAGIC_DAMAGE",
            DmgType::Physicical => "PHYSICAL_DAMAGE",
            DmgType::True => "TRUE_DAMAGE",
            DmgType::Mixed => "MIXED_DAMAGE",
            DmgType::Adaptative => "ADAPTATIVE_DAMAGE",
        }
    }
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
/// auto-generated content
fn __test_item_dmg() -> TestResult {
    let _ = item_1043();
    let _ = item_2015();
    let _ = item_2502();
    let _ = item_2503();
    let _ = item_3032();
    let _ = item_3042();
    let _ = item_3050();
    let _ = item_3057();
    let _ = item_3068();
    let _ = item_3074();
    let _ = item_3075();
    let _ = item_3076();
    let _ = item_3077();
    let _ = item_3078();
    let _ = item_3084();
    let _ = item_3085();
    let _ = item_3087();
    let _ = item_3091();
    let _ = item_3094();
    let _ = item_3100();
    let _ = item_3107();
    let _ = item_3115();
    let _ = item_3118();
    let _ = item_3124();
    let _ = item_3128();
    let _ = item_3145();
    let _ = item_3146();
    let _ = item_3152();
    let _ = item_3153();
    let _ = item_3181();
    let _ = item_3302();
    let _ = item_3504();
    let _ = item_3742();
    let _ = item_3748();
    let _ = item_3870();
    let _ = item_3871();
    let _ = item_3877();
    let _ = item_4005();
    let _ = item_4017();
    let _ = item_4636();
    let _ = item_4637();
    let _ = item_4646();
    let _ = item_6029();
    let _ = item_6610();
    let _ = item_6630();
    let _ = item_6631();
    let _ = item_6632();
    let _ = item_6653();
    let _ = item_6655();
    let _ = item_6656();
    let _ = item_6660();
    let _ = item_6662();
    let _ = item_6664();
    let _ = item_6671();
    let _ = item_6672();
    let _ = item_6677();
    let _ = item_6692();
    let _ = item_6693();
    let _ = item_6698();
    let _ = item_6699();
    let _ = item_7001();
    let _ = item_7004();
    let _ = item_7005();
    let _ = item_7006();
    let _ = item_7007();
    let _ = item_7010();
    let _ = item_7011();
    let _ = item_7012();
    let _ = item_7013();
    let _ = item_7014();
    let _ = item_7015();
    let _ = item_7016();
    let _ = item_7017();
    let _ = item_7018();
    let _ = item_7025();
    let _ = item_7026();
    let _ = item_7030();
    let _ = item_7034();
    let _ = item_7039();
    let _ = item_7040();
    let _ = item_7041();
    let _ = item_7042();
    let _ = item_221043();
    let _ = item_222015();
    let _ = item_223042();
    let _ = item_223057();
    let _ = item_223068();
    let _ = item_223074();
    let _ = item_223075();
    let _ = item_223076();
    let _ = item_223077();
    let _ = item_223078();
    let _ = item_223084();
    let _ = item_223085();
    let _ = item_223087();
    let _ = item_223091();
    let _ = item_223094();
    let _ = item_223095();
    let _ = item_223100();
    let _ = item_223107();
    let _ = item_223115();
    let _ = item_223124();
    let _ = item_223145();
    let _ = item_223146();
    let _ = item_223152();
    let _ = item_223153();
    let _ = item_223181();
    let _ = item_223504();
    let _ = item_223508();
    let _ = item_223742();
    let _ = item_223748();
    let _ = item_224005();
    let _ = item_224636();
    let _ = item_224637();
    let _ = item_226029();
    let _ = item_226630();
    let _ = item_226632();
    let _ = item_226653();
    let _ = item_226655();
    let _ = item_226656();
    let _ = item_226660();
    let _ = item_226662();
    let _ = item_226671();
    let _ = item_226672();
    let _ = item_226677();
    let _ = item_226692();
    let _ = item_226693();
    let _ = item_227001();
    let _ = item_227005();
    let _ = item_227006();
    let _ = item_227010();
    let _ = item_227011();
    let _ = item_227012();
    let _ = item_227013();
    let _ = item_227014();
    let _ = item_227015();
    let _ = item_227016();
    let _ = item_227017();
    let _ = item_227018();
    let _ = item_227025();
    let _ = item_227026();
    let _ = item_227030();
    Ok(())
}

pub fn assign_item_damages() -> TestResult {
    item_3115()?;
    Ok(())
}

/// Nashor's Tooth
#[generator_macros::item_generator]
fn item_3115() -> TestResult {
    write_type!(DmgType::Magic);
    let damage = extract_damagelike_expr(&cdn_value.passives[0].effects);
    write_dmg!(damage);
    save_change!(cur_value)
}

/// Blade of the Ruined King
/// [Context](https://tutorlol.com/formulas/)
#[generator_macros::item_generator]
fn item_3153() -> TestResult {
    write_type!(DmgType::Magic);
    let damagelike_expr = extract_damagelike_expr(&cdn_value.passives[0].effects);
    let numbers = Regex::new(r"(\d+)%")
        .unwrap()
        .captures_iter(&damagelike_expr)
        .map(|cap| cap.get(1).unwrap().as_str().parse::<f64>().unwrap() / 100.0)
        .collect::<Vec<f64>>();
    let get_dmg = |number: f64| {
        let power = 1;
        format!(
            "ENEMY_HEALTH - (({number} * ENEMY_HEALTH * (1 - {number} * PHYSICAL_MULTIPLIER).powf({power}) - AD + AD * (1 - {number} * PHYSICAL_MULTIPLIER).powf({power})) / {number})"
        )
    };
    let min_dmg = get_dmg(numbers[0]);
    let max_dmg = get_dmg(numbers[1]);
    write_dmg!(min_dmg, max_dmg);
    save_change!(cur_value)
}

#[generator_macros::item_generator]
fn item_1043() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_2015() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_2502() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_2503() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3032() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3042() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3050() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3057() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3068() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3074() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3075() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3076() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3077() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3078() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3084() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3085() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3087() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3091() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3094() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3100() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3107() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3118() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3124() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3128() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3145() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3146() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3152() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3181() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3302() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3504() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3742() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3748() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3870() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3871() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_3877() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_4005() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_4017() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_4636() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_4637() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_4646() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6029() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6610() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6630() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6631() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6632() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6653() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6655() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6656() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6660() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6662() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6664() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6671() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6672() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6677() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6692() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6693() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6698() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_6699() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7001() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7004() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7005() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7006() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7007() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7010() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7011() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7012() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7013() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7014() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7015() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7016() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7017() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7018() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7025() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7026() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7030() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7034() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7039() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7040() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7041() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_7042() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_221043() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_222015() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223042() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223057() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223068() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223074() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223075() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223076() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223077() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223078() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223084() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223085() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223087() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223091() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223094() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223095() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223100() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223107() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223115() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223124() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223145() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223146() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223152() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223153() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223181() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223504() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223508() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223742() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_223748() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_224005() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_224636() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_224637() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226029() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226630() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226632() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226653() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226655() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226656() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226660() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226662() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226671() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226672() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226677() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226692() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_226693() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227001() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227005() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227006() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227010() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227011() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227012() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227013() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227014() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227015() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227016() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227017() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227018() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227025() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227026() -> TestResult {
    Ok(())
}

#[generator_macros::item_generator]
fn item_227030() -> TestResult {
    Ok(())
}
