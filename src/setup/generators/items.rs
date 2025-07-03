//! Generate fields "melee" and "ranged" for each item in "internal/items/*.json".

use crate::{
    model::items::{CdnItem, DamageObject, Item},
    setup::{
        generators::extractors::{extract_damagelike_expr, process_scaled_string},
        helpers::{SetupError, read_json_file, write_to_file},
    },
};
use regex::Regex;

#[test]
/// auto-generated content
fn __test_item_dmg() -> Result<(), SetupError> {
    item_1043()?;
    item_2015()?;
    item_2502()?;
    item_2503()?;
    item_3032()?;
    item_3042()?;
    item_3050()?;
    item_3057()?;
    item_3068()?;
    item_3074()?;
    item_3075()?;
    item_3076()?;
    item_3077()?;
    item_3078()?;
    item_3084()?;
    item_3085()?;
    item_3087()?;
    item_3091()?;
    item_3094()?;
    item_3100()?;
    item_3107()?;
    item_3115()?;
    item_3118()?;
    item_3124()?;
    item_3128()?;
    item_3145()?;
    item_3146()?;
    item_3152()?;
    item_3153()?;
    item_3181()?;
    item_3302()?;
    item_3504()?;
    item_3742()?;
    item_3748()?;
    item_3870()?;
    item_3871()?;
    item_3877()?;
    item_4005()?;
    item_4017()?;
    item_4636()?;
    item_4637()?;
    item_4646()?;
    item_6029()?;
    item_6610()?;
    item_6630()?;
    item_6631()?;
    item_6632()?;
    item_6653()?;
    item_6655()?;
    item_6656()?;
    item_6660()?;
    item_6662()?;
    item_6664()?;
    item_6671()?;
    item_6672()?;
    item_6677()?;
    item_6692()?;
    item_6693()?;
    item_6698()?;
    item_6699()?;
    item_7001()?;
    item_7004()?;
    item_7005()?;
    item_7006()?;
    item_7007()?;
    item_7010()?;
    item_7011()?;
    item_7012()?;
    item_7013()?;
    item_7014()?;
    item_7015()?;
    item_7016()?;
    item_7017()?;
    item_7018()?;
    item_7025()?;
    item_7026()?;
    item_7030()?;
    item_7034()?;
    item_7039()?;
    item_7040()?;
    item_7041()?;
    item_7042()?;
    item_221043()?;
    item_222015()?;
    item_223042()?;
    item_223057()?;
    item_223068()?;
    item_223074()?;
    item_223075()?;
    item_223076()?;
    item_223077()?;
    item_223078()?;
    item_223084()?;
    item_223085()?;
    item_223087()?;
    item_223091()?;
    item_223094()?;
    item_223095()?;
    item_223100()?;
    item_223107()?;
    item_223115()?;
    item_223124()?;
    item_223145()?;
    item_223146()?;
    item_223152()?;
    item_223153()?;
    item_223181()?;
    item_223504()?;
    item_223508()?;
    item_223742()?;
    item_223748()?;
    item_224005()?;
    item_224636()?;
    item_224637()?;
    item_226029()?;
    item_226630()?;
    item_226632()?;
    item_226653()?;
    item_226655()?;
    item_226656()?;
    item_226660()?;
    item_226662()?;
    item_226671()?;
    item_226672()?;
    item_226677()?;
    item_226692()?;
    item_226693()?;
    item_227001()?;
    item_227005()?;
    item_227006()?;
    item_227010()?;
    item_227011()?;
    item_227012()?;
    item_227013()?;
    item_227014()?;
    item_227015()?;
    item_227016()?;
    item_227017()?;
    item_227018()?;
    item_227025()?;
    item_227026()?;
    item_227030()?;
    Ok(())
}

pub fn assign_item_damages() -> Result<(), SetupError> {
    item_3115()?;
    Ok(())
}

/// Nashor's Tooth
#[writer_macros::item_generator]
fn item_3115() -> Result<(), SetupError> {
    let damage = extract_damagelike_expr(&cdn_value.passives[0].effects);
    write_dmg!(damage);
    save_change!(cur_value)
}

/// Blade of the Ruined King
/// [Context](https://tutorlol.com/formulas/)
#[writer_macros::item_generator]
fn item_3153() -> Result<(), SetupError> {
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

#[writer_macros::item_generator]
fn item_1043() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_2015() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_2502() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_2503() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3032() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3042() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3050() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3057() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3068() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3074() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3075() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3076() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3077() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3078() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3084() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3085() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3087() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3091() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3094() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3100() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3107() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3118() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3124() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3128() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3145() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3146() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3152() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3181() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3302() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3504() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3742() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3748() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3870() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3871() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_3877() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_4005() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_4017() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_4636() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_4637() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_4646() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6029() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6610() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6630() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6631() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6632() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6653() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6655() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6656() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6660() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6662() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6664() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6671() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6672() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6677() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6692() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6693() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6698() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_6699() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7001() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7004() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7005() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7006() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7007() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7010() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7011() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7012() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7013() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7014() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7015() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7016() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7017() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7018() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7025() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7026() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7030() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7034() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7039() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7040() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7041() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_7042() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_221043() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_222015() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223042() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223057() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223068() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223074() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223075() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223076() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223077() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223078() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223084() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223085() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223087() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223091() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223094() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223095() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223100() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223107() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223115() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223124() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223145() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223146() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223152() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223153() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223181() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223504() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223508() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223742() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_223748() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_224005() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_224636() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_224637() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226029() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226630() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226632() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226653() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226655() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226656() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226660() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226662() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226671() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226672() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226677() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226692() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_226693() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227001() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227005() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227006() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227010() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227011() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227012() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227013() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227014() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227015() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227016() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227017() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227018() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227025() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227026() -> Result<(), SetupError> {
    Ok(())
}

#[writer_macros::item_generator]
fn item_227030() -> Result<(), SetupError> {
    Ok(())
}
