use crate::{
    generators::parser::{Parser, infer_damage_type, items::Item, runes::RuneParser},
    scripts::utils::{ItemOrRuneExt, cast_f32},
};

#[test]
fn ranged_dispatch_uses_the_ranged_formula() {
    let mut item = Item::try_from(tutorlolv2_wiki::items::item_parser::WikiItem {
        name: "Blade of the Ruined King".into(),
        item_id: "BladeOfTheRuinedKing".into(),
        ..Default::default()
    })
    .unwrap();
    item.melee.min_dmg = "100".into();
    item.ranged.min_dmg = "50".into();
    let output = item.eval();
    let ranged = output.split("AttackType :: Ranged =>").nth(1).unwrap();
    assert!(
        ranged.contains("::blade_of_the_ruined_king_ranged("),
        "{ranged}"
    );
    assert!(!ranged.contains("::blade_of_the_ruined_king_melee("));
}

#[test]
fn damage_inference_recognizes_magic() {
    let mut output = String::new();
    infer_damage_type(&mut output, "Deals magic damage.");
    assert_eq!(output, ".damage_type(Magic)");
}

#[test]
fn numeric_rewriting_preserves_ranges_and_match_patterns() {
    for source in [
        "match ctx.level { 1..=5 => 10, _ => 20 }",
        "match ctx.level { 1 => 10, _ => 20 }",
        "1.0..=18.0",
    ] {
        let output = cast_f32(source).unwrap();
        assert!(syn::parse_str::<syn::Expr>(&output).is_ok(), "{output}");
    }
}

#[test]
fn exporting_an_unknown_entity_returns_an_error() {
    let parser = RuneParser {
        data: Default::default(),
    };
    assert!(parser.run("UnknownRune").is_err());
}

#[test]
fn generator_documentation_preserves_braces_in_strings_and_comments() {
    let source = r##"impl Generator for Test { fn generate(&mut self) { let text = "}"; /* } */ do_something(); } }"##;
    let parsed =
        crate::scripts::batch::generator_impl(&format!("use super::*;\n{source}")).unwrap();
    assert_eq!(parsed, source);
}

struct TestDir(std::path::PathBuf);
impl TestDir {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("tutorlolv2-build-test-{}-{id}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn invalid_numeric_expressions_return_errors() {
    assert!(cast_f32("ctx.ap +").is_err());
}

#[test]
fn numeric_rewriting_leaves_pattern_literals_integral() {
    let output = cast_f32("match ctx.level { 1..=5 => 10, _ => 20 }").unwrap();
    let syn::Expr::Match(expr) = syn::parse_str::<syn::Expr>(&output).unwrap() else {
        panic!()
    };
    let syn::Pat::Range(pattern) = &expr.arms[0].pat else {
        panic!()
    };
    for bound in [&pattern.start, &pattern.end] {
        let syn::Expr::Lit(bound) = bound.as_ref().unwrap().as_ref() else {
            panic!()
        };
        assert!(matches!(bound.lit, syn::Lit::Int(_)));
    }
    let syn::Expr::Lit(value) = expr.arms[0].body.as_ref() else {
        panic!()
    };
    let suffix = match &value.lit {
        syn::Lit::Float(value) => value.suffix(),
        syn::Lit::Int(value) => value.suffix(),
        _ => panic!(),
    };
    assert_eq!(suffix, "f32");
}

#[test]
fn optional_generator_files_distinguish_absence_from_invalid_input() {
    use crate::{generators::utils::Tag, scripts::batch::read_generator};
    let dir = TestDir::new();
    assert!(
        read_generator(&dir.0, Tag::Items, "Example")
            .unwrap()
            .is_none()
    );
    let path = dir.0.join("build/src/generators/impls/items/example.rs");
    tutorlolv2_wiki::write(&path, "impl Generator for Example {").unwrap();
    assert!(read_generator(&dir.0, Tag::Items, "Example").is_err());
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    assert!(read_generator(&dir.0, Tag::Items, "Example").is_err());
}

#[test]
fn formatter_failures_are_errors() {
    let dir = TestDir::new();
    let path = dir.0.join("invalid.rs");
    std::fs::write(&path, "fn broken( {").unwrap();
    let result = crate::context::run_formatter(std::process::Command::new("rustfmt").arg(path));
    assert!(result.unwrap_err().to_string().contains("rustfmt failed"));
}

#[test]
fn documentation_uses_explicit_indices_and_ignores_stale_files() {
    use crate::{
        BuildContext,
        generators::utils::Tag,
        scripts::batch::{EntityDocs, FormulaDoc},
    };
    use tutorlolv2_codec::{DamageSlot, EntityKind, FormulaDb};
    let dir = TestDir::new();
    let context = BuildContext::new(&dir.0, "out", true).unwrap();
    tutorlolv2_wiki::write(context.out.join("items/Stale.w48"), "not valid Rust").unwrap();
    let mut docs = vec![EntityDocs {
        tag: Tag::Items,
        index: 1,
        variant: "Active".into(),
        generator: None,
        formulas: vec![FormulaDoc {
            local: 2,
            function: "active_f2".into(),
            source: "10 + 20".into(),
            name: "Name }".into(),
            comment: "words separated by spaces /* } */".into(),
        }],
    }];

    context.build_docs(&mut docs, [0, 2, 0]).unwrap();
    let bytes = std::fs::read(context.out.join("formulas.bin")).unwrap();
    let db = FormulaDb::new(&bytes).unwrap();
    assert!(
        db.item_or_rune_formula_id(EntityKind::Item, 0, DamageSlot::RangedMin)
            .is_none()
    );
    let id = db
        .item_or_rune_formula_id(EntityKind::Item, 1, DamageSlot::RangedMin)
        .unwrap();
    let rendered = db
        .render_formula_plain(id, |_| unreachable!(), |_| unreachable!())
        .unwrap();
    assert_eq!(rendered, "10 + 20");
    let recovered: Vec<EntityDocs> =
        serde_json::from_slice(&std::fs::read(context.out.join("documentation.json")).unwrap())
            .unwrap();
    assert_eq!(
        recovered[0].formulas[0].comment,
        "words separated by spaces /* } */"
    );
    assert_eq!(recovered[0].formulas[0].name, "Name }");
}

#[test]
fn formatting_recovers_the_complete_nested_match_expression() {
    use crate::{
        BuildContext,
        generators::utils::Tag,
        scripts::batch::{EntityDocs, FormulaDoc},
    };
    let dir = TestDir::new();
    let context = BuildContext::new(&dir.0, "out", true).unwrap();
    let mut docs = vec![EntityDocs {
        tag: Tag::Champions,
        index: 0,
        variant: "Example".into(),
        generator: None,
        formulas: vec![FormulaDoc {
            local: 0,
            function: "example_p".into(),
            source: "(match ctx.level { 1 => 10, _ => 20 }) * 2".into(),
            name: String::new(),
            comment: String::new(),
        }],
    }];
    context.format_formulas(&mut docs).unwrap();
    let syn::Expr::Binary(expr) = syn::parse_str::<syn::Expr>(&docs[0].formulas[0].source).unwrap()
    else {
        panic!()
    };
    assert!(matches!(expr.op, syn::BinOp::Mul(_)));
    let syn::Expr::Lit(value) = expr.right.as_ref() else {
        panic!()
    };
    let syn::Lit::Int(value) = &value.lit else {
        panic!()
    };
    assert_eq!(value.base10_digits(), "2");
}

fn recommendation_rows(source: &str) -> Vec<Vec<Vec<String>>> {
    let file = syn::parse_file(source).unwrap();
    let syn::Item::Static(table) = &file.items[0] else {
        panic!()
    };
    let syn::Expr::Array(rows) = table.expr.as_ref() else {
        panic!()
    };
    rows.elems
        .iter()
        .map(|row| {
            let syn::Expr::Array(roles) = row else {
                panic!()
            };
            roles
                .elems
                .iter()
                .map(|role| {
                    let syn::Expr::Reference(role) = role else {
                        panic!()
                    };
                    let syn::Expr::Array(ids) = role.expr.as_ref() else {
                        panic!()
                    };
                    ids.elems
                        .iter()
                        .map(|id| {
                            let syn::Expr::Path(id) = id else { panic!() };
                            id.path.segments.last().unwrap().ident.to_string()
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[test]
fn recommendations_follow_champion_and_position_order_with_empty_defaults() {
    use crate::scripts::finish::{Recommendations, recommendations_code};
    let data: Recommendations =
        serde_json::from_str(r#"{"Zed":{"adc":[["RangedItem"],[]],"top":[["MeleeItem"],[]]}}"#)
            .unwrap();
    let code = recommendations_code(["Ahri", "Zed"].into_iter(), &data).unwrap();
    let rows = recommendation_rows(&code);
    assert_eq!(rows[0], vec![Vec::<String>::new(); 5]);
    assert_eq!(
        rows[1],
        vec![
            vec!["MeleeItem"],
            vec![],
            vec![],
            vec!["RangedItem"],
            vec![]
        ]
    );
}

#[test]
fn recommendations_reject_malformed_optional_files() {
    use crate::generators::parser::champions::ChampionParser;
    let dir = TestDir::new();
    let parser = ChampionParser {
        data: Default::default(),
    };
    assert!(parser.recommendations(&dir.0).is_ok());
    tutorlolv2_wiki::write(dir.0.join("internal/scraper/data.json"), "{").unwrap();
    assert!(parser.recommendations(&dir.0).is_err());
}

#[test]
fn exports_propagate_write_errors() {
    let dir = TestDir::new();
    std::fs::write(dir.0.join("internal"), "blocking file").unwrap();
    let parser = RuneParser {
        data: Default::default(),
    };
    assert!(parser.run_all_in(&dir.0).is_err());
}

#[test]
fn aggregate_exports_only_contain_current_entities() {
    use tutorlolv2_wiki::runes::{RuneKeystone, RuneSlot, WikiRune};
    let dir = TestDir::new();
    tutorlolv2_wiki::write(dir.0.join("internal/runes/Stale.json"), "{}").unwrap();
    let rune = WikiRune {
        name: "Example".into(),
        rune_id: "Example".into(),
        riot_id: 42,
        effects: Default::default(),
        descriptions: vec![],
        custom: true,
        path: RuneKeystone::Inspiration,
        slot: RuneSlot::Keystone,
    };
    let parser = RuneParser {
        data: [("Example".into(), rune)].into(),
    };
    parser.run_all_in(&dir.0).unwrap();
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.0.join("internal/runes.json")).unwrap()).unwrap();
    assert_eq!(result.as_object().unwrap().len(), 1);
    assert_eq!(result["Example"]["riot_id"], 42);
}

#[test]
fn prepared_formulas_keep_functions_that_simplify_to_zero() {
    let mut item = Item::try_from(tutorlolv2_wiki::items::item_parser::WikiItem {
        item_id: "Cancelled".into(),
        ..Default::default()
    })
    .unwrap();
    item.melee.min_dmg = "1 - 1".into();
    let plan = item.prepare_formulas();
    let source = item.closures(&plan).unwrap();
    let file = syn::parse_file(&source).unwrap();
    assert!(file.items.iter().any(|item| matches!(item,
        syn::Item::Fn(function) if function.sig.ident == "cancelled_melee")));
    assert_eq!(item.formula_docs(&plan).len(), 1);
}

#[test]
fn formatting_accepts_formulas_ending_in_line_comments() {
    use crate::{
        BuildContext,
        generators::utils::Tag,
        scripts::batch::{EntityDocs, FormulaDoc},
    };
    let dir = TestDir::new();
    let context = BuildContext::new(&dir.0, "out", true).unwrap();
    let mut docs = vec![EntityDocs {
        tag: Tag::Items,
        index: 0,
        variant: "Example".into(),
        generator: None,
        formulas: vec![FormulaDoc {
            local: 0,
            function: "example_f0".into(),
            source: "10 // trailing comment".into(),
            name: String::new(),
            comment: String::new(),
        }],
    }];
    context.format_formulas(&mut docs).unwrap();
    assert!(syn::parse_str::<syn::Expr>(&docs[0].formulas[0].source).is_ok());
}

#[test]
fn missing_recommendations_watch_the_existing_parent_then_the_file() {
    let dir = TestDir::new();
    std::fs::create_dir(dir.0.join("internal")).unwrap();
    let context = crate::BuildContext::new(&dir.0, "out", false).unwrap();
    assert!(context.inputs().contains(&dir.0.join("internal")));
    tutorlolv2_wiki::write(dir.0.join("internal/scraper/data.json"), "{}").unwrap();
    assert!(
        context
            .inputs()
            .contains(&dir.0.join("internal/scraper/data.json"))
    );
}

fn wiki_item(
    id: u32,
    key: &str,
    name: &str,
    recipe: &[&str],
) -> tutorlolv2_wiki::items::item_parser::WikiItem {
    tutorlolv2_wiki::items::item_parser::WikiItem {
        id,
        item_id: key.into(),
        name: name.into(),
        recipe: recipe.iter().map(|s| (*s).into()).collect(),
        ..Default::default()
    }
}

fn riot_item(name: &str, from: &[&str], into: &[&str]) -> serde_json::Value {
    serde_json::json!({"name": name, "description": "", "gold": {"total": 1000, "purchasable": true},
        "maps": {"11": true}, "from": from, "into": into})
}

fn recipe_fixture(
    wiki: Vec<tutorlolv2_wiki::items::item_parser::WikiItem>,
    riot: serde_json::Value,
) -> (TestDir, crate::generators::parser::items::ItemParser) {
    let dir = TestDir::new();
    let wiki = wiki
        .into_iter()
        .map(|item| (item.item_id.clone(), item))
        .collect::<std::collections::BTreeMap<_, _>>();
    tutorlolv2_wiki::write(
        dir.0.join("cache/wiki/items/full.json"),
        serde_json::to_vec(&wiki).unwrap(),
    )
    .unwrap();
    tutorlolv2_wiki::write(
        dir.0.join("cache/riot/items.json"),
        serde_json::to_vec(&serde_json::json!({"data": riot})).unwrap(),
    )
    .unwrap();
    let parser = crate::generators::parser::items::ItemParser::from_root(&dir.0).unwrap();
    (dir, parser)
}

#[test]
fn riot_recipes_take_precedence_and_preserve_repeated_components() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "Component", "Component", &[]),
            wiki_item(2, "Other", "Other", &[]),
            wiki_item(3, "Finished", "Finished", &["Other"]),
        ],
        serde_json::json!({
            "1": riot_item("Component", &[], &["3", "3"]),
            "2": riot_item("Other", &[], &[]),
            "3": riot_item("Finished", &["1", "1"], &[]),
        }),
    );
    assert_eq!(
        parser.run_fn("Finished").unwrap().builds_from,
        ["Component", "Component"]
    );
    assert_eq!(
        parser.run_fn("Component").unwrap().builds_into,
        ["Finished"]
    );
    assert!(parser.run_fn("Other").unwrap().builds_into.is_empty());
}

#[test]
fn wiki_only_recipes_resolve_display_names_and_add_reverse_links() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "ComponentsBlade", "Component's Blade", &[]),
            wiki_item(
                3,
                "Finished",
                "Finished",
                &["Component's Blade", "Component's Blade"],
            ),
        ],
        serde_json::json!({"1": riot_item("Component's Blade", &[], &[])}),
    );
    assert_eq!(
        parser.run_fn("Finished").unwrap().builds_from,
        ["ComponentsBlade", "ComponentsBlade"]
    );
    assert_eq!(
        parser.run_fn("ComponentsBlade").unwrap().builds_into,
        ["Finished"]
    );
}

#[test]
fn numeric_recipes_follow_collapsed_mode_ids() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "Component", "Component", &[]),
            wiki_item(3, "Finished", "Finished", &[]),
        ],
        serde_json::json!({
            "1": riot_item("Component", &[], &["3"]),
            "101": riot_item("Component", &[], &["3"]),
            "3": riot_item("Finished", &["101"], &[]),
        }),
    );
    assert_eq!(
        parser.run_fn("Finished").unwrap().builds_from,
        ["Component"]
    );
    assert!(!parser.map().contains_key("Component101"));
}

#[test]
fn numeric_recipes_keep_distinct_duplicate_name_variants() {
    let (_dir, parser) = recipe_fixture(
        vec![wiki_item(3, "Finished", "Finished", &[])],
        serde_json::json!({
            "10": riot_item("Token", &[], &["3"]),
            "20": riot_item("Token", &[], &["3"]),
            "3": riot_item("Finished", &["20"], &[]),
        }),
    );
    assert_eq!(parser.run_fn("Finished").unwrap().builds_from, ["Token20"]);
    assert_eq!(parser.run_fn("Token10").unwrap().builds_into, ["Finished"]);
}

#[test]
fn unavailable_and_collapsed_self_references_are_not_emitted() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "Component", "Component", &[]),
            wiki_item(2, "WikiOnly", "Wiki Only", &["Missing Item"]),
        ],
        serde_json::json!({"1": riot_item("Component", &["999"], &["1", "999"])}),
    );
    let item = parser.run_fn("Component").unwrap();
    assert!(item.builds_from.is_empty());
    assert!(item.builds_into.is_empty());
    assert!(parser.run_fn("WikiOnly").unwrap().builds_from.is_empty());
}

#[test]
fn empty_riot_recipes_do_not_fall_back_to_wiki_recipes() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "Component", "Component", &[]),
            wiki_item(3, "Finished", "Finished", &["Component"]),
        ],
        serde_json::json!({
            "1": riot_item("Component", &[], &[]),
            "3": riot_item("Finished", &[], &[]),
        }),
    );
    assert!(parser.run_fn("Finished").unwrap().builds_from.is_empty());
    assert!(parser.run_fn("Component").unwrap().builds_into.is_empty());
}

#[test]
fn wiki_only_components_receive_reverse_links_from_riot_recipes() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "Component", "Component", &[]),
            wiki_item(3, "Finished", "Finished", &[]),
        ],
        serde_json::json!({"3": riot_item("Finished", &["1"], &[])}),
    );
    assert_eq!(
        parser.run_fn("Component").unwrap().builds_into,
        ["Finished"]
    );
}

#[test]
fn recipes_use_the_riot_record_selected_by_name_when_wiki_ids_are_stale() {
    let (_dir, parser) = recipe_fixture(
        vec![
            wiki_item(1, "Component", "Component", &[]),
            wiki_item(2, "Other", "Other", &[]),
            wiki_item(300, "Finished", "Finished", &["Other"]),
        ],
        serde_json::json!({
            "1": riot_item("Component", &[], &["3"]),
            "2": riot_item("Other", &[], &[]),
            "3": riot_item("Finished", &["1"], &[]),
        }),
    );
    assert_eq!(
        parser.run_fn("Finished").unwrap().builds_from,
        ["Component"]
    );
    assert_eq!(
        parser.run_fn("Component").unwrap().builds_into,
        ["Finished"]
    );
}
