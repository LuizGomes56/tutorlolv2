use {
    scraper::Html,
    std::path::Path,
    tutorlolv2_types::{CtxVar, Key, MapMode},
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
    let sample = Sample::new([
        (QLevel, 3.0),
        (AbilityPower, 100.0),
        (Level, 0.0),
        (RLevel, 0.0),
    ]);
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
        crit_damage: 200.0,
    };
    AbilityModel::new(src, &context).unwrap().unwrap().1
}

fn data(skill: &str, fields: &str) -> String {
    format!(
        "{{{{{{{{{{1<noinclude>|Ability data</noinclude>}}}}}}|Name\n|skill = {skill}\n|damagetype = Magic\n{fields}\n}}}}"
    )
}

fn sample(pairs: &[(CtxVar, f64)]) -> Sample {
    Sample::new(pairs.iter().copied())
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
    let s = Sample::new([(CtxVar::Level, 11.0), (CtxVar::BonusMoveSpeed, 100.0)]);
    assert!((table.eval(&s) - 50.0).abs() < 1e-9);
    let ranks = LegacyFormula::parse("match ctx.r_level {..=1 => 375.0, 2 => 630.0, 3.. => 945.0}")
        .unwrap();
    assert_eq!(ranks.eval(&Sample::new([(CtxVar::RLevel, 2.0)])), 630.0);

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
        crit_damage: 200.0,
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

#[test]
fn values_by_map() {
    use CtxVar::*;
    let ratio = Formula::ByRank {
        rank: RLevel,
        values: vec![0.2, 0.3, 0.4],
    }
    .with_map(
        MapMode::Arena,
        Formula::ByRank {
            rank: RLevel,
            values: vec![0.3, 0.425, 0.55],
        },
    )
    .with_map(MapMode::Aram, Formula::Const(0.5));
    let f = ratio.clone() * Formula::Var(AttackDamage);

    let s = Sample::new([(RLevel, 2.0), (AttackDamage, 100.0)]);
    assert_eq!(f.eval(&s), Some(30.0));
    assert_eq!(f.eval(&s.clone().on(MapMode::Arena)), Some(42.5));
    assert_eq!(f.eval(&s.on(MapMode::Aram)), Some(50.0));

    // One `ByMap` with an arm per mode, not nested ones
    let Formula::ByMap { maps, .. } = &ratio else {
        panic!()
    };
    assert_eq!(maps.len(), 2);

    assert_eq!(
        ratio.to_string(),
        "match ctx.map { #[cold] Aram => 0.5, #[cold] Arena => (0.3) + (0.125 * (ctx.r_level - 1.0)), _ => (0.2) + (0.1 * (ctx.r_level - 1.0)) }"
    );
    assert_eq!(ratio.default_values(), Some(vec![0.2, 0.3, 0.4]));
}

fn change(text: &str, kind: &str) -> tutorlolv2_wiki::map_changes::change::MapChange {
    let change = tutorlolv2_wiki::map_changes::change::MapChange::parse(text, false);
    assert!(
        format!("{:?}", change.kind).starts_with(kind),
        "{:?}",
        change.kind
    );
    change
}

#[test]
fn map_values_are_read_with_the_target_key() {
    use CtxVar::*;
    let lookup = Lookup::new("return {}", "return {}").unwrap();

    // Aatrox R in Arena, with an explicit count
    let aatrox = change(
        "Bonus attack damage changed to {{as|{{ap|30 to 55 3}}% AD}}.",
        "Ratio",
    );
    let Some(MapValue::Ratio { stat, ratio }) = MapValue::new(&aatrox, Key::R, &lookup) else {
        panic!()
    };
    assert_eq!(stat, AttackDamage);
    assert_eq!(ratio.default_values(), Some(vec![0.3, 0.425, 0.55]));

    // An R without a count has 3 values, not 5
    let r = change("AP ratio changed to {{as|{{ap|60 to 100}}% AP}}.", "Ratio");
    let Some(MapValue::Ratio { ratio, .. }) = MapValue::new(&r, Key::R, &lookup) else {
        panic!()
    };
    assert_eq!(ratio.default_values(), Some(vec![0.6, 0.8, 1.0]));

    let base = change("Base damage changed to {{ap|70 to 190}}.", "Base");
    let Some(MapValue::Base(values)) = MapValue::new(&base, Key::Q, &lookup) else {
        panic!()
    };
    assert_eq!(
        values.default_values(),
        Some(vec![70.0, 100.0, 130.0, 160.0, 190.0])
    );

    let cooldown = change("Cooldown changed to {{ap|18 to 14}} seconds.", "Ignored");
    assert_eq!(MapValue::new(&cooldown, Key::W, &lookup), None);
}

/// Model of a champion with `pages`, built in its own folder named `name`
fn champion(name: &str, pages: &[(&str, &str)]) -> ChampionModel {
    let dir = std::env::temp_dir().join(format!("tutorlolv2_map_{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    for (page, src) in pages {
        std::fs::write(dir.join(format!("{page}.lua")), src).unwrap();
    }

    let lookup = Lookup::new("return {}", "return {}").unwrap();
    ChampionModel::from_dir("Test", &dir, &lookup).0
}

#[test]
fn ratio_changes_wrap_only_the_ratio_and_multiples() {
    use CtxVar::*;
    // A hit and a total of 3 hits, as Vex R
    let mut model = champion(
        "vex",
        &[(
            "R0",
            &data(
                "R",
                "|leveling = {{st|Magic Damage|{{ap|75 to 175}} {{as|(+ 20% AP)}}|Total Magic Damage|{{ap|225 to 525}} {{as|(+ 60% AP)}}}}",
            ),
        )],
    );
    let value = MapValue::Ratio {
        stat: AbilityPower,
        ratio: Formula::Const(0.3),
    };

    let Placement::Applied(placed) =
        model.apply_map_change(Key::R, None, "AP", MapMode::Aram, &value)
    else {
        panic!()
    };
    assert_eq!(placed.len(), 2);

    let effects = &model.abilities[&Key::R][0].effects;
    let s = Sample::new([(RLevel, 1.0), (AbilityPower, 100.0)]);
    assert_eq!(effects[0].formula.eval(&s), Some(95.0));
    assert_eq!(
        effects[0].formula.eval(&s.clone().on(MapMode::Aram)),
        Some(105.0)
    );
    assert_eq!(
        effects[1].formula.eval(&s.on(MapMode::Aram)),
        Some(225.0 + 90.0)
    );
}

#[test]
fn qualifiers_bases_and_unplaced_changes() {
    use CtxVar::*;
    let mut model = champion(
        "dashes",
        &[(
            "Q0",
            &data(
                "Q",
                "|leveling = {{st|First Dash Damage|{{ap|30 to 110}} {{as|(+ 60% AP)}}|Second Dash Damage|{{ap|60 to 220}} {{as|(+ 30% AP)}}}}",
            ),
        )],
    );
    let ratio = MapValue::Ratio {
        stat: AbilityPower,
        ratio: Formula::Const(0.4),
    };

    let Placement::Applied(placed) =
        model.apply_map_change(Key::Q, None, "First dash AP", MapMode::Arena, &ratio)
    else {
        panic!()
    };
    assert_eq!(placed.len(), 1);
    assert_eq!(placed[0].label, "First Dash Damage");

    let base = MapValue::Base(Formula::ByRank {
        rank: QLevel,
        values: vec![40.0, 80.0, 120.0, 160.0, 200.0],
    });
    assert!(matches!(
        model.apply_map_change(Key::Q, None, "Base", MapMode::Arena, &base),
        Placement::Applied(_)
    ));

    assert!(matches!(
        model.apply_map_change(Key::Q, None, "Third slash AP", MapMode::Arena, &ratio),
        Placement::NotPlaced(_)
    ));

    let health = MapValue::Ratio {
        stat: BonusHealth,
        ratio: Formula::Const(0.1),
    };
    assert!(matches!(
        model.apply_map_change(Key::Q, None, "Health", MapMode::Arena, &health),
        Placement::NotPlaced(_)
    ));

    // A stat in a term with a second stat (Tahm Kench's AP × health) is not placed
    let mut tahm = champion(
        "tahm",
        &[(
            "Q0",
            &data(
                "Q",
                "|leveling = {{st|Magic Damage|{{ap|75 to 255}} {{as|(+ 1.25% AP per 100 '''bonus''' health)}}}}",
            ),
        )],
    );
    let tahm_ratio = MapValue::Ratio {
        stat: AbilityPower,
        ratio: Formula::Const(0.02),
    };
    assert!(matches!(
        tahm.apply_map_change(Key::Q, None, "AP", MapMode::Aram, &tahm_ratio),
        Placement::NotPlaced(_)
    ));
}

#[test]
fn map_changes_of_a_module_are_applied_by_mode() {
    use tutorlolv2_wiki::map_changes::{ChangeTarget, MapChanges, MapEntry};

    let mut models = [(
        "Test".to_string(),
        champion(
            "module",
            &[(
                "R0",
                &data(
                    "R",
                    "|leveling = {{st|Magic Damage|{{ap|75 to 175}} {{as|(+ 20% AP)}}}}",
                ),
            )],
        ),
    )]
    .into();

    let entry = MapEntry {
        target: ChangeTarget::Ability {
            champion_id: "Test".into(),
            key: Key::R,
        },
        changes: vec![
            change("AP ratio changed to {{as|30% AP}}.", "Ratio"),
            change("Cooldown changed to 80 seconds.", "Ignored"),
        ],
    };
    let changes = [(
        "aram".to_string(),
        MapChanges {
            maps: vec![
                tutorlolv2_types::GameMap::Aram,
                tutorlolv2_types::GameMap::AramMayhem,
            ],
            entries: [("Test R".to_string(), entry)].into(),
        },
    )]
    .into();

    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let report = MapChangesReport::apply(&mut models, &changes, &lookup);

    assert_eq!((report.applied, report.not_placed), (1, 0));
    assert_eq!(report.entries[0].mode, MapMode::Aram);
    assert_eq!(report.ignored.values().sum::<usize>(), 1);

    let formula = &models["Test"].abilities[&Key::R][0].effects[0].formula;
    let s = Sample::new([(CtxVar::RLevel, 1.0), (CtxVar::AbilityPower, 100.0)]);
    assert_eq!(formula.eval(&s.on(MapMode::Aram)), Some(105.0));
}

#[test]
#[ignore = "needs the cache of wiki pages"]
fn every_damage_change_is_applied_or_explained() {
    let lookup = Lookup::from_cache_at(Path::new("..")).unwrap();
    let mut models = std::collections::BTreeMap::new();

    for entry in std::fs::read_dir("../cache/wiki/champions")
        .unwrap()
        .flatten()
    {
        let abilities = entry.path().join("abilities");

        if abilities.is_dir() {
            let id = entry.file_name().into_string().unwrap();
            models.insert(
                id.clone(),
                ChampionModel::from_dir(&id, &abilities, &lookup).0,
            );
        }
    }

    let changes =
        serde_json::from_slice(&std::fs::read("../cache/wiki/map_changes/full.json").unwrap())
            .unwrap();
    let report = MapChangesReport::apply(&mut models, &changes, &lookup);

    println!(
        "applied {} not placed {}",
        report.applied, report.not_placed
    );

    for entry in report
        .entries
        .iter()
        .filter(|e| matches!(e.placement, Placement::NotPlaced(_)))
    {
        println!(
            "{} {:?} {:?}: {} -> {:?}",
            entry.champion_id, entry.key, entry.mode, entry.text, entry.placement
        );
    }

    assert_eq!(report.applied + report.not_placed, report.entries.len());
    assert!(report.applied > 0);
}

#[test]
fn changes_to_abilities_without_effects_say_what_is_missing() {
    use CtxVar::*;
    // Bard's passive has no `{{st}}`, so there is no AP ratio to change
    let mut bard = champion("bard", &[("P0", &data("I", "|description = Chimes."))]);
    let ratio = MapValue::Ratio {
        stat: AbilityPower,
        ratio: Formula::Const(0.5),
    };

    let Placement::NotPlaced(reason) =
        bard.apply_map_change(Key::P, None, "AP", MapMode::Aram, &ratio)
    else {
        panic!()
    };
    assert!(reason.contains("ratio"), "{reason}");
}

#[test]
fn named_ability_changes_apply_only_to_that_ability() {
    use CtxVar::*;
    // Ambessa: two Q pages, each with its own Arena change
    let mut model = champion(
        "ambessa",
        &[
            (
                "Q0",
                &data("Q", "|leveling = {{st|Physical Damage|{{ap|40 to 100}} {{as|(+ 40% '''bonus''' AD)}}}}")
                    .replace("|Name", "|Cunning Sweep"),
            ),
            (
                "Q1",
                &data("Q", "|leveling = {{st|Physical Damage|{{ap|40 to 100}} {{as|(+ 60% '''bonus''' AD)}}}}")
                    .replace("|Name", "|Sundering Slam"),
            ),
        ],
    );

    for (name, ratio) in [("Cunning Sweep", 0.45), ("Sundering Slam", 0.7)] {
        let value = MapValue::Ratio {
            stat: BonusAd,
            ratio: Formula::Const(ratio),
        };
        let placement =
            model.apply_map_change(Key::Q, Some(name), "Bonus AD", MapMode::Arena, &value);
        assert!(
            matches!(placement, Placement::Applied(ref p) if p.len() == 1),
            "{placement:?}"
        );
    }

    let s = Sample::new([(QLevel, 1.0), (BonusAd, 100.0)]).on(MapMode::Arena);
    let q = &model.abilities[&Key::Q];
    assert_eq!(q[0].effects[0].formula.eval(&s), Some(85.0));
    assert_eq!(q[1].effects[0].formula.eval(&s), Some(110.0));
}

#[test]
fn qualified_changes_are_placed_before_unqualified_ones() {
    use CtxVar::*;
    use tutorlolv2_wiki::map_changes::{ChangeTarget, MapChanges, MapEntry};

    // Kassadin R: an unqualified base change and a qualified one, in the module's order
    let mut models = [(
        "Test".to_string(),
        champion(
            "kassadin",
            &[(
                "R0",
                &data(
                    "R",
                    "|leveling = {{st|Magic Damage|{{ap|80 to 110}}|Bonus Damage Per Stack|{{ap|35 to 55}}}}",
                ),
            )],
        ),
    )]
    .into();

    let entry = MapEntry {
        target: ChangeTarget::Ability {
            champion_id: "Test".into(),
            key: Key::R,
        },
        changes: vec![
            change("Base damage changed to {{ap|100 to 140}}.", "Base"),
            change("Bonus damage per stack changed to {{ap|40 to 60}}.", "Base"),
        ],
    };
    let changes = [(
        "ar".to_string(),
        MapChanges {
            maps: vec![tutorlolv2_types::GameMap::Arena],
            entries: [("Test R".to_string(), entry)].into(),
        },
    )]
    .into();

    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let report = MapChangesReport::apply(&mut models, &changes, &lookup);
    assert_eq!((report.applied, report.not_placed), (2, 0), "{report:?}");

    let effects = &models["Test"].abilities[&Key::R][0].effects;
    let s = Sample::new([(RLevel, 1.0)]).on(MapMode::Arena);
    assert_eq!(effects[0].formula.eval(&s), Some(100.0));
    assert_eq!(effects[1].formula.eval(&s), Some(40.0));
}

#[test]
fn bases_starting_at_zero_and_rank_bases_beside_level_tables() {
    use CtxVar::*;
    // Gnar W: a base that starts at 0 is still the primary
    let mut gnar = champion(
        "gnar",
        &[(
            "W0",
            &data("W", "|leveling = {{st|Bonus Magic Damage|{{ap|0 to 40}}}}"),
        )],
    );
    let base = MapValue::Base(Formula::ByRank {
        rank: WLevel,
        values: vec![5.0, 15.0, 25.0, 35.0, 45.0],
    });
    assert!(matches!(
        gnar.apply_map_change(Key::W, None, "Base", MapMode::Arena, &base),
        Placement::Applied(ref p) if p.len() == 1
    ));

    // Azir W: the base by rank, not the level table before it
    let mut azir = champion(
        "azir",
        &[(
            "W0",
            &data(
                "W",
                "|leveling = {{st|Magic Damage|{{pplevel|0; then +8*x|1;10+(x-1)}} (+ {{ap|50 to 110}})}}",
            ),
        )],
    );
    let rank_base = MapValue::Base(Formula::ByRank {
        rank: WLevel,
        values: vec![60.0, 75.0, 90.0, 105.0, 120.0],
    });
    assert!(matches!(
        azir.apply_map_change(Key::W, None, "Base", MapMode::Arena, &rank_base),
        Placement::Applied(_)
    ));
    let s = Sample::new([(Level, 1.0), (WLevel, 1.0)]).on(MapMode::Arena);
    assert_eq!(
        azir.abilities[&Key::W][0].effects[0].formula.eval(&s),
        Some(60.0)
    );

    // Nothing placed is never counted as applied
    let mut empty = champion(
        "empty",
        &[("W0", &data("W", "|leveling = {{st|Damage|{{ap|0 to 0}}}}"))],
    );
    assert!(matches!(
        empty.apply_map_change(Key::W, None, "Base", MapMode::Arena, &base),
        Placement::Applied(ref p) if p.len() == 1
    ));
}

#[test]
fn formulas_for_generators() {
    use CtxVar::*;
    let f = Formula::Const(13.0)
        + (Level.to_formula() - 1.to_formula())
        + 0.3.to_formula() * BonusAd.to_formula();
    let s = Sample::new([(Level, 3.0), (BonusAd, 100.0)]);
    assert_eq!(f.eval(&s), Some(45.0));
    assert_eq!((-Formula::Var(Level)).eval(&s), Some(-3.0));
    assert!(f.to_string().contains("ctx.level - 1"), "{f}");

    let branch = Formula::Const(0.6).with_map(MapMode::Aram, Formula::Const(0.65))
        * Formula::Var(AbilityPower);
    assert_eq!(
        branch.rust(),
        "(match ctx.map { tutorlolv2_types::MapMode::Aram => { core::hint::cold_path(); 0.65 } _ => 0.6 }) * ctx.ability_power"
    );
    assert_eq!(
        branch.on(MapMode::Aram),
        Formula::Const(0.65) * Formula::Var(AbilityPower)
    );
    assert_eq!(
        branch.on(MapMode::Arena),
        Formula::Const(0.6) * Formula::Var(AbilityPower)
    );
}

#[test]
fn legacy_formulas_read_map_branches() {
    let rust = "(match ctx.map { tutorlolv2_types::MapMode::Aram => { core::hint::cold_path(); 0.65 } _ => 0.6 }) * ctx.ability_power";
    let doc = "(match ctx.map { #[cold] Aram => 0.65, _ => 0.6 }) * ctx.ability_power";
    let s = Sample::new([(CtxVar::AbilityPower, 100.0)]);

    for src in [rust, doc] {
        let f = LegacyFormula::parse(src).unwrap();
        assert!((f.eval(&s) - 60.0).abs() < 1e-9, "{src}");
        assert!(
            (f.eval(&s.clone().on(MapMode::Aram)) - 65.0).abs() < 1e-9,
            "{src}"
        );
    }
}

#[test]
fn the_closest_value_takes_a_change_and_smaller_multiples_follow() {
    use CtxVar::*;
    // Darius Q: the handle is 35% of the blade, and Arena changes the blade's ratio
    let mut model = champion(
        "darius",
        &[(
            "Q0",
            &data(
                "Q",
                "|leveling = {{st|Physical Damage (Blade)|{{ap|50 to 170}} {{as|(+ {{ap|100 to 140}}% AD)}}|Reduced Damage (Handle)|{{ap|50*0.35 to 170*0.35}} {{as|(+ {{ap|100*0.35 to 140*0.35}}% AD)}}}}",
            ),
        )],
    );
    let value = MapValue::Ratio {
        stat: AttackDamage,
        ratio: Formula::ByRank {
            rank: QLevel,
            values: vec![0.8, 0.9, 1.0, 1.1, 1.2],
        },
    };

    let Placement::Applied(placed) =
        model.apply_map_change(Key::Q, None, "AD", MapMode::Arena, &value)
    else {
        panic!()
    };
    assert_eq!(placed.len(), 2);

    let effects = &model.abilities[&Key::Q][0].effects;
    let s = Sample::new([(QLevel, 1.0), (AttackDamage, 100.0)]).on(MapMode::Arena);
    assert_eq!(effects[0].formula.eval(&s), Some(50.0 + 80.0));
    let handle = effects[1].formula.eval(&s).unwrap();
    assert!((handle - (17.5 + 28.0)).abs() < 1e-9, "{handle}");
}

#[test]
fn description_values_keep_their_scalings_under_the_heading() {
    use CtxVar::*;
    let label = |model: &AbilityModel, i: usize| model.extras[i].label.clone();
    let at = |level: f64| sample(&[(Level, level), (BonusAd, 100.0), (AbilityPower, 100.0)]);

    // Darius: values colored by `{{as|…|physical damage}}`, in both branches of `{{ft}}`
    let darius = page(&data(
        "P",
        "|description2 = {{sbc|Hemorrhage:}} For each stack, the target is dealt{{ft|{{as|{{pplevel|13 to 30}}|physical damage}} {{as|(+ 30% '''bonus''' AD)}} {{as|'''total''' physical damage}} over the duration,|{{as|{{pplevel|3.25 to 7.5}}|physical damage}} {{as|(+ {{fd|7.5}}% '''bonus''' AD)}} {{as|physical damage}} every {{fd|1.25}} seconds,}}up to a maximum of{{ft|{{as|{{pplevel|13*5 to 30*5}}|physical damage}} {{as|(+ 150% '''bonus''' AD)}} {{as|'''total''' physical damage}}.|{{as|{{pplevel|3.25*5 to 7.5*5}}|physical damage}} {{as|(+ {{fd|37.5}}% '''bonus''' AD)}} {{as|physical damage}}.}}",
    ));
    assert_eq!(darius.extras.len(), 4);
    assert_eq!(label(&darius, 0), "Hemorrhage");
    assert_eq!(darius.extras[0].formula.eval(&at(1.0)), Some(13.0 + 30.0));
    assert_eq!(
        darius.extras[2].formula.eval(&at(18.0)),
        Some(150.0 + 150.0)
    );
    assert!(
        darius.extras.iter().all(|e| e.issues.is_empty()),
        "{:?}",
        darius.extras
    );

    // Akali: two scalings after the value, then a colored keyword that ends it
    let akali = page(&data(
        "P",
        "|description3 = {{sbc|Swinging Kama:}} deal {{pplevel|35 to 53 for 7; then + 9*x for 6; then + 15*x|formula=+3 per level|color=magic damage}} {{as|(+ 60% '''bonus''' AD)}} {{as|(+ 55% AP)}} {{as|'''bonus''' magic damage}}.",
    ));
    assert_eq!(label(&akali, 0), "Swinging Kama");
    assert_eq!(
        akali.extras[0].formula.eval(&at(1.0)),
        Some(35.0 + 60.0 + 55.0)
    );

    // Vex: a field without a heading continues the previous one
    let vex = page(&data(
        "P",
        "|description2 = {{sbc|Innate - Gloom:}} Nearby enemies are marked.\n|description3 = ''Gloom's'' detonation deals {{pp|40 + (150-40)/17*(x-1) for 20|color=magic damage}} {{as|(+ 25% AP)}} {{ccs|{{as|'''bonus''' magic damage}}|magic}}.",
    ));
    assert_eq!(label(&vex, 0), "Innate - Gloom");
    assert_eq!(vex.extras[0].formula.eval(&at(1.0)), Some(40.0 + 25.0));

    // Aatrox: a percent of a stat, written inside `{{as}}`
    let aatrox = page(&data(
        "P",
        "|description = {{sbc|Innate:}} deal {{as|'''bonus''' magic damage}} equal to {{as|{{pplevel|key=%|4 to 10}} of the target's '''maximum''' health}}, capped at {{pplevel|100 to 320}} against monsters.",
    ));
    assert_eq!(label(&aatrox, 0), "Innate");
    let health = sample(&[(Level, 1.0), (EnemyMaxHealth, 1000.0)]);
    assert_eq!(aatrox.extras[0].formula.eval(&health), Some(40.0));
    assert_eq!(aatrox.extras[1].formula.eval(&health), Some(100.0));
}

#[test]
fn changes_of_a_key_without_effects_apply_to_its_description_values() {
    use CtxVar::*;
    // Akali's passive is only written in its description, and ARAM changes its AP ratio
    let mut akali = champion(
        "akali",
        &[(
            "P0",
            &data(
                "P",
                "|description3 = {{sbc|Swinging Kama:}} deal {{pplevel|35 to 182}} {{as|(+ 60% '''bonus''' AD)}} {{as|(+ 55% AP)}} {{as|'''bonus''' magic damage}}.",
            ),
        )],
    );
    let value = MapValue::Ratio {
        stat: AbilityPower,
        ratio: Formula::Const(0.65),
    };

    let placement = akali.apply_map_change(Key::P, None, "AP", MapMode::Aram, &value);
    assert!(matches!(placement, Placement::Applied(_)), "{placement:?}");

    let kama = &akali.abilities[&Key::P][0].extras[0].formula;
    let s = Sample::new([(Level, 1.0), (AbilityPower, 100.0)]);
    assert_eq!(kama.eval(&s), Some(35.0 + 55.0));
    assert_eq!(kama.eval(&s.on(MapMode::Aram)), Some(35.0 + 65.0));
}

#[test]
fn critical_damage_ratios_scale_with_the_critical_damage_stat() {
    use CtxVar::*;
    // Yasuo Q: a crit deals the critical damage of his AD. `ctx.crit_damage` already
    // has his modifier of 0.95, so the formula doesn't apply it again
    let yasuo = page(&data(
        "Q",
        "|leveling = {{st|Critical Strike Damage|{{ap|20 to 120}} {{as|(+ {{as|{{critical damage|200*(100/100)|100|mod=0.95}}|critical damage}} AD)|ad}}}}",
    ));
    let crit = &yasuo.effects[0];
    assert!(crit.issues.is_empty(), "{:?}", crit.issues);
    let s = sample(&[(QLevel, 1.0), (AttackDamage, 100.0), (CritDamage, 175.0)]);
    let value = crit.formula.eval(&s).unwrap();
    assert!((value - (20.0 + 1.75 * 100.0)).abs() < 1e-9, "{value}");
}

#[test]
fn a_change_of_damage_prefers_damage_effects_over_closer_ones() {
    use CtxVar::*;
    // Kassadin Q: Arena's base 100 to 210 is closer to the shield, but it is the damage
    let mut kassadin = champion(
        "kassadin",
        &[(
            "Q0",
            &data(
                "Q",
                "|leveling = {{st|Magic Damage|{{ap|65 to 185}} {{as|(+ 80% AP)}}}}\n|leveling2 = {{st|Magic Shield Strength|{{ap|80 to 200}} {{as|(+ 30% AP)}}}}",
            ),
        )],
    );
    let base = MapValue::Base(Formula::ByRank {
        rank: QLevel,
        values: vec![100.0, 127.5, 155.0, 182.5, 210.0],
    });

    let Placement::Applied(placed) =
        kassadin.apply_map_change(Key::Q, None, "Base", MapMode::Arena, &base)
    else {
        panic!()
    };
    assert_eq!(placed[0].label, "Magic Damage");

    let effects = &kassadin.abilities[&Key::Q][0].effects;
    let s = Sample::new([(QLevel, 1.0)]).on(MapMode::Arena);
    assert_eq!(effects[0].formula.eval(&s), Some(100.0));
    assert_eq!(effects[1].formula.eval(&s), Some(80.0));
}

#[test]
fn critical_damage_reads_its_share_and_leaves_other_forms_unread() {
    use CtxVar::*;
    let s = sample(&[(QLevel, 1.0), (AttackDamage, 100.0), (CritDamage, 190.0)]);

    // Yone Q: 105% of the critical damage
    let yone = page(&data(
        "Q",
        "|leveling = {{st|Critical Strike Damage|{{ap|25 to 125}} {{as|(+ {{as|{{critical damage|200*1.05|105|mod=0.95}}|critical damage}} AD)|ad}}}}",
    ));
    let value = yone.effects[0].formula.eval(&s).unwrap();
    assert!((value - (25.0 + 1.05 * 190.0)).abs() < 1e-9, "{value}");
}

#[test]
fn critical_scaling_shares_grow_with_chance_and_infinity_edge() {
    use CtxVar::*;
    // A real game, Caitlyn R at rank 1: 300 with no items, 0% chance and 200% critical
    // damage. Infinity Edge (75 AD, 25% chance, +30% critical damage) makes it 412:
    // (300 + 75) * (1 + 30% * 25% * (230 - 100) / 100)
    let caitlyn = page(&data(
        "R",
        "|description = She fires a bullet that deals {{as|physical damage}}, increased by {{critical damage|0 to 30|critScaling=true}}.\n|leveling = {{st|Physical damage|{{ap|300 to 650 3}} {{as|(+ 100% '''bonus''' AD)}}}}",
    ));
    let damage = &caitlyn.effects[0].formula;
    let increase = &caitlyn.extras[0].formula;
    assert!(
        caitlyn.extras[0].issues.is_empty(),
        "{:?}",
        caitlyn.extras[0].issues
    );

    let r = |s: &Sample| damage.eval(s).unwrap() * (1.0 + increase.eval(s).unwrap());
    let none = sample(&[(RLevel, 1.0), (CritChance, 0.0), (CritDamage, 200.0)]);
    let edge = sample(&[
        (RLevel, 1.0),
        (BonusAd, 75.0),
        (CritChance, 25.0),
        (CritDamage, 230.0),
    ]);
    assert_eq!(r(&none), 300.0);
    assert_eq!(r(&edge).round(), 412.0);

    // Caitlyn P: a ratio of AD by chance, 80% at 100% chance without Infinity Edge
    let headshot = page(&data(
        "P",
        "|leveling = {{st|Bonus Damage|{{pp|key=%|60 to 80 for 3|1 to 13}} {{as|(+ {{critical damage|0 to 80|critScaling=true}}) AD}}}}",
    ));
    let effect = &headshot.effects[0];
    assert!(effect.issues.is_empty(), "{:?}", effect.issues);
    let full = sample(&[
        (Level, 1.0),
        (AttackDamage, 100.0),
        (CritChance, 100.0),
        (CritDamage, 200.0),
    ]);
    let value = effect.formula.eval(&full).unwrap();
    assert!((value - (60.0 + 80.0)).abs() < 1e-9, "{value}");
}

#[test]
fn critical_scaling_counts_infinity_edge_from_the_champion_base() {
    use CtxVar::*;
    // A real game: Ashe's critical damage is 100 (200 * 0.5) and 130 with Infinity Edge
    // (+30%, 25% chance), and Frost Shot shows 133%: 25% * (100 + 100 * 30 / 100) = 32.5
    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: Default::default(),
        crit_damage: 100.0,
    };
    let src = data(
        "P",
        "|description = {{sbc|Innate:}} '''Ashe's''' {{tip|basic attacks}} deal {{as|'''bonus''' physical damage}} equal to {{critical damage|0 to 100|0 to 100|critScaling=true}} of the {{as|attack's damage}}.",
    );
    let ashe = AbilityModel::new(&src, &context).unwrap().unwrap().1;
    let frost = &ashe.extras[0];
    assert!(frost.issues.is_empty(), "{:?}", frost.issues);

    let none = sample(&[(CritChance, 0.0), (CritDamage, 100.0)]);
    let edge = sample(&[(CritChance, 25.0), (CritDamage, 130.0)]);
    assert_eq!(frost.formula.eval(&none), Some(0.0));
    let value = frost.formula.eval(&edge).unwrap();
    assert!((value - 0.325).abs() < 1e-9, "{value}");
}
