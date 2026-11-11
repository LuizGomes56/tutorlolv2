use {
    scraper::Html,
    std::path::Path,
    tutorlolv2_types::{CtxVar, Key},
    tutorlolv2_wiki::{selector, wikitext::*},
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
    assert!(
        got.iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-9),
        "{got:?}"
    );
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

    let labels = ability
        .tabs
        .iter()
        .map(|tab| tab.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(labels, ["Physical Damage", "Reduced Damage", "Slow"]);

    let reduced = &ability.tabs[1];
    assert_eq!(reduced.scalings[0].ratios, [62.5]);
    assert_eq!(ability.tabs[2].unit, "%");

    assert_eq!(
        RawAbility::new("<noinclude>{{Champion data|Bel'Veth}}</noinclude>"),
        Ok(None)
    );
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
                        && rendered
                            .iter()
                            .zip(&base)
                            .all(|(a, b)| (a - b).abs() < 0.01)
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

    println!(
        "matched: {matched}, not rendered: {not_rendered}, mismatched: {}",
        mismatched.len()
    );
    println!("scalings: {scalings}, unparsed: {}", unparsed.len());

    for raw in &unparsed {
        println!("[unparsed] {raw}");
    }

    for mismatch in &mismatched {
        println!("{mismatch}");
    }

    assert!(mismatched.is_empty());
}

#[test]
fn formula_display_matches_current_style() {
    use CtxVar::*;
    let q = Formula::ByRank {
        rank: QLevel,
        values: vec![5.0, 45.0, 85.0, 125.0, 165.0],
    };
    assert_eq!(q.to_string(), "(5.0) + (40.0 * (ctx.q_level - 1.0))");

    let r = Formula::ByRank {
        rank: RLevel,
        values: vec![375.0, 630.0, 945.0],
    };
    assert_eq!(
        r.to_string(),
        "match ctx.r_level {..=1 => 375.0, 2 => 630.0, 3.. => 945.0}"
    );

    let t = Formula::Table {
        var: Level,
        keys: vec![1, 6, 11, 16],
        values: vec![0.3, 0.4, 0.5, 0.6],
    };
    assert_eq!(
        t.to_string(),
        "match ctx.level { ..6 => 0.3, 6..11 => 0.4, 11..16 => 0.5, 16.. => 0.6 }"
    );

    let sum = q + Formula::Var(AttackDamage).scale(1.25);
    assert_eq!(
        sum.to_string(),
        "((5.0) + (40.0 * (ctx.q_level - 1.0))) + (1.25 * ctx.attack_damage)"
    );
}

#[test]
fn formula_eval() {
    use CtxVar::*;
    let sample = Sample(
        [
            (QLevel, 3.0),
            (AbilityPower, 100.0),
            (Level, 0.0),
            (RLevel, 0.0),
        ]
        .into(),
    );
    let f = Formula::ByRank {
        rank: QLevel,
        values: vec![10.0, 20.0, 30.0, 40.0, 50.0],
    } + Formula::Var(AbilityPower) * 0.5;
    assert_eq!(f.eval(&sample), Some(80.0));

    // Below the first key, as R rank 0 or level 1 with keys starting at 10
    let t = Formula::Table {
        var: RLevel,
        keys: vec![1, 2, 3],
        values: vec![0.1, 0.4, 0.7],
    };
    assert_eq!(t.eval(&sample), Some(0.1));

    let p = Formula::Pow(Box::new(Formula::Var(AbilityPower) * 0.0114), 2);
    assert!((p.eval(&sample).unwrap() - 1.2996).abs() < 1e-9);
    assert_eq!(Formula::Raw("ctx.level".into()).eval(&sample), None);
}

#[test]
fn lookups_of_champion_and_item_data() {
    let champions = r#"return { ["Yasuo"] = { ["id"] = 157, ["stats"] = { ["crit_mod"] = 0.95, ["ms"] = 345 } } }"#;
    let items =
        r#"return { ["Infinity Edge"] = { ["id"] = 3031, ["stats"] = { ["critdamage"] = 30 } } }"#;
    let lookup = Lookup::new(champions, items).unwrap();

    assert_eq!(lookup.champion("Yasuo", "crit_mod"), Some(0.95));
    assert_eq!(lookup.champion("Yasuo", "crit_base"), Some(200.0));
    assert_eq!(lookup.item("Infinity Edge", "critdamage"), Some(30.0));

    let src = "{{ap|40*(1+{{#invoke:ItemData|get|item=Infinity Edge|datatype=critdamage}}*0.3/100) to 52}} {{ccd|Yasuo|crit_mod}} {{ccd|Yasuo|unknown}}";
    let expanded = lookup.expand(&Nodes::parse(src)).to_string();
    assert_eq!(
        expanded,
        "{{ap|40*(1+30*0.3/100) to 52}} 0.95 {{ccd|Yasuo|unknown}}"
    );
}

fn pp(src: &str) -> PassiveProgression {
    PassiveProgression::new(&template(src)).unwrap()
}

#[test]
fn passive_progressions() {
    // Aatrox Q: the bottom row fills up to the 4 keys
    let aatrox = pp("{{pp|55 to 70|key=%|1;6;11;16|formula=55% base}}");
    assert_eq!(
        (aatrox.values, aatrox.keys),
        (vec![55.0, 60.0, 65.0, 70.0], vec![1.0, 6.0, 11.0, 16.0])
    );

    // Azir W: both rows have an x filling, so both fill up to 18 values
    let azir = pp("{{pplevel|0; then +8*x|1;10+(x-1)|formula=0 at level 1}}");
    assert_eq!(azir.values.len(), 18);
    assert_eq!(&azir.values[..3], &[0.0, 8.0, 16.0]);
    assert_eq!(&azir.keys[..3], &[1.0, 10.0, 11.0]);

    // Ryze Q's Flux bonus, by the rank of his R
    let ryze = pp(
        "{{pp|type=[[File:Realm Warp.png|20px|border]] ''Realm Warp's'' Rank|key=%|10 to 100 for 4|0 to 3}}",
    );
    assert_eq!(
        (ryze.values, ryze.keys),
        (vec![10.0, 40.0, 70.0, 100.0], vec![0.0, 1.0, 2.0, 3.0])
    );
    assert!(ryze.axis.unwrap().contains("Realm Warp's"));

    // Malzahar W: growth formula over 18 levels, rounded to 2 decimals
    let malzahar = pp("{{pp|5+3.5*(x-1)*(0.7025+0.0175*(x-1))|formula=5 + 10.5 growth}}");
    assert_eq!(malzahar.values.len(), 18);
    assert_eq!(malzahar.values[1], 7.52);
    assert_eq!(malzahar.keys[17], 18.0);

    // Shen Q and Mordekaiser Q: a top row with a range
    assert_eq!(
        pp("{{pp|10 to 40 for 6|1 to 16}}").keys,
        vec![1.0, 4.0, 7.0, 10.0, 13.0, 16.0]
    );
    assert_eq!(
        pp("{{pp|0 + (63-0)/9*(x-1) for 12|1;10 to 20}}").keys[..3],
        [1.0, 10.0, 11.0]
    );

    // Cassiopeia E: 18 values by default
    assert_eq!(pp("{{pp|50 to 120}}").values.len(), 18);
}

#[test]
fn squared_and_split_scalings() {
    let syndra = Scaling::new(
        &template("{{as|'''(+ {{fd|1.14}}% AP)<sup>2</sup>'''}}"),
        Key::W,
    )
    .unwrap();
    assert_eq!(
        (syndra.ratios.clone(), syndra.power, syndra.stat.as_str()),
        (vec![1.14], 2, "AP")
    );

    let nodes = Variables::default().expand(&Nodes::parse(
        "{{as|(+ {{ap|8*0.8}}%|hp}} {{as|of target's '''maximum''' health)}}",
    ));
    let templates = nodes.templates();
    let (first, second) = (templates[0], templates[2]);
    let ksante = Scaling::merged(first, second, Key::W).unwrap();
    assert_eq!(ksante.ratios, vec![6.4]);
    assert_eq!(ksante.stat, "of target's maximum health");
    assert_eq!(Scaling::new(first, Key::W), None);
}

fn page(src: &str) -> AbilityModel {
    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: [("Realm Warp".to_string(), Key::R)].into(),
    };
    AbilityModel::new(src, &context).unwrap().unwrap().1
}

fn data(skill: &str, fields: &str) -> String {
    format!(
        "{{{{{{{{{{1<noinclude>|Ability data</noinclude>}}}}}}|Name\n|skill = {skill}\n|damagetype = Magic\n{fields}\n}}}}"
    )
}

fn sample(pairs: &[(CtxVar, f64)]) -> Sample {
    Sample(pairs.iter().copied().collect())
}

#[test]
fn effects_are_sums_of_terms() {
    use CtxVar::*;
    // Azir W: level table + rank values + ranked AP ratio
    let azir = page(&data(
        "W",
        "|leveling2 = {{st|Magic Damage|{{pplevel|0; then +8*x|1;10+(x-1)}} (+ {{ap|50 to 110}}) {{as|(+ {{ap|35 to 65}}% AP)}}}}",
    ));
    let effect = &azir.effects[0];
    assert_eq!((effect.label.as_str(), effect.section), ("Magic Damage", 1));
    assert_eq!(azir.damage_type, tutorlolv2_types::DamageType::Magic);
    let s = sample(&[(Level, 11.0), (WLevel, 2.0), (AbilityPower, 100.0)]);
    assert!((effect.formula.eval(&s).unwrap() - (16.0 + 65.0 + 42.5)).abs() < 1e-9);

    // Ultimates have 3 ranks by r_level
    let r = page(&data(
        "R",
        "|leveling = {{st|Bonus Movement Speed|{{ap|60 to 100}}%}}",
    ));
    assert_eq!(r.effects[0].unit, "%");
    assert_eq!(
        r.effects[0].formula,
        Formula::ByRank {
            rank: RLevel,
            values: vec![60.0, 80.0, 100.0]
        }
    );

    // Syndra W: the squared term
    let syndra = page(&data(
        "W",
        "|leveling4 = {{st|Bonus True Damage|{{ap|70*0.12 to 190*0.12|round=1}} {{as|'''(+ {{fd|1.14}}% AP)<sup>2</sup>'''}}}}",
    ));
    let s = sample(&[(WLevel, 1.0), (AbilityPower, 100.0)]);
    assert!((syndra.effects[0].formula.eval(&s).unwrap() - (8.4 + 1.2996)).abs() < 1e-9);

    // K'Sante W: the split scaling is merged. Vel'Koz Q: the colored value is a value
    let ksante = page(&data(
        "W",
        "|leveling = {{st|Physical Damage|{{ap|10}} {{as|(+ 8%|hp}} {{as|of target's '''maximum''' health)}}}}",
    ));
    assert!(
        ksante.effects[0].issues.is_empty(),
        "{:?}",
        ksante.effects[0].issues
    );
    let s = sample(&[(WLevel, 1.0), (EnemyMaxHealth, 1000.0)]);
    assert_eq!(ksante.effects[0].formula.eval(&s), Some(90.0));

    let colored = page(&data(
        "Q",
        "|leveling = {{st|Mana Restored|{{as|{{ap|20 to 30}}|mana}}}}",
    ));
    assert_eq!(
        colored.effects[0].formula.eval(&sample(&[(QLevel, 5.0)])),
        Some(30.0)
    );
}

#[test]
fn issues_and_extras() {
    use CtxVar::*;
    // Non-numeric values and unknown templates are issues, not panics
    let partial = page(&data(
        "Q",
        "|leveling = {{st|Magic Damage|{{ap|10 to 50}} (+ {{ai|Feast|Cho'Gath}})}}",
    ));
    assert_eq!(partial.effects[0].issues.len(), 1);
    assert_eq!(
        partial.effects[0].formula.eval(&sample(&[(QLevel, 1.0)])),
        Some(10.0)
    );

    let text = page(&data(
        "Q",
        "|leveling = {{st|Magic Damage|{{ap|1 to 5|10 to 20}}}}",
    ));
    assert_eq!(text.effects[0].issues.len(), 1);

    // Ryze Q: the Flux bonus by R rank
    let ryze = page(&data(
        "Q",
        "|description4 = deals {{pp|type=''Realm Warp's'' Rank|key=%|10 to 100 for 4|0 to 3}} increased damage",
    ));
    assert_eq!(ryze.extras[0].label, "description4");
    assert_eq!(
        ryze.extras[0].formula.eval(&sample(&[(RLevel, 2.0)])),
        Some(70.0)
    );
}

#[test]
fn champions_are_assembled_by_file_order() {
    let dir = std::env::temp_dir().join("tutorlolv2_wikitext_assembly");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let page = |name: &str, skill: &str, label: &str| {
        format!(
            "{{{{{{{{{{1<noinclude>|Ability data</noinclude>}}}}}}|{name}\n|skill = {skill}\n|leveling = {{{{st|{label}|{{{{ap|1 to 5}}}}|{label}|{{{{ap|2 to 6}}}}}}}}\n}}}}"
        )
    };
    std::fs::write(dir.join("Q1.lua"), page("Second", "Q", "Magic Damage")).unwrap();
    std::fs::write(dir.join("Q0.lua"), page("First", "Q", "Magic Damage")).unwrap();
    std::fs::write(
        dir.join("W0.lua"),
        "<noinclude>{{Champion data|Jayce}}</noinclude>",
    )
    .unwrap();
    std::fs::write(
        dir.join("E0.lua"),
        page("Bad", "{{Data Jayce/Hextech Capacitor|pst2|skill}}", "X"),
    )
    .unwrap();

    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let (model, skipped) = ChampionModel::from_dir("Jayce", &dir, &lookup);

    let q = &model.abilities[&Key::Q];
    assert_eq!(
        (q[0].name.as_str(), q[1].name.as_str()),
        ("First", "Second")
    );
    // A repeated label keeps both effects
    assert_eq!(q[0].effects.len(), 2);
    assert!(!model.abilities.contains_key(&Key::W));
    assert_eq!(skipped.len(), 1);
    assert!(skipped[0].contains("E0.lua"));
}

#[test]
#[ignore = "needs the cache of wiki pages"]
fn every_cached_champion_parses() {
    let lookup = Lookup::from_cache_at(Path::new("..")).unwrap();
    let mut skipped = Vec::new();

    for entry in std::fs::read_dir("../cache/wiki/champions")
        .unwrap()
        .flatten()
    {
        let abilities = entry.path().join("abilities");

        if abilities.is_dir() {
            let id = entry.file_name().into_string().unwrap();
            skipped.extend(ChampionModel::from_dir(&id, &abilities, &lookup).1);
        }
    }

    println!("{skipped:#?}");
    assert!(skipped.len() < 5, "{} pages skipped", skipped.len());
}

#[test]
fn legacy_formulas_compare_by_value() {
    let samples = Sample::series();
    let values = |src: &str| -> Vec<f64> {
        let f = LegacyFormula::parse(src).unwrap();
        samples.iter().map(|s| f.eval(s)).collect()
    };

    let a = values("((5.0) + (40.0 * (ctx.q_level - 1.0))) + ((1.25) * ctx.attack_damage)");
    let b = values("ctx.attack_damage * 1.25 + 5.0 + 40.0 * (ctx.q_level - 1.0)");
    assert!(Report::equal(&a, &b));
    assert!(!Report::equal(&a, &values("0.8 * ctx.ability_power")));

    let table = LegacyFormula::parse(
        "(match ctx.level { ..6 => 0.3, 6..11 => 0.4, 11..16 => 0.5, 16.. => 0.6 }) * ctx.bonus_move_speed",
    )
    .unwrap();
    let s = Sample([(CtxVar::Level, 11.0), (CtxVar::BonusMoveSpeed, 100.0)].into());
    assert!((table.eval(&s) - 50.0).abs() < 1e-9);
    let ranks = LegacyFormula::parse("match ctx.r_level {..=1 => 375.0, 2 => 630.0, 3.. => 945.0}")
        .unwrap();
    assert_eq!(ranks.eval(&Sample([(CtxVar::RLevel, 2.0)].into())), 630.0);

    // Both non-finite at a sample is equal; one finite and one not is different
    assert!(Report::equal(&[f64::INFINITY], &[f64::NAN]));
    assert!(!Report::equal(&[1.0], &[f64::INFINITY]));

    // Model formulas print to strings the same reader understands
    let f = Formula::ByRank {
        rank: CtxVar::QLevel,
        values: vec![5.0, 45.0, 85.0, 125.0, 165.0],
    } + Formula::Var(CtxVar::AttackDamage) * 1.25;
    let printed = values(&f.to_string());
    let evaluated = samples
        .iter()
        .map(|s| f.eval(s).unwrap())
        .collect::<Vec<_>>();
    assert!(Report::equal(&printed, &evaluated));
    assert!(Report::equal(&printed, &a));
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn at_least_60_percent_of_formulas_are_equal() {
    let report = Report::new(Path::new("..")).unwrap();
    println!(
        "equal {} different {} unreadable {} only current {} only model {} ({:.1}%)",
        report.equal,
        report.different,
        report.unreadable,
        report.only_current,
        report.only_model,
        report.percent_equal
    );
    assert!(report.percent_equal >= 60.0);
}

#[test]
fn unmatched_rows_and_empty_values_never_panic() {
    // The wiki shows "Variables are unmatched" for rows of different sizes
    assert!(PassiveProgression::new(&template("{{pp|1;2|5}}")).is_err());

    let tables = [
        Formula::Table {
            var: CtxVar::Level,
            keys: vec![1],
            values: vec![1.0, 2.0],
        },
        Formula::Table {
            var: CtxVar::Level,
            keys: vec![],
            values: vec![],
        },
        Formula::ByRank {
            rank: CtxVar::QLevel,
            values: vec![],
        },
    ];

    for formula in tables {
        let _ = formula.to_string();
    }

    // `{{ap}}` without arguments has no ratio
    assert_eq!(
        Scaling::new(&template("{{as|(+ {{ap}}% AP)}}"), Key::Q),
        None
    );
}

#[test]
fn plain_number_bases() {
    use CtxVar::*;
    // Cho'Gath R: `1200 {{as|(+ 50% AP)}}`
    let chogath = page(&data(
        "R",
        "|leveling = {{st|Non-Champion True Damage|1200 {{as|(+ 50% AP)}}}}",
    ));
    let s = sample(&[(RLevel, 1.0), (AbilityPower, 100.0)]);
    assert_eq!(chogath.effects[0].formula.eval(&s), Some(1250.0));
    assert!(chogath.effects[0].issues.is_empty());

    // Text that is not a number is an issue
    let text = page(&data(
        "R",
        "|leveling = {{st|Damage|about {{as|(+ 50% AP)}}}}",
    ));
    assert_eq!(text.effects[0].issues.len(), 1);
}

#[test]
fn printed_formulas_evaluate_as_eval_beyond_the_last_rank() {
    use CtxVar::*;
    let formulas = [
        // Aurelion Sol Q lists 4 values of a 5 rank ability
        Formula::ByRank {
            rank: QLevel,
            values: vec![100.0, 150.0, 200.0, 250.0],
        },
        // Aphelios P lists 6 values by level
        Formula::ByRank {
            rank: Level,
            values: vec![4.0, 8.0, 12.0, 16.0, 20.0, 24.0],
        },
        Formula::ByRank {
            rank: RLevel,
            values: vec![100.0, 200.0, 300.0],
        },
    ];

    for formula in formulas {
        let printed = LegacyFormula::parse(&formula.to_string()).unwrap();

        for rank in 0..=18 {
            let s = sample(&[
                (QLevel, rank as f64),
                (Level, rank as f64),
                (RLevel, rank as f64),
            ]);
            assert_eq!(
                Some(printed.eval(&s)),
                formula.eval(&s),
                "{formula} at {rank}"
            );
        }
    }
}

#[test]
fn passive_progressions_round_as_the_module() {
    // Kai'Sa: `round=ceil`
    assert_eq!(
        pp("{{pp|1 to 2 for 3|round=ceil}}").values,
        vec![1.0, 2.0, 2.0]
    );
    assert_eq!(
        pp("{{pp|1 to 2 for 3|round=floor}}").values,
        vec![1.0, 1.0, 2.0]
    );
}

#[test]
fn per_percent_scalings() {
    use CtxVar::*;
    // Katarina R: 240% (+ 750% per 100% bonus attack speed) bonus AD
    let katarina = page(&data(
        "R",
        "|leveling = {{st|Maximum Physical Damage|{{as|{{ap|16*15}}% {{as|(+ {{ap|50*15}}% per 100% '''bonus''' attack speed)}} '''bonus''' AD}}}}",
    ));
    let s = sample(&[(BonusAttackSpeed, 50.0), (BonusAd, 100.0)]);
    let value = katarina.effects[0].formula.eval(&s).unwrap();
    assert!((value - (2.4 + 7.5 * 0.5) * 100.0).abs() < 1e-9, "{value}");
}

#[test]
fn rounding_of_the_wiki_is_equal() {
    // Maokai E: 33.3325 per rank against the rounded 166.67
    assert!(Report::equal(&[166.663], &[166.67]));
    // Rumble Q: rounded %-health ratios against 1000 maximum health
    assert!(Report::equal(&[4.67], &[4.67 + 0.003]));
    // Differences beyond rounding are still different
    assert!(!Report::equal(&[100.0], &[101.0]));
    assert!(!Report::equal(&[0.5], &[0.6]));
}

/// Like `page`, for the champions whose ultimate has 4 ranks, as Karma and Nidalee
fn four_rank_page(src: &str) -> AbilityModel {
    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: true,
        ability_keys: Default::default(),
    };
    AbilityModel::new(src, &context).unwrap().unwrap().1
}

#[test]
fn trailing_stat_multiplies_the_ratio_before_it() {
    use CtxVar::*;
    // Yorick E: 6-8% (+ 3% per 100 AP) of target's maximum health
    let yorick = page(&data(
        "E",
        "|leveling = {{st|Magic Damage|{{as|{{ap|6 to 8}}%|hp}} {{as|(+ 3% per 100 AP)}} {{as|of target's '''maximum''' health}}}}",
    ));
    let effect = &yorick.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[
        (ELevel, 1.0),
        (AbilityPower, 100.0),
        (EnemyMaxHealth, 1000.0),
    ]);
    assert!((effect.formula.eval(&s).unwrap() - 90.0).abs() < 1e-9);
}

#[test]
fn times_multiplies() {
    use CtxVar::*;
    // Akshan E: (8-40 (+ 25% AD)) × (1 + 0.3 per 100% bonus attack speed)
    let akshan = page(&data(
        "E",
        "|leveling3 = {{st|Physical Damage per Shot|{{ap|8 to 40}} {{as|(+ 25% AD)}} {{times}} {{as|(1 + {{fd|0.3}} per 100% '''bonus''' attack speed)}}}}",
    ));
    let effect = &akshan.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[
        (ELevel, 1.0),
        (AttackDamage, 100.0),
        (BonusAttackSpeed, 50.0),
    ]);
    assert!((effect.formula.eval(&s).unwrap() - 33.0 * 1.15).abs() < 1e-9);
}

#[test]
fn values_inside_icon_and_tooltip_templates() {
    use CtxVar::*;
    // Karma Q: the Mantra bonus by R rank, inside the icon of Mantra
    let karma = four_rank_page(&data(
        "Q",
        "|leveling2 = {{st|Total Damage|{{ap|60 to 260}} (+ {{ai|Mantra|Karma|{{ap|40+40 to 220+310 4}}}}) {{as|(+ {{ap|70+50+30}}% AP)}}}}",
    ));
    let effect = &karma.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(QLevel, 1.0), (RLevel, 2.0), (AbilityPower, 0.0)]);
    assert_eq!(effect.formula.eval(&s), Some(60.0 + 230.0));

    // Gangplank Q: gold, inside `{{g}}`
    let gangplank = page(&data(
        "Q",
        "|leveling = {{st|Gold Plunder|{{g|{{ap|3 to 7}}}}}}",
    ));
    assert_eq!(
        gangplank.effects[0].formula.eval(&sample(&[(QLevel, 5.0)])),
        Some(7.0)
    );

    // Nidalee W: a cooldown inside a tooltip, by the rank of her R
    let nidalee = four_rank_page(&data(
        "W",
        "|leveling = {{st|Reduced Cooldown|{{tt|{{ap|3 to 1.5 4}} seconds|Affected by ability haste}}}}",
    ));
    assert_eq!(
        nidalee.effects[0].formula.eval(&sample(&[(RLevel, 4.0)])),
        Some(1.5)
    );
}

#[test]
fn values_per_collected_thing_are_by_stacks() {
    use CtxVar::*;
    // Thresh E: 1.7 per Soul collected
    let thresh = page(&data(
        "E",
        "|leveling = {{st|Minimum Bonus Magic Damage|{{fd|1.7}} per {{ai|Damnation|Thresh|Soul}} collected {{as|(+ 0% AD)}}}}",
    ));
    let effect = &thresh.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(Stacks, 10.0), (AttackDamage, 100.0)]);
    assert!((effect.formula.eval(&s).unwrap() - 17.0).abs() < 1e-9);
}

#[test]
fn trailing_stat_closes_the_group_its_parenthesis_opened() {
    use CtxVar::*;
    // K'Sante W: the base is not multiplied by the target's maximum health
    let ksante = page(&data(
        "W",
        "|leveling = {{st|Physical Damage|{{ap|45 to 165}} {{as|(+ 8%|hp}} {{as|(+ 2% per 100 '''bonus''' armor)}} {{as|(+ 2% per 100 '''bonus''' magic resistance)}} {{as|of target's '''maximum''' health)}}}}",
    ));
    let effect = &ksante.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[
        (WLevel, 1.0),
        (BonusArmor, 100.0),
        (BonusMagicResist, 100.0),
        (EnemyMaxHealth, 1000.0),
    ]);
    assert!((effect.formula.eval(&s).unwrap() - (45.0 + 0.12 * 1000.0)).abs() < 1e-9);
}

#[test]
fn scalings_and_stacks_inside_icons() {
    use CtxVar::*;
    // Gwen Q: a scaling inside the icon of her passive
    let gwen = page(&data(
        "Q",
        "|leveling = {{st|Center Damage per Snip|{{ap|10 to 26}} {{as|(+ 5% AP)}}<br />{{ai|A Thousand Cuts|Gwen|{{as|(+ 1% {{as|(+ {{fd|0.6}}% per 100 AP)}} of the target's '''maximum''' health)}}}}}}",
    ));
    let effect = &gwen.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[
        (QLevel, 1.0),
        (AbilityPower, 100.0),
        (EnemyMaxHealth, 1000.0),
    ]);
    assert!((effect.formula.eval(&s).unwrap() - (10.0 + 5.0 + 0.016 * 1000.0)).abs() < 1e-9);

    // Thresh W: 2 per Soul collected, written as text
    let thresh = page(&data(
        "W",
        "|leveling = {{st|Shield Strength|{{ap|50 to 130}} (+ 2 per {{ai|Damnation|Thresh|Soul}} collected)}}",
    ));
    let effect = &thresh.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(WLevel, 1.0), (Stacks, 10.0)]);
    assert_eq!(effect.formula.eval(&s), Some(70.0));

    // Senna R: the icon displays "Mist", her stacks
    let senna = page(&data(
        "R",
        "|leveling = {{st|Shield Strength|{{ap|120 to 200}} {{as|(+ 50% AP)}} {{as|(+ 150% {{ai|Absolution|Senna|Mist}})|mist}}}}",
    ));
    let effect = &senna.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(RLevel, 1.0), (AbilityPower, 0.0), (Stacks, 10.0)]);
    assert_eq!(effect.formula.eval(&s), Some(135.0));
}

#[test]
fn four_values_of_a_basic_ability_extend_to_five_ranks() {
    use CtxVar::*;
    // Aurelion Sol Q lists ranks 1-4, the 5th continues the same step
    let aurelion = page(&data(
        "Q",
        "|leveling = {{st|Total Maximum Magic Damage|{{ap|(45/8)*26 to ((45+(105-45)*(3/4))/8)*26 4}}}}",
    ));
    let s = sample(&[(QLevel, 5.0)]);
    assert!((aurelion.effects[0].formula.eval(&s).unwrap() - 341.25).abs() < 1e-9);
}

#[test]
fn damage_stored_and_dealt_are_stacks() {
    use CtxVar::*;
    // Zed R: 100% AD (+ 25/40/55% of damage stored)
    let zed = page(&data(
        "R",
        "|leveling = {{st|Physical Damage|{{as|100% AD}} (+ {{ap|25|40|55}}% of damage stored)}}",
    ));
    let effect = &zed.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(RLevel, 2.0), (AttackDamage, 100.0), (Stacks, 1000.0)]);
    assert!((effect.formula.eval(&s).unwrap() - 500.0).abs() < 1e-9);

    // Yone E: 25-35% of damage dealt
    let yone = page(&data(
        "E",
        "|leveling = {{st|Damage Stored|{{ap|25 to 35}}% of damage dealt}}",
    ));
    let effect = &yone.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(ELevel, 1.0), (Stacks, 100.0)]);
    assert!((effect.formula.eval(&s).unwrap() - 25.0).abs() < 1e-9);
}

#[test]
fn expended_grit_is_the_maximum_resource() {
    use CtxVar::*;
    // Sett W: (+ 25% (+ 25% per 100 bonus AD) of expended Grit), Grit is his resource
    let sett = page(&data(
        "W",
        "|leveling = {{st|Damage|{{ap|80 to 160}} (+ 25% {{as|(+ 25% per 100 '''bonus''' AD)}} of expended ''Grit'')}}",
    ));
    let effect = &sett.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[(WLevel, 1.0), (BonusAd, 100.0), (MaxMana, 340.0)]);
    assert!((effect.formula.eval(&s).unwrap() - (80.0 + 0.5 * 340.0)).abs() < 1e-9);
}

#[test]
fn scalings_grouped_inside_one_as_and_icons_without_values() {
    use CtxVar::*;
    // Tahm Kench Q: 75-255 (+ 100% AP) + [icon 5-60 (based on level)
    // (+ 1.25% AP per 100 bonus health) (+ 4% bonus health)]
    let tahm = page(&data(
        "Q",
        "|leveling = {{st|Magic Damage|{{ap|75 to 255}} {{as|(+ 100% AP)}} + [ {{ai|An Acquired Taste|Tahm Kench|icononly=true}} {{pp|5 to 60 for 12}} {{as|({{as|+ {{fd|1.25}}% AP}} {{as|per 100 '''bonus''' health)|hp}} {{as|(+ 4% '''bonus''' health)}}}} ]}}",
    ));
    let effect = &tahm.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let s = sample(&[
        (QLevel, 1.0),
        (Level, 1.0),
        (AbilityPower, 100.0),
        (BonusHealth, 1000.0),
    ]);
    let expected = 75.0 + 100.0 + 5.0 + 0.0125 * 100.0 * 1000.0 / 100.0 + 0.04 * 1000.0;
    assert!((effect.formula.eval(&s).unwrap() - expected).abs() < 1e-9);
}
