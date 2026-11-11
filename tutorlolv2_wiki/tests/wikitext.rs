use {
    scraper::Html,
    std::path::Path,
    tutorlolv2_types::Key,
    tutorlolv2_wiki::{
        selector,
        wikitext::{Expr, Nodes, Progression, RawAbility, Scaling, Template, Value, Variables},
    },
};

/// First template of `src`, after expanding its variables
fn template(src: &str) -> Template {
    let nodes = Variables::default().expand(&Nodes::parse(src));
    nodes.templates()[0].clone()
}

fn values(src: &str, skill: Key) -> Vec<Value> {
    Progression::new(&template(src), skill).unwrap().values
}

fn numbers(src: &str, skill: Key) -> Vec<f64> {
    Progression::new(&template(src), skill)
        .unwrap()
        .numbers()
        .unwrap()
}

#[test]
fn evaluates_wiki_arithmetic() {
    assert_eq!(Expr::new("40*(1+0.4*2)").eval(), Some(72.0));
    assert_eq!(Expr::new("70/10").eval(), Some(7.0));
    assert_eq!(Expr::new("-2^2").eval(), Some(4.0));
    assert_eq!(Expr::new("2^3^2").eval(), Some(64.0));
    assert_eq!(Expr::new("7 mod 3").eval(), Some(1.0));
    assert_eq!(Expr::new("1.23456 round 2").eval(), Some(1.23));
    assert_eq!(Expr::new("5 to 10").eval(), None);
    assert_eq!(Expr::new("").eval(), None);
}

#[test]
fn ranges_have_five_values_or_three_in_ultimates() {
    // Gnar's Boulder Toss
    assert_eq!(
        numbers("{{ap|45 to 225}}", Key::Q),
        [45.0, 90.0, 135.0, 180.0, 225.0]
    );
    // Aatrox's World Ender, rendered as 60 / 80 / 100%
    assert_eq!(numbers("{{ap|60 to 100}}", Key::R), [60.0, 80.0, 100.0]);
}

#[test]
fn explicit_counts_override_the_default() {
    // Udyr's Blazing Stampede
    assert_eq!(
        numbers("{{ap|25 to 55 6}}", Key::R),
        [25.0, 31.0, 37.0, 43.0, 49.0, 55.0]
    );
    assert_eq!(numbers("{{ap|8 to 22 4}}", Key::E).len(), 4);
}

#[test]
fn only_the_first_range_uses_the_default_count() {
    assert_eq!(
        values("{{ap|1 to 5|10 to 20}}", Key::Q),
        [1.0, 2.0, 3.0, 4.0, 5.0]
            .map(Value::Number)
            .into_iter()
            .chain([Value::Text("10 to 20".into())])
            .collect::<Vec<_>>()
    );
}

#[test]
fn arithmetic_in_ranges_and_single_values() {
    assert_eq!(
        numbers("{{ap|5/2 to 165/2}}", Key::Q),
        [2.5, 22.5, 42.5, 62.5, 82.5]
    );
    assert_eq!(numbers("{{ap|70/10}}", Key::E), [7.0]);
    assert_eq!(numbers("{{ap|300|475|650}}", Key::R), [300.0, 475.0, 650.0]);
    assert_eq!(numbers("{{ap|75/7|round=2}}", Key::Q), [10.71]);
}

#[test]
fn formulas_of_nested_ranges() {
    // Nidalee's Takedown: (5 to 80) * (1 + (1 to 1.75)) over 4 ranks
    let expected = [5.0 * 2.0, 30.0 * 2.25, 55.0 * 2.5, 80.0 * 2.75];
    let got = numbers("{{ap|(5 to 80)*(1+(1 to 1.75)) 4}}", Key::Q);

    assert_eq!(got.len(), 4);
    assert!(got.iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-9), "{got:?}");
}

#[test]
fn variables_are_replaced_in_order() {
    let src = "{{#vardefine:b1|10}}{{#vardefine:b2|{{#expr:{{#var:b1}}*7}}}}{{ap|{{#var:b1}} to {{#var:b2}}}}";
    assert_eq!(numbers(src, Key::Q), [10.0, 25.0, 40.0, 55.0, 70.0]);
}

#[test]
fn scalings() {
    let scaling = |src: &str| Scaling::new(&template(src), Key::Q);

    let simple = scaling("{{as|(+ 140% '''bonus''' AD)}}").unwrap();
    assert_eq!(simple.ratios, [140.0]);
    assert_eq!(simple.stat, "bonus AD");
    assert!(simple.additive);

    let ranked = scaling("{{as|(+ {{ap|45 to 105}}% '''bonus''' AD)}}").unwrap();
    assert_eq!(ranked.ratios, [45.0, 60.0, 75.0, 90.0, 105.0]);

    let per = scaling("{{as|(+ 5% per 100 '''bonus''' AD)}}").unwrap();
    assert_eq!((per.ratios[0], per.per), (5.0, Some(100.0)));

    let nested =
        scaling("{{as|6% {{as|(+ 1.5% per 100 AP)}} of target's '''maximum''' health}}").unwrap();
    assert!(!nested.additive);
    assert_eq!(nested.stat, "of target's maximum health");
    assert_eq!(nested.nested[0].per, Some(100.0));

    assert_eq!(scaling("{{as|physical damage}}"), None);
}

#[test]
fn ability_data_pages() {
    let src = r#"<!-- comment -->{{{{{1<noinclude>|Ability data</noinclude>}}}|Boomerang Throw|{{{2|}}}
|champion     = Gnar
|skill        = Q
|damagetype   = Physical
|leveling     = {{st|Physical Damage|{{ap|5 to 165}} {{as|(+ 125% AD)}}|Reduced Damage|{{ap|5/2 to 165/2}} {{as|(+ {{ap|125/2}}% AD)}}}}{{st|Slow|{{ap|15 to 35}}%}}
}}"#;

    let ability = RawAbility::new(src).unwrap().unwrap();
    assert_eq!(ability.name, "Boomerang Throw");
    assert_eq!(ability.skill, Key::Q);
    assert_eq!(ability.damage_type.as_deref(), Some("Physical"));

    let labels = ability.tabs.iter().map(|tab| tab.label.as_str()).collect::<Vec<_>>();
    assert_eq!(labels, ["Physical Damage", "Reduced Damage", "Slow"]);

    let reduced = &ability.tabs[1];
    assert_eq!(reduced.scalings[0].ratios, [62.5]);
    assert_eq!(ability.tabs[2].unit, "%");

    assert_eq!(RawAbility::new("<noinclude>{{Champion data|Bel'Veth}}</noinclude>"), Ok(None));
}

/// Leading `a / b / c` values of a rendered `<dd>`, up to the first value followed
/// by text, such as the `%` or the scalings after the last value
fn rendered_values(text: &str) -> Vec<f64> {
    let mut values = Vec::new();

    for part in text.split('/').map(str::trim) {
        let end = part
            .find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | ',' | '-')))
            .unwrap_or(part.len());

        match part[..end].replace(',', "").parse() {
            Ok(value) => values.push(value),
            Err(_) => break,
        }

        if end != part.len() {
            break;
        }
    }

    values
}

/// Compares the base values of every cached ability data page with its rendered html.
/// Fails if a page changed between the downloads of its html and of its source
#[test]
#[ignore = "needs the cache of wiki pages"]
fn base_values_match_the_rendered_pages() {
    let root = Path::new("../cache/wiki/champions");

    let (mut matched, mut mismatched, mut not_rendered) = (0, Vec::new(), 0);
    let (mut scalings, mut unparsed) = (0, Vec::new());

    for champion in std::fs::read_dir(root).unwrap().flatten() {
        let Ok(entries) = std::fs::read_dir(champion.path().join("abilities")) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().is_none_or(|ext| ext != "lua") {
                continue;
            }

            let src = std::fs::read_to_string(&path).unwrap();
            let Ok(Some(ability)) = RawAbility::new(&src) else {
                continue;
            };

            let html = std::fs::read_to_string(path.with_extension("html")).unwrap();
            let html = Html::parse_document(&html);

            let rendered = html
                .select(selector!("div.ability-info-stats dl"))
                .flat_map(|dl| {
                    let dts = dl.select(selector!("dt"));
                    let dds = dl.select(selector!("dd"));

                    dts.zip(dds)
                        .map(|(dt, dd)| {
                            let label = dt.text().collect::<String>();
                            let label = label.trim().trim_end_matches(':').trim().to_string();
                            (label, rendered_values(&dd.text().collect::<String>()))
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            for tab in &ability.tabs {
                scalings += tab.scalings.len();
                unparsed.extend(tab.unparsed.iter().map(|raw| format!("{path:?} {raw}")));

                let Some(base) = tab.base.as_ref().and_then(|base| base.numbers()) else {
                    continue;
                };

                let candidates = rendered
                    .iter()
                    .filter(|(label, _)| *label == tab.label)
                    .collect::<Vec<_>>();

                if candidates.is_empty() {
                    not_rendered += 1;
                    continue;
                }

                // Rendered values are rounded for display
                let same = |rendered: &[f64]| {
                    rendered.len() == base.len()
                        && rendered.iter().zip(&base).all(|(a, b)| (a - b).abs() < 0.01)
                };

                match candidates.iter().any(|(_, rendered)| same(rendered)) {
                    true => matched += 1,
                    false => mismatched.push(format!(
                        "{path:?} {:?}: {base:?} != {:?}",
                        tab.label,
                        candidates.iter().map(|(_, v)| v).collect::<Vec<_>>()
                    )),
                }
            }
        }
    }

    println!("matched: {matched}, not rendered: {not_rendered}, mismatched: {}", mismatched.len());
    println!("scalings: {scalings}, unparsed: {}", unparsed.len());

    for raw in &unparsed {
        println!("[unparsed] {raw}");
    }

    for mismatch in &mismatched {
        println!("{mismatch}");
    }

    assert!(mismatched.is_empty());
}
