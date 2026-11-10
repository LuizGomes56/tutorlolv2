#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![cfg(feature = "engine")]

use strum::EnumCount;
use tutorlolv2::{
    ChampionId,
    model::DamageTable,
    model::{Modifiers, RangeDamage},
};

#[test]
fn named_batch_is_const_evaluable() {
    const N: usize = ChampionId::Neeko.number_of_abilities();
    const RESULT: ([i32; N], RangeDamage) = {
        let mut values = [0; N];
        let mut onhit = RangeDamage {
            minimum_damage: 0,
            maximum_damage: 0,
        };
        tutorlolv2::libgen::champions::neeko::neeko_batch(
            &tutorlolv2::test::CTX,
            tutorlolv2::test::MODIFIERS,
            &mut onhit,
            &mut values,
        );
        (values, onhit)
    };
    let mut values = [0; N];
    let mut onhit = RangeDamage::default();
    (ChampionId::Neeko.data().ability_batch)(
        &tutorlolv2::test::CTX,
        tutorlolv2::test::MODIFIERS,
        &mut onhit,
        &mut values,
    );
    assert_eq!(RESULT, (values, onhit));
}

#[test]
fn generated_batches_match_metadata_evaluation() {
    // A populated, valid context also exercises formulas whose own champion differs.
    let ctx = tutorlolv2::test::CTX;
    for i in 0..ChampionId::COUNT {
        let id = ChampionId::from_repr(i as _).unwrap();
        let data = id.data();
        for mut modifiers in [
            Modifiers::default(),
            Modifiers::new(&ctx, data.adaptive_type),
        ] {
            modifiers.damages.global_mod = 0.87;
            modifiers.damages.physical_mod = 0.61;
            modifiers.damages.magic_mod = 0.73;
            modifiers.damages.true_mod = 1.13;
            modifiers.abilities.q = 1.17;
            modifiers.abilities.w = 0.83;
            modifiers.abilities.e = 1.29;
            modifiers.abilities.r = 0.91;
            let mut expected = vec![0; data.metadata.len()];
            let mut actual = expected.clone();
            let mut expected_onhit = RangeDamage::default();
            let mut actual_onhit = RangeDamage::default();
            DamageTable::eval_abilities(
                &ctx,
                &mut expected_onhit,
                modifiers,
                data.metadata,
                data.closures,
                &mut expected,
            );
            (data.ability_batch)(&ctx, modifiers, &mut actual_onhit, &mut actual);
            assert_eq!(actual, expected, "{id:?}");
            assert_eq!(actual_onhit, expected_onhit, "{id:?}");
        }
    }
}
