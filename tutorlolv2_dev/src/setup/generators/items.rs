#![allow(unused_imports, dead_code)]
//! Generate fields "melee" and "ranged" for each item in "internal/items/*.json".
//! Sort fields by their name to avoid same problem involving ordering effects

use crate::generators_v2::RegExtractor;

use super::*;
use regex::Regex;
use serde::Deserialize;
use tutorlolv2_gen::EvalIdent;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// auto-generated content
pub fn assign_item_damages() -> TestResult {
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

/// Nashor's Tooth
#[tutorlolv2_macros::item_generator(Magic, Onhit)]
fn item_3115() -> TestResult {
    write_dmg!(cdn_value.passives[0].effects.get_damage());
}

/// Blade of the Ruined King
/// [Context](https://tutorlol.com/formulas)
#[tutorlolv2_macros::item_generator(Physical)]
fn item_3153() -> TestResult {
    let damagelike_expr = extract_damagelike_expr(&cdn_value.passives[0].effects);
    let numbers = Regex::new(r"(\d+)%")
        .unwrap()
        .captures_iter(&damagelike_expr)
        .map(|cap| cap.get(1).unwrap().as_str().parse::<f64>().unwrap() / 100.0)
        .collect::<Vec<f64>>();
    let get_dmg = |number| {
        let power = 1;
        format!(
            "{ENEMY_HEALTH} - (({number} * {ENEMY_HEALTH} * (1 - {number} * {PHYSICAL_MULTIPLIER}).powf({power}) - {AD} + {AD} * (1 - {number} * {PHYSICAL_MULTIPLIER}).powf({power})) / {number})",
            ENEMY_HEALTH = EvalIdent::EnemyHealth,
            PHYSICAL_MULTIPLIER = EvalIdent::PhysicalMultiplier,
            AD = EvalIdent::Ad
        )
    };
    let melee_dmg = get_dmg(numbers[0]);
    let ranged_dmg = get_dmg(numbers[1]);
    write_dmg!(@ranged ranged_dmg, @melee melee_dmg);
}

#[tutorlolv2_macros::item_generator]
fn item_1043() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_2015() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_2502() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_2503() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3032() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3042() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3050() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3057() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3068() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3074() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3075() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3076() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3077() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3078() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3084() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3085() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3087() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3091() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3094() -> TestResult {}

/// Lich Bane
#[tutorlolv2_macros::item_generator(Magic, OnhitMax)]
fn item_3100() -> TestResult {
    let damage = cdn_value
        .passives
        .get(0)
        .ok_or("Index 0 is for cdn_value.passives is no longer valid")?
        .effects
        .get_damage();
    let dmg = format!(
        "{} + {} * {}",
        damage.capture_parens(0)?,
        damage.capture_percent(1)? / 100.0,
        EvalIdent::BaseAd
    );
    write_dmg!(dmg);
}

/// Redemption
#[tutorlolv2_macros::item_generator(True)]
fn item_3107() -> TestResult {
    let damage = extract_damagelike_expr(&cdn_value.active[0].effects);
    let dmg = format!(
        "{} * {}",
        damage.capture_percent(0)? / 100.0,
        EvalIdent::EnemyMaxHealth
    );
    write_dmg!(dmg);
}

/// Malignance
#[tutorlolv2_macros::item_generator(Magic)]
fn item_3118() -> TestResult {
    // let damage = extract_damagelike_expr(&cdn_value.passives[1].effects);
    // let numbers = cap_numbers!(damage);
    // let values = format!("{} + 0.0{} * AP", numbers[0], numbers[2]);
    // write_dmg!(values.clone(), format!("3 * ({values})"));
}

#[tutorlolv2_macros::item_generator]
fn item_3124() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3128() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3145() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3146() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3152() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3181() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3302() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3504() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3742() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3748() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3870() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3871() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_3877() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_4005() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_4017() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_4636() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_4637() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_4646() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6029() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6610() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6630() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6631() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6632() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6653() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6655() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6656() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6660() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6662() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6664() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6671() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6672() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6677() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6692() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6693() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6698() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_6699() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7001() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7004() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7005() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7006() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7007() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7010() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7011() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7012() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7013() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7014() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7015() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7016() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7017() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7018() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7025() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7026() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7030() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7034() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7039() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7040() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7041() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_7042() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_221043() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_222015() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223042() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223057() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223068() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223074() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223075() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223076() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223077() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223078() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223084() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223085() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223087() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223091() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223094() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223095() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223100() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223107() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223115() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223124() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223145() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223146() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223152() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223153() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223181() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223504() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223508() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223742() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_223748() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_224005() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_224636() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_224637() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226029() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226630() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226632() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226653() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226655() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226656() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226660() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226662() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226671() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226672() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226677() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226692() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_226693() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227001() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227005() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227006() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227010() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227011() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227012() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227013() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227014() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227015() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227016() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227017() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227018() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227025() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227026() -> TestResult {}

#[tutorlolv2_macros::item_generator]
fn item_227030() -> TestResult {}
