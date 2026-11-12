use {
    std::collections::HashMap,
    tutorlolv2_codec::{FormulaDb, FormulaDbBuilder, FormulaSource},
};

const CTX: [&str; 5] = ["ability_power", "level", "attack_damage", "q_level", "bonus_ad"];
const MAPS: [&str; 3] = ["Default", "Aram", "Arena"];

fn ctx(name: &str) -> Option<u8> {
    CTX.iter().position(|n| *n == name).map(|i| i as u8)
}

fn map(name: &str) -> Option<u8> {
    MAPS.iter().position(|n| *n == name).map(|i| i as u8)
}

/// Parses, encodes and renders `source` as the only formula of a champion
fn render(source: &str) -> Result<String, String> {
    let mut builder = FormulaDbBuilder::new(1, 0, 0, ctx, map);
    builder
        .push_champion(0, &[FormulaSource { local: 0, source }], &HashMap::new())
        .map_err(|e| format!("{e:?}"))?;

    let bytes = builder.finish().map_err(|e| format!("{e:?}"))?;
    let db = FormulaDb::new(&bytes).map_err(|e| format!("{e:?}"))?;

    db.render_formula_plain(
        0,
        |c| CTX[c as usize].into(),
        |m| MAPS[m as usize].into(),
        |_| unreachable!(),
    )
    .map_err(|e| format!("{e:?}"))
}

#[test]
fn whole_formula_and_value_level_map_matches_round_trip() {
    let whole = "match ctx.map { #[cold] Aram => 0.65 * ctx.ability_power, #[cold] Arena => 0.7 * ctx.ability_power, _ => 0.6 * ctx.ability_power }";
    assert_eq!(render(whole).unwrap(), whole.replace("ctx.", ""));

    let nested = "(match ctx.map { #[cold] Aram => 0.65, _ => 0.6 }) * ctx.ability_power";
    assert_eq!(render(nested).unwrap(), nested.replace("ctx.", ""));

    let with_levels =
        "match ctx.map { #[cold] Aram => match ctx.level { ..6 => 1, 6.. => 2 }, _ => 3 }";
    assert_eq!(render(with_levels).unwrap(), with_levels.replace("ctx.", ""));
}

#[test]
fn block_map_matches_keep_their_layout() {
    // As rustfmt writes them: `#[cold]` on its own line above its mode
    let block = "match ctx.map {\n    #[cold]\n    Aram => 0.65,\n    #[cold]\n    Arena => 0.7,\n    _ => 0.6,\n}";
    assert_eq!(render(block).unwrap(), block.replace("ctx.", ""));
}

#[test]
fn braced_arm_bodies_keep_their_layout() {
    // Akali Q as rustfmt formats its doc source: long arms become blocks, without a comma
    let block = "match ctx.map {
    #[cold]
    Arena => {
        55 + 40 * (ctx.q_level - 1)
            + 0.9 * ctx.bonus_ad
    }
    _ => {
        45 + 30 * (ctx.q_level - 1)
            + 0.7 * ctx.bonus_ad
    }
}";
    assert_eq!(render(block).unwrap(), block.replace("ctx.", ""));

    // A short arm next to a braced one keeps its comma
    let mixed = "match ctx.map {
    #[cold]
    Aram => 0.65,
    _ => {
        45 + 30 * (ctx.q_level - 1)
            + 0.7 * ctx.bonus_ad
    }
}";
    assert_eq!(render(mixed).unwrap(), mixed.replace("ctx.", ""));
}

#[test]
fn invalid_map_matches_are_errors() {
    // An unknown mode, an arm without `#[cold]`, and no default arm
    assert!(render("match ctx.map { #[cold] Nexus => 1, _ => 2 }").is_err());
    assert!(render("match ctx.map { Aram => 1, _ => 2 }").is_err());
    assert!(render("match ctx.map { #[cold] Aram => 1 }").is_err());
}
