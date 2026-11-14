use tutorlolv2_build_dep::scripts::parentheses::remove_redundant_parentheses as clean;

#[test]
fn removes_delimiters_without_folding_or_reordering() {
    for (input, expected) in [
        (
            "((49) + (49 * (ctx.e_level - 1))) + (0.7 * ctx.attack_damage) + (0.77 * ctx.ability_power)",
            "49 + 49 * (ctx.e_level - 1) + 0.7 * ctx.attack_damage + 0.77 * ctx.ability_power",
        ),
        ("((2) * (3)) + (4 - 1)", "2 * 3 + (4 - 1)"),
        ("((a * b) / c)", "a * b / c"),
        ("((a.b) + (a.method((2))))", "a.b + a.method(2)"),
        ("-(a * b)", "-(a * b)"),
        ("((a + b) as f32)", "(a + b) as f32"),
    ] {
        assert_eq!(clean(input), expected, "{input}");
    }
}

#[test]
fn preserves_evaluation_tree_and_required_groups() {
    for input in [
        "a + (b + c)",
        "a + (b - c)",
        "a - (b + c)",
        "a - (b - c)",
        "a * (b * c)",
        "a * (b / c)",
        "a / (b * c)",
        "a / (b / c)",
        "(a + b) * c",
        "a * (b + c)",
        "(a + b).max(c)",
        "(match ctx.level { 1 => 2, _ => 3 }) + a",
        "(a < b) == c",
    ] {
        assert_eq!(clean(input), input, "{input}");
    }
    assert_eq!(clean("a + (((b + c)))"), "a + (b + c)");
}

#[test]
fn descends_into_map_arms_and_numeric_blocks() {
    let input = "match ctx.map { #[cold] Aram => ((2) + (3 * (a - 1))), _ => a + (b + c) }";
    assert_eq!(
        clean(input),
        "match ctx.map { #[cold] Aram => 2 + 3 * (a - 1), _ => a + (b + c) }"
    );
    let input = "({ let axis: f32 = (ctx.stacks); if (axis < (10.5)) { ((2) + (3 * (axis - (0)).max((0.0)))) } else { (5) } }) * (0.5)";
    let output = clean(input);
    assert_eq!(
        output,
        "({ let axis: f32 = ctx.stacks; if axis < 10.5 { 2 + 3 * (axis - 0).max(0.0) } else { 5 } }) * 0.5"
    );
    syn::parse_str::<syn::Expr>(&output).unwrap();
}

#[test]
fn preserves_comments_literal_spelling_and_unknown_syntax() {
    let input = "((0.7000000) /* (keep café) */ * (ctx.ap)) // (tail)";
    assert_eq!(
        clean(input),
        "0.7000000 /* (keep café) */ * ctx.ap // (tail)"
    );
    assert_eq!(clean("(x?)"), "(x?)");
    assert_eq!(clean("a +"), "a +");
}

#[test]
fn removed_delimiters_do_not_merge_tokens() {
    for (input, expected) in [
        (
            "if((x < (y))) { (1) } else { (2) }",
            "if x < y { 1 } else { 2 }",
        ),
        ("(a)as f32", "a as f32"),
        ("a/(*b)", "a/ *b"),
        ("a/(/* keep */ b)", "a/ /* keep */ b"),
        ("(1.).max((0.))", "1. .max(0.)"),
    ] {
        let output = clean(input);
        assert_eq!(output, expected);
        syn::parse_str::<syn::Expr>(&output).unwrap();
    }
}

#[test]
fn cast_comparisons_keep_their_rust_syntax() {
    for input in ["(a as f32) < b", "(a as u32) << b", "(a + b as u32) << c"] {
        let output = clean(input);
        syn::parse_str::<syn::Expr>(&output).unwrap();
        assert_eq!(output, input);
    }
}

#[test]
fn floating_point_evaluation_order_is_identical() {
    use tutorlolv2_types::CtxVar::*;
    use tutorlolv2_wiki::wikitext::{LegacyFormula, Sample};
    for input in [
        "((ctx.ability_power) + (ctx.attack_damage + ctx.max_health))",
        "((ctx.ability_power + ctx.attack_damage)) + ctx.max_health",
        "(ctx.ability_power) / (ctx.attack_damage * ctx.max_health)",
        "((ctx.ability_power / ctx.attack_damage)) * ctx.max_health",
        "ctx.ability_power - (ctx.attack_damage - ctx.max_health)",
    ] {
        let before = LegacyFormula::parse(input).unwrap();
        let after = LegacyFormula::parse(&clean(input)).unwrap();
        for [a, b, c] in [
            [1e20, -1e20, 3.0],
            [1e-20, 1e20, 1e-20],
            [3.0, 7.0, 11.0],
            [-0.0, 0.0, 1.0],
        ] {
            let sample = Sample::new([(AbilityPower, a), (AttackDamage, b), (MaxHealth, c)]);
            assert_eq!(
                before.eval(&sample).to_bits(),
                after.eval(&sample).to_bits(),
                "{input}"
            );
        }
    }
}
