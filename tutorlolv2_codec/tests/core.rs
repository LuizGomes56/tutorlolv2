use std::collections::HashMap;
use tutorlolv2_codec::{Class, FormulaDb, FormulaDbBuilder, FormulaSource};

fn ctx_id(name: &str) -> Option<u8> {
    match name {
        "level" => Some(0),
        "ability_power" => Some(1),
        _ => None,
    }
}

#[test]
fn multiline_match_keeps_layout_and_drops_ctx_prefix() {
    let source = "(match ctx.level as u8 {\n    ..=1 => 5,\n    2 => 10,\n    3.. => 145,\n}) + 1.5 * ctx.ability_power";
    let mut builder = FormulaDbBuilder::new(1, 0, 0, ctx_id);
    builder
        .push_champion(0, &[FormulaSource { local: 0, source }], &HashMap::new())
        .unwrap();

    let bytes = builder.finish().unwrap();
    let db = FormulaDb::parse(&bytes).unwrap();
    let formula_id = db.champion_formula_id(0, 0).unwrap();

    let plain = db
        .render_formula_plain(
            formula_id,
            |ctx| match ctx {
                0 => "level".into(),
                1 => "ability_power".into(),
                _ => unreachable!(),
            },
            |_| unreachable!(),
        )
        .unwrap();

    assert_eq!(
        plain,
        "(match level as u8 {\n    ..=1 => 5,\n    2 => 10,\n    3.. => 145,\n}) + 1.5 * ability_power"
    );
}

#[test]
fn html_colors_only_semantic_classes() {
    let source = "match ctx.level {\n    1 => 10,\n    2.. => 20,\n}";
    let mut builder = FormulaDbBuilder::new(1, 0, 0, ctx_id);
    builder
        .push_champion(0, &[FormulaSource { local: 0, source }], &HashMap::new())
        .unwrap();

    let bytes = builder.finish().unwrap();
    let db = FormulaDb::parse(&bytes).unwrap();
    let html = db
        .render_formula_html(0, |_| "level".into(), |_| unreachable!())
        .unwrap();

    assert!(html.contains(&format!(
        "<span class=\"C{}\">match</span>",
        Class::Control as u8
    )));
    assert!(html.contains(&format!(
        "<span class=\"C{}\">level</span>",
        Class::Variable as u8
    )));
    assert!(!html.contains("ctx."));

    // These tokens are escaped/plain text, not standalone styled spans.
    assert!(!html.contains(">=&gt;</span>"));
    assert!(!html.contains(">..</span>"));
    assert!(!html.contains(">,</span>"));
}
