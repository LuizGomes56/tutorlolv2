//! Run with `cargo run -p tutorlolv2_build_dep --example native_generator_audit`.
//! Reads cached raw source; never rewrites production generated files.
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use tutorlolv2_build_dep::{
    MayFail,
    generators::parser::{Parser, items::ItemParser, runes::RuneParser},
};
use tutorlolv2_types::CtxVar;
use tutorlolv2_wiki::wikitext::{
    LegacyFormula, Lookup, PageContext, Sample, SourceDocument, SourceForm, SourceRole,
    SourceVariant,
};

fn main() -> MayFail {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let lookup = Lookup::from_cache_at(root)?;
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: Default::default(),
        crit_damage: 200.0,
    };

    let items = ItemParser::from_root(root)?;
    let runes = RuneParser::from_root(root)?;
    let module = std::fs::read_to_string(root.join("cache/wiki/items/data.lua"))?;
    let sample = Sample::new([
        (CtxVar::Level, 18.0),
        (CtxVar::Stacks, 10.0),
        (CtxVar::BonusAd, 100.0),
        (CtxVar::BaseAd, 100.0),
        (CtxVar::AbilityPower, 200.0),
        (CtxVar::MaxHealth, 3000.0),
        (CtxVar::BonusHealth, 1000.0),
        (CtxVar::EnemyMaxHealth, 2000.0),
        (CtxVar::CritChance, 25.0),
    ]);

    let mut report = Vec::new();

    for ranged in [false, true] {
        for alternate in [false, true] {
            let variant = SourceVariant { ranged, alternate };

            for (id, name) in [
                ("EssenceReaver", "Essence Reaver"),
                ("BamisCinder", "Bami's Cinder"),
                ("LiandrysTorment", "Liandry's Torment"),
                ("Heartsteel", "Heartsteel"),
                ("OverlordsBloodmail", "Overlord's Bloodmail"),
                ("HollowRadiance", "Hollow Radiance"),
                ("TitanicHydra", "Titanic Hydra"),
            ] {
                let current = items.run_fn(id).and_then(|v| Ok(serde_json::to_value(v)?));

                report.push(audit(
                    "item",
                    id,
                    variant,
                    SourceDocument::item_effects(&module, name, &context, variant),
                    current,
                    &sample,
                ));
            }

            if alternate {
                continue;
            }

            for id in [
                "ArcaneComet",
                "DarkHarvest",
                "GraspOfTheUndying",
                "LethalTempo",
            ] {
                let raw =
                    std::fs::read_to_string(root.join(format!("cache/wiki/runes/{id}/data.lua")))?;

                let current = runes.run_fn(id).and_then(|v| Ok(serde_json::to_value(v)?));

                report.push(audit(
                    "rune",
                    id,
                    variant,
                    SourceDocument::rune_descriptions(&raw, &context, variant),
                    current,
                    &sample,
                ));
            }
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "sample": "level18 stacks10 bonusAD100 baseAD100 AP200 HP3000 bonusHP1000 targetHP2000 crit25",
            "scope": "Default map; raw values and diagnostic comparisons, not production dispatch",
            "results": report
        }))?
    );

    Ok(())
}

fn audit(
    kind: &str,
    id: &str,
    variant: SourceVariant,
    documents: MayFail<BTreeMap<String, SourceDocument>>,
    current: MayFail<Value>,
    sample: &Sample,
) -> Value {
    let current = current.map_err(|e| e.to_string());
    let attack = if variant.ranged { "ranged" } else { "melee" };
    let mut comparisons = Vec::new();

    if let Ok(documents) = &documents {
        for (name, document) in documents {
            for form in [SourceForm::Base, SourceForm::Maximum, SourceForm::Total] {
                let selected = document
                    .select(SourceRole::Damage, form)
                    .and_then(|value| value.formula());

                let native = selected
                    .as_ref()
                    .ok()
                    .and_then(|formula| formula.eval(sample));

                let field = if form == SourceForm::Base {
                    "min_dmg"
                } else {
                    "max_dmg"
                };

                let old = current
                    .as_ref()
                    .ok()
                    .and_then(|value| value.get(attack))
                    .and_then(|value| value.get(field))
                    .and_then(Value::as_str);

                let old_value = old
                    .filter(|_| !variant.alternate)
                    .and_then(LegacyFormula::parse)
                    .map(|formula| formula.eval(sample));

                comparisons.push(json!({
                    "effect": name, "form": form, "native": native,
                    "current": old_value, "current_formula": old,
                    "difference": native.zip(old_value).map(|(a,b)|a-b),
                    "selection_error": selected.err(),
                }));
            }
        }
    }

    json!({
        "kind": kind,
        "id": id,
        "variant": variant,
        "documents": documents.as_ref().ok(),
        "source_error": documents.as_ref().err().map(|e| e.to_string()),
        "current_error": current.err(),
        "comparisons": comparisons
    })
}
