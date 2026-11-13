use tutorlolv2_build_dep::{
    formula as f,
    generators::parser::items::{Item, Source},
};
use tutorlolv2_types::{AttackType, CtxVar, DamageIndex, MapMode};
use tutorlolv2_wiki::{
    items::item_parser::{ItemEffect, WikiItem},
    parser::Effect,
    wikitext::{Formula, Sample},
};

#[test]
fn composed_item_damage_keeps_map_branches_until_assignment() {
    let mut item = Item::try_from(WikiItem::default()).unwrap();
    let damage = Formula::Const(20.0).with_map(MapMode::Arena, Formula::Const(30.0));
    let attack_damage = CtxVar::AttackDamage;
    let combined = f![damage + 0.5 * attack_damage];
    let sample = Sample::new([(CtxVar::AttackDamage, 100.0)]);
    assert_eq!(combined.eval(&sample), Some(70.0));
    assert_eq!(combined.eval(&sample.on(MapMode::Arena)), Some(80.0));
    item.set_min(combined.clone());
    assert_eq!(item.melee.min_dmg, combined.on(MapMode::Default).rust());
    assert_eq!(item.ranged.min_dmg, item.melee.min_dmg);
    assert_eq!(
        item.map_damages[0],
        vec![(MapMode::Arena, combined.on(MapMode::Arena).rust())]
    );
    assert_eq!(item.map_damages[2], item.map_damages[0]);
}

#[test]
fn item_queries_keep_distinct_melee_and_ranged_effects() {
    let effect = |damage| Effect {
        base: Some(vec![damage]),
        ..Effect::default()
    };
    let mut data = WikiItem::default();
    data.effects.pass = Some(ItemEffect {
        effect: effect(100.0),
        ranged: Some(effect(60.0)),
        ..ItemEffect::default()
    });
    let mut item = Item::try_from(data).unwrap();
    for attack_type in [AttackType::Melee, AttackType::Ranged] {
        let damage = item.formula_for(Source::Passive, attack_type).unwrap();
        item.assign(attack_type, DamageIndex::Min, f![damage * 2]);
    }
    assert_eq!(item.melee.min_dmg, f![100 * 2].rust());
    assert_eq!(item.ranged.min_dmg, f![60 * 2].rust());
}
