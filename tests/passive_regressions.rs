#![cfg(feature = "engine")]

use tutorlolv2::{
    AbilityId::P,
    AbilityName::{_1, Void},
    ChampionId,
};
use tutorlolv2_types::{AbilityLevels, Ctx};

#[test]
fn reparsed_passives_preserve_their_damage_components() {
    let ctx = Ctx {
        level: 18.0,
        ability_levels: AbilityLevels {
            w: 1,
            ..AbilityLevels::default()
        },
        attack_damage: 200.0,
        // No bonus attack damage
        base_ad: 200.0,
        ability_power: 100.0,
        // No base armor, so all of it is bonus armor
        armor: 100.0,
        enemy_max_health: 2000.0,
        stacks: 10.0,
        ..Default::default()
    };
    for (champion, ability, expected) in [
        // 30 (+ 6 per 5 Chimes, as stacks) (+ 40% AP)
        (ChampionId::Bard, P(Void), 82.0),
        (ChampionId::Lucian, P(Void), 120.0),
        (ChampionId::Lucian, P(_1), 35.0),
        (ChampionId::Taric, P(Void), 108.0),
        (ChampionId::Urgot, P(Void), 320.0),
        (ChampionId::Brand, P(_1), 280.0),
        // 15% AD, then the burn's 50 (+ 20% AP)
        (ChampionId::Milio, P(Void), 100.0),
        (
            ChampionId::Samira,
            tutorlolv2::AbilityId::P(tutorlolv2::AbilityName::Min),
            40.0,
        ),
        (ChampionId::Malzahar, tutorlolv2::AbilityId::W(Void), 96.5),
    ] {
        let actual = champion.eval(&ctx, ability);
        assert!(
            (actual - expected).abs() < 0.01,
            "{champion:?} {ability:?}: {actual} != {expected}"
        );
    }
    let vigilance = ChampionId::Lucian.index_of_ability(P(_1)).unwrap();
    assert_eq!(
        ChampionId::Lucian.metadata()[vigilance].damage_type,
        tutorlolv2::DamageType::Magic
    );
}
