use tutorlolv2_wiki::wikitext::{Node, Nodes, SourceVariant};

#[test]
fn lethal_tempo_keeps_level_formula_as_a_progression_tree() {
    let source = Nodes::parse("{{rd|20 to 40 for 20|10 to 20 for 20|pp=true|key=%}}");
    for (ranged, expression) in [(false, "20 to 40 for 20"), (true, "10 to 20 for 20")] {
        let resolved = source
            .for_variant(SourceVariant {
                ranged,
                alternate: false,
            })
            .unwrap();
        let Node::Template(pp) = &resolved.0[0] else {
            panic!("expected pp template")
        };
        assert_eq!(pp.name, "pp");
        assert_eq!(pp.positional(0).unwrap(), &Nodes::parse(expression));
        assert_eq!(pp.named("key").unwrap(), &Nodes::parse("%"));
        assert!(pp.named("pp").is_none());
    }
}

#[test]
fn grasp_selects_fd_percent_and_primary_or_alternate_text() {
    let source = Nodes::parse(
        "{{as|{{rd|{{fd|3.5}}%|{{fd|2.1}}%}} maximum health}} {{ft|primary|alternate}}",
    );
    assert_eq!(
        source.for_variant(SourceVariant::default()).unwrap(),
        Nodes::parse("{{as|{{fd|3.5}}% maximum health}} primary")
    );
    assert_eq!(
        source
            .for_variant(SourceVariant {
                ranged: true,
                alternate: true
            })
            .unwrap(),
        Nodes::parse("{{as|{{fd|2.1}}% maximum health}} alternate")
    );
}

#[test]
fn normalizes_nested_arguments_without_changing_other_templates() {
    let source = Nodes::parse(
        "before {{as|{{ft|{{rd|melee|ranged}}|other}}|kind={{rd|damage|heal}}|keep=value}} after",
    );
    assert_eq!(
        source
            .for_variant(SourceVariant {
                ranged: true,
                alternate: false
            })
            .unwrap(),
        Nodes::parse("before {{as|ranged|kind=heal|keep=value}} after")
    );
}

#[test]
fn only_the_selected_branch_is_normalized() {
    let source = Nodes::parse("{{ft|kept|{{rd}}|label=ignored}}");
    assert_eq!(
        source.for_variant(SourceVariant::default()).unwrap(),
        Nodes::parse("kept")
    );
    assert!(
        source
            .for_variant(SourceVariant {
                ranged: false,
                alternate: true
            })
            .is_err()
    );
    assert!(
        Nodes::parse("{{rd|melee}}")
            .for_variant(SourceVariant {
                ranged: true,
                alternate: false
            })
            .is_err()
    );
}

#[test]
fn non_progression_rd_preserves_selected_typed_nodes() {
    let source = Nodes::parse("{{rd|{{fd|4}}%|{{fd|2}}%|pp=false}}");
    assert_eq!(
        source.for_variant(SourceVariant::default()).unwrap(),
        Nodes::parse("{{fd|4}}%")
    );
}

#[test]
fn progression_rd_preserves_explicit_level_domain() {
    let source = Nodes::parse("{{rd|150;155 to 210|100;105 to 160|pp=true|levels=1;9 to 20}}");
    let normalized = source.for_variant(SourceVariant::default()).unwrap();
    assert_eq!(normalized, Nodes::parse("{{pp|150;155 to 210|1;9 to 20}}"));
    let progression =
        tutorlolv2_wiki::wikitext::PassiveProgression::new(normalized.templates()[0]).unwrap();
    assert_eq!(&progression.keys[..2], &[1.0, 9.0]);
}
