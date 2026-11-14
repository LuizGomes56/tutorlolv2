use syn::Expr;
use tutorlolv2_build_dep::scripts::simplify::*;

#[test]
fn literals_keep_only_the_precision_f32_can_represent() {
    assert_eq!(
        format_formula("0.333333333333 * ctx.ap"),
        "0.33333334 * ctx.ap"
    );
    assert_eq!(
        format_formula("0.016666666667 * ctx.ap"),
        "0.016666668 * ctx.ap"
    );
    assert_eq!(
        format_formula("1.4285714286 * ctx.ap"),
        "1.4285715 * ctx.ap"
    );
    assert_eq!(format_formula_exact("0.1 + 0.2"), "0.30000000000000004");
    assert_eq!(format_formula("0.1 + 0.2"), "0.3");
    assert_eq!(format_formula("2.0000000000001 * ctx.ap"), "2 * ctx.ap");
}

#[test]
fn constant_expressions_are_folded() {
    assert_eq!(format_formula("1 / 3"), "0.33333334");
    assert_eq!(format_formula("1 - 1"), "0");
    assert_eq!(format_formula_exact("2 * 3 + 0.5"), "6.5");
    // Zero formulas stay detectable
    assert_eq!(format_formula("0.0 * 2.0"), "0");
}

#[test]
fn formulas_keep_their_shape() {
    // Not reordered, combined or factored, as the symbolic step did
    assert_eq!(
        format_formula("((5.0) + (40.0 * (ctx.q_level - 1.0))) + ((1.25) * ctx.attack_damage)"),
        "5 + 40 * (ctx.q_level - 1) + 1.25 * ctx.attack_damage"
    );
    assert_eq!(format_formula("ctx.ap * 0.5 * 2"), "ctx.ap * 0.5 * 2");
    assert_eq!(format_formula("ctx.ap + zero"), "ctx.ap + 0");
    assert_eq!(
        format_formula("10 + 5 * ctx.q_level"),
        "10 + 5 * ctx.q_level"
    );
}

#[test]
fn redundant_parentheses_are_removed() {
    assert_eq!(
        format_formula("((0.45) * ctx.max_health)"),
        "0.45 * ctx.max_health"
    );
    assert_eq!(
        format_formula("(((0.1) * ctx.ap) + ((0.03) * ctx.hp)) + ((0.75) * ctx.ad)"),
        "0.1 * ctx.ap + 0.03 * ctx.hp + 0.75 * ctx.ad"
    );
    assert_eq!(normalize("a - (b - c)").unwrap(), "a - (b - c)");
    assert_eq!(
        normalize("ctx.ap * (0.5 * 2)").unwrap(),
        "ctx.ap * (0.5 * 2)"
    );
    assert_eq!(normalize("(a * b) / c").unwrap(), "a * b / c");
    assert_eq!(normalize("-(a + b) * 2").unwrap(), "-(a + b) * 2");
}

#[test]
fn match_arms_are_formatted() {
    // Arms are left as written, apart from the precision of their literals
    assert_eq!(
        format_formula("match ctx.level { ..=1 => 0.1 + 0.2, 2..5 => 1 / 3, _ => 20 }"),
        "match ctx.level { ..=1 => 0.1 + 0.2, 2..5 => 1 / 3, _ => 20 }"
    );
    assert_eq!(
        format_formula("match ctx.level { 1 => 0.333333333333, _ => 2 }"),
        "match ctx.level { 1 => 0.33333334, _ => 2 }"
    );

    let product = format_formula("ctx.ap * (match ctx.level { 1 => 2, _ => 3 })");
    assert!(syn::parse_str::<Expr>(&product).is_ok(), "{product}");
}

#[test]
fn approximate_constants_follow_clippy() {
    assert_eq!(approx_constant("3.14"), Some("PI"));
    assert_eq!(approx_constant("3.1416"), Some("PI"));
    assert_eq!(approx_constant("3.1415927"), Some("PI"));
    assert_eq!(approx_constant("6.28"), Some("TAU"));
    assert_eq!(approx_constant("3.1"), None);
    assert_eq!(approx_constant("0.785"), None);
    assert_eq!(approx_constant("31.4"), None);
}

#[test]
fn matches_inside_expressions_are_parenthesized() {
    // A `match` starting a sum is a statement in Rust, so it needs parentheses
    assert_eq!(
        format_formula("match ctx.level { ..7 => 250, 7.. => 290 } + 0.8 * ctx.bonus_ad"),
        "(match ctx.level { ..7 => 250, 7.. => 290 }) + 0.8 * ctx.bonus_ad"
    );
    assert_eq!(
        format_formula("1.5 * match ctx.level { 1 => 2, _ => 3 }"),
        "1.5 * (match ctx.level { 1 => 2, _ => 3 })"
    );
    // Alone, or already in parentheses, it is kept as it is
    assert_eq!(
        format_formula("match ctx.level { 1 => 2, _ => 3 }"),
        "match ctx.level { 1 => 2, _ => 3 }"
    );
    assert_eq!(
        format_formula("(match ctx.level { 1 => 2, _ => 3 }) * 2"),
        "(match ctx.level { 1 => 2, _ => 3 }) * 2"
    );

    let formula = format_formula("match ctx.level { ..7 => 250, 7.. => 290 } + 0.8 * ctx.bonus_ad");
    // As the body of the generated functions
    let body = format!("{{ {formula} }}");
    assert!(syn::parse_str::<syn::Block>(&body).is_ok(), "{body}");
    let unwrapped = "{ match ctx.level { ..7 => 250, 7.. => 290 } + 0.8 * ctx.bonus_ad }";
    assert!(syn::parse_str::<syn::Block>(unwrapped).is_err());
}

#[test]
fn parentheses_around_the_whole_formula_are_removed() {
    assert_eq!(
        format_formula("((match ctx.level { 1 => 2, _ => 3 }) + (0.04 * ctx.max_health))"),
        "(match ctx.level { 1 => 2, _ => 3 }) + 0.04 * ctx.max_health"
    );
    assert_eq!(
        format_formula("((match ctx.level { 1 => 2, _ => 3 }))"),
        "match ctx.level { 1 => 2, _ => 3 }"
    );
    // Parentheses that don't wrap the whole formula are kept
    assert_eq!(
        format_formula("(match ctx.level { 1 => 2, _ => 3 }) * (1 + ctx.ap)"),
        "(match ctx.level { 1 => 2, _ => 3 }) * (1 + ctx.ap)"
    );
}

#[test]
fn formulas_with_matches_are_normalized_around_them() {
    // Ahri's doc source, which kept every parenthesis of the generator
    assert_eq!(
        format_formula(
            "((match ctx.level { ..6 => 15, 6.. => 40 }) + ((0.6) * ctx.bonus_ad)) + ((0.65) * ctx.ability_power)"
        ),
        "(match ctx.level { ..6 => 15, 6.. => 40 }) + 0.6 * ctx.bonus_ad + 0.65 * ctx.ability_power"
    );
    // Two matches, and one inside a product
    assert_eq!(
        format_formula(
            "((0.5) * (match ctx.level { 1 => 2, _ => 3 })) + (match ctx.q_level { 1 => 4, _ => 5 })"
        ),
        "0.5 * (match ctx.level { 1 => 2, _ => 3 }) + (match ctx.q_level { 1 => 4, _ => 5 })"
    );
}

#[test]
fn a_match_after_a_statement_is_not_parenthesized() {
    // A map arm whose value is a table by rank, as Ezreal's Q on Arena
    let arm = "match ctx.map { tutorlolv2_types::MapMode::Arena => { core::hint::cold_path(); match ctx.q_level { ..=1 => 30.0, 2.. => 60.0 } } _ => 1.0 }";
    let formatted = format_formula(arm).replace(char::is_whitespace, "");
    assert!(
        formatted.contains("cold_path();matchctx.q_level"),
        "{formatted}"
    );
}
