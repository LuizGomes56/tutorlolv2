use wiki::spells::SpellData;

fn module(effect: &str) -> String {
    format!("return {{ Ignite = {{ effect = {effect:?} }} }}")
}

#[test]
fn ignite_reads_the_damage_progression_instead_of_other_numbers() {
    let src = module(
        "For {{pp|3 to 5}} seconds, deals {{as|{{pp|70; then +20*x for 4;then +25*x|1 to 20 by 1|formula=70 base, then +20 per level until level 5, then +25 per level.}} {{sti|true damage}}}}. Ticks every {{fd|1.056}} seconds.",
    );
    let data = SpellData::from_lua(&src).unwrap();
    assert_eq!(
        data.ignite,
        [
            70, 90, 110, 130, 150, 175, 200, 225, 250, 275, 300, 325, 350, 375, 400, 425, 450, 475,
            500, 525
        ]
    );
    let changed = SpellData::from_lua(&module(
        "Deals {{as|{{pp|100; then +30*x|1 to 20 by 1}} {{sti|true damage}}}}.",
    ))
    .unwrap();
    assert_eq!((changed.ignite[0], changed.ignite[19]), (100, 670));
}

#[test]
fn missing_ambiguous_and_unsupported_damage_is_an_error() {
    for src in [
        "return {}".into(),
        module("No damage here"),
        module(
            "{{as|{{pp|70 to 100}} {{sti|true damage}}}} and {{as|{{pp|80 to 110}} {{sti|true damage}}}}",
        ),
        module("{{as|{{pp|70 to 100|1 to 3|type=seconds elapsed}} {{sti|true damage}}}}"),
        module("{{as|{{pp|70.5 to 100.5}} {{sti|true damage}}}}"),
        module("{{as|{{pp|70;90|2;3}} {{sti|true damage}}}}"),
        module("{{as|{{pp|70 to 100}} {{sti|true damage}}}} {{as|(+ 20% AP)}}"),
        module("{{as|{{pp|70 to 100}} * 2 {{sti|true damage}}}}"),
        module("{{as|{{pp|70 to 100}} {{sti|true damage}}}} plus {{as|100|true damage}}"),
        "<html>error page</html>".into(),
    ] {
        assert!(SpellData::from_lua(&src).is_err(), "{src}");
    }
}
