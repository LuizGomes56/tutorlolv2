use {
    codec::{FormulaDb, FormulaDbBuilder, FormulaSource},
    std::collections::HashMap,
};

const CTX: [&str; 5] = [
    "ability_power",
    "level",
    "attack_damage",
    "q_level",
    "bonus_ad",
];
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
    assert_eq!(
        render(with_levels).unwrap(),
        with_levels.replace("ctx.", "")
    );
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

#[test]
fn numeric_tables_round_trip_in_arithmetic_and_map_arms() {
    let table = "{ let axis: f32 = ctx.level; if axis < 6.5 { 20 + 15 * (axis - 1).max(0.0) / 5.5 } else if axis < 12.25 { 35 } else { 50 } }";
    for source in [
        table.to_owned(),
        format!("2 * ({table}) + 5"),
        format!("match ctx.map {{ #[cold] Aram => {table}, _ => 3 }}"),
    ] {
        assert_eq!(render(&source).unwrap(), source.replace("ctx.", ""));
    }
}

#[test]
fn malformed_numeric_tables_remain_errors() {
    for source in [
        "axis",
        "{ let axis: f32 = ctx.level; if axis < 6 { arbitrary(ctx) } else { 50 } }",
        "{ let axis: f32 = ctx.level; if axis < ctx.level { 20 } else { 50 } }",
        "{ let axis: f32 = ctx.level; if axis < 6 { (axis - 1).max(2) } else { 50 } }",
        "{ let axis: f32 = ctx.level; if axis < 6 { 20 } else { 50 }",
    ] {
        assert!(render(source).is_err(), "{source}");
    }
}

#[test]
fn numeric_table_bytecode_corruption_is_rejected() {
    let mut builder = FormulaDbBuilder::new(1, 0, 0, ctx, map);
    builder
        .push_champion(
            0,
            &[FormulaSource {
                local: 0,
                source: "{ let axis: f32 = ctx.level; if axis < 6.5 { 20 } else { 50 } }",
            }],
            &HashMap::new(),
        )
        .unwrap();
    let mut bytes = builder.finish().unwrap();
    // The final numeric literal is an immediate u8; replace its opcode.
    let end = bytes.len();
    bytes[end - 2] = 0xff;
    let db = FormulaDb::new(&bytes).unwrap();
    assert!(
        db.render_formula_plain(
            0,
            |c| CTX[c as usize].into(),
            |m| MAPS[m as usize].into(),
            |_| unreachable!()
        )
        .is_err()
    );
}

#[test]
fn numeric_table_map_arm_does_not_require_a_comma() {
    // Kraken Slayer's Arena table is a block expression, so rustfmt omits its comma.
    let source = "match ctx.map {
    #[cold]
    Aram => match ctx.level { ..9 => 262.5, 9.. => 350 },
    #[cold]
    Arena => { let axis: f32 = 100 * (ctx.level / 100); if axis < 6.67 { 0 } else if axis < 13.33 { 5 } else if axis < 100 { 70 } else { 75 } }
    _ => match ctx.level { ..9 => 150, 9.. => 210 },
}";
    assert_eq!(render(source).unwrap(), source.replace("ctx.", ""));
    assert!(render("match ctx.map { #[cold] Arena => 5 _ => 1 }").is_err());
}

#[test]
fn affine_level_lower_bounds_round_trip_outside_numeric_table_blocks() {
    let expression = "10 + 3 * (ctx.level - 1).max(0.0)";
    for source in [
        expression.to_string(),
        format!("match ctx.map {{ #[cold] Arena => {expression}, _ => 10 }}"),
    ] {
        assert_eq!(render(&source).unwrap(), source.replace("ctx.", ""));
    }
    assert!(render("(ctx.level - 1).max(2)").is_err());
    assert!(render("axis.max(0.0)").is_err());
}
