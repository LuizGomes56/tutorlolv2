// Generate fields "melee" and "ranged" for each item in "internal/items/*.json".

use crate::{
    model::items::CdnItem,
    setup::{
        generators::extractors::{extract_damagelike_expr, process_scaled_string},
        helpers::{SetupError, read_json_file},
    },
};

/// Nashor's Tooth
fn _item_3115() -> Result<(), SetupError> {
    let _value = read_json_file::<CdnItem>("cache/cdn/items/3115.json")?;
    let _v = extract_damagelike_expr(&_value.passives[0].effects);
    let _r = process_scaled_string(&_v);
    println!("{}", _r);
    Ok(())
}

/// Blade of the Ruined King
fn _item_3153() -> Result<(), SetupError> {
    todo!()
}

// Shadowflame
fn _item_4645() -> Result<(), SetupError> {
    todo!()
}

pub fn assign_item_damages() -> Result<(), SetupError> {
    _item_3115()?;
    Ok(())
}
