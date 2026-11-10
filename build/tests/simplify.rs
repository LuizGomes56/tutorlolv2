use syn::Expr;
use tutorlolv2_build_dep::scripts::simplify::*;

#[test]
fn literals_keep_only_the_precision_f32_can_represent() {
    assert_eq!(simplify("0.333333333333 * ctx.ap"), "0.33333334 * ctx.ap");
    assert_eq!(simplify("0.016666666667 * ctx.ap"), "0.016666668 * ctx.ap");
    assert_eq!(simplify("1.4285714286 * ctx.ap"), "1.4285715 * ctx.ap");
    assert_eq!(simplify_exact("0.1 + 0.2"), "0.30000000000000004");
    assert_eq!(simplify("0.1 + 0.2"), "0.3");
    assert_eq!(simplify("2.0000000000001 * ctx.ap"), "2 * ctx.ap");
}

#[test]
fn constant_expressions_are_folded() {
    assert_eq!(simplify("1 / 3"), "0.33333334");
    assert_eq!(simplify("1 - 1"), "0");
    assert_eq!(simplify_exact("2 * 3 + 0.5"), "6.5");
    assert_eq!(simplify("(2 * ctx.ap) / (ctx.ap)"), "2");
}

#[test]
fn original_is_kept_unless_the_simplification_is_cheaper() {
    // symb_anafis rewrites this as `(7 + 0.33*ctx.ap)/21`, which adds a runtime division.
    assert_eq!(
        simplify("0.11 / 7 * ctx.ap + 1 / 3"),
        "0.11 / 7 * ctx.ap + 1 / 3"
    );
    assert_eq!(simplify("ctx.ap * 0.5 * 2"), "ctx.ap");
    assert_eq!(simplify("ctx.ap + zero"), "ctx.ap");
    assert_eq!(simplify("10 + 5 * ctx.q_level"), "10 + 5 * ctx.q_level");
}

#[test]
fn redundant_parentheses_are_removed() {
    assert_eq!(
        simplify("((0.45) * ctx.max_health)"),
        "0.45 * ctx.max_health"
    );
    assert_eq!(
        simplify("(((0.1) * ctx.ap) + ((0.03) * ctx.hp)) + ((0.75) * ctx.ad)"),
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
fn powers_become_multiplications() {
    assert_eq!(expand_powers("ctx.a^2").unwrap(), "(ctx.a * ctx.a)");
    assert_eq!(
        expand_powers("c*(a*(1 + b))^2").unwrap(),
        "c*((a*(1 + b)) * (a*(1 + b)))"
    );
    assert_eq!(expand_powers("a/b^-1").unwrap(), "a/(1 / (b))");
    assert_eq!(expand_powers("x^0.5"), None);

    let squared = simplify("(ctx.ap + 1) * (ctx.ap + 1)");
    assert!(!squared.contains('^'), "{squared}");
}

#[test]
fn match_arms_are_simplified_and_parenthesized() {
    assert_eq!(
        simplify("match ctx.level { ..=1 => 0.1 + 0.2, 2..5 => 1 / 3, _ => 20 }"),
        "match ctx.level { ..=1 => 0.3, 2..5 => 0.33333334, _ => 20 }"
    );

    let product = simplify("ctx.ap * match ctx.level { 1 => 2, _ => 3 }");
    assert!(
        product.contains("(match ctx.level { 1 => 2, _ => 3 })"),
        "{product}"
    );
    assert!(syn::parse_str::<Expr>(&product).is_ok(), "{product}");

    let nested =
        simplify("match ctx.level { 1 => match ctx.q_level { 1 => 1 / 4, _ => 2 }, _ => 3 }");
    assert_eq!(
        nested,
        "match ctx.level { 1 => match ctx.q_level { 1 => 0.25, _ => 2 }, _ => 3 }"
    );
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
