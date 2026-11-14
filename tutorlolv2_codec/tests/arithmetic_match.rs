use std::collections::HashMap;
use tutorlolv2_codec::{FormulaDb, FormulaDbBuilder, FormulaSource, VERSION};

fn ctx(name: &str) -> Option<u8> {
    (name == "level").then_some(0)
}
fn pack(source: &str) -> Result<Vec<u8>, tutorlolv2_codec::Error> {
    let mut builder = FormulaDbBuilder::new(1, 0, 0, ctx, |_| None);
    builder.push_champion(0, &[FormulaSource { local: 0, source }], &HashMap::new())?;
    builder.finish()
}
fn plain(bytes: &[u8]) -> Result<String, tutorlolv2_codec::Error> {
    FormulaDb::new(bytes)?.render_formula_plain(
        0,
        |_| "level".into(),
        |_| unreachable!(),
        |_| unreachable!(),
    )
}
const SEGMENTS: &str = "match ctx.level { ..8 => 35 + 3 * (ctx.level - 1).max(0.0), 8..14 => 62 + 9 * (ctx.level - 8), 14.. => 122 + 15 * (ctx.level - 14) }";

#[test]
fn arithmetic_match_round_trips_structure_names_casts_and_patterns() {
    for source in [SEGMENTS.to_owned(), SEGMENTS.replace("ctx.level {", "ctx.level as u8 {"),
        "match ctx.level { 1 => 1 + 2, ..=3 => 4, 4..=7 => -2, 8.. => (ctx.level - 1) }".to_owned(),
        "match ctx.level {\n    ..8 => {\n        35 + 3 * (ctx.level - 1).max(0.0)\n    }\n    8.. => 62 + 9 * (ctx.level - 8),\n}".to_owned()] {
        let bytes = pack(&source).unwrap();
        assert_eq!(plain(&bytes).unwrap(), source.replace("ctx.", ""));
        let html = FormulaDb::new(&bytes).unwrap().render_formula_html(0, |_| "renamed_level".into(), |_| unreachable!(), |_| unreachable!()).unwrap();
        assert!(html.contains("renamed_level"));
        assert!(html.contains("span"));
    }
}

#[test]
fn grouped_segments_beat_eighteen_literals_including_pools_and_header() {
    let values = [
        35, 38, 41, 44, 47, 50, 53, 62, 71, 80, 89, 98, 107, 122, 137, 152, 167, 182,
    ];
    let arms = values
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{} => {v}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");
    let original = pack(&format!("match ctx.level {{ {arms} }}")).unwrap();
    let grouped = pack(SEGMENTS).unwrap();
    assert!(
        grouped.len() < original.len(),
        "{} >= {}",
        grouped.len(),
        original.len()
    );
    // Literal matches still use the shared match definition pool.
    assert_eq!(u16::from_le_bytes([original[10], original[11]]), 1);
    assert_eq!(u16::from_le_bytes([grouped[10], grouped[11]]), 0);
    assert_eq!(grouped[4], VERSION);
}

#[test]
fn malformed_arithmetic_match_is_rejected() {
    for source in [
        "match ctx.level {}",
        "match ctx.level { ..8 => arbitrary(ctx.level), 8.. => 2 }",
        "match ctx.level { ..8 => 1 +, 8.. => 2 }",
        "match ctx.level { ..8 => 1 + 2 8.. => 3 }",
        "match ctx.level { ..8 => { 1 + 2; }, 8.. => 3 }",
        "match ctx.level { ..8 => (ctx.level - 1).max(2), 8.. => 3 }",
    ] {
        assert!(pack(source).is_err(), "{source}");
    }
}

#[test]
fn corrupt_expression_match_metadata_and_arm_bytecode_are_rejected() {
    let bytes = pack("match ctx.level { ..8 => 1 + 2, 8.. => 3 }").unwrap();
    // Header, one champion base entry and two formula offsets precede this
    // formula; this formula has no constants or shared match definitions.
    let start = 24 + 4 + 2 + 8;
    assert_eq!(bytes[start], 0x1b);
    for (offset, value) in [(2, 2), (3, 0), (4, 255), (6, 255), (7, 255)] {
        let mut corrupt = bytes.clone();
        corrupt[start + offset] = value;
        assert!(plain(&corrupt).is_err(), "offset {offset}");
    }
    for end in start..bytes.len() {
        assert!(plain(&bytes[..end]).is_err());
    }
}

#[test]
fn nested_block_expressions_can_omit_arm_commas() {
    let source = "match ctx.level {\n    ..8 => match ctx.level { ..4 => 1 + 2, 4.. => 4 }\n    8.. => 5 + 6,\n}";
    assert_eq!(
        plain(&pack(source).unwrap()).unwrap(),
        source.replace("ctx.", "")
    );
}
