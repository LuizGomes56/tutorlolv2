use core::mem::size_of;
use types::{AttackSpeedRatio, Ctx, CtxAttacker, CtxTarget, CtxVar};

#[test]
fn compact_ratio_preserves_context_cache_line_budget_and_attacker_state() {
    assert_eq!(size_of::<AttackSpeedRatio>(), 2);
    assert_eq!(size_of::<Ctx>(), 128);
    assert_eq!(size_of::<CtxAttacker>(), 100);
    assert_eq!(size_of::<CtxTarget>(), 32);
    let attacker = CtxAttacker {
        bonus_attack_speed_ratio: AttackSpeedRatio::new(1.5),
        attack_speed: 2.0,
        base_attack_speed: 0.625,
        ..Default::default()
    };
    let ctx = Ctx::new(&attacker, &CtxTarget::default());
    assert_eq!(ctx.get(CtxVar::BonusAttackSpeedRatio), 1.5);
    assert_eq!(ctx.bonus_attack_speed(), 1.375);
    assert_eq!(ctx.attacker(), attacker);
    assert_eq!(Ctx::default().bonus_attack_speed_ratio(), 0.0);
    assert!(CtxVar::BonusAttackSpeedRatio.is_method());
    assert_eq!(
        CtxVar::BonusAttackSpeedRatio.as_var(),
        "ctx.bonus_attack_speed_ratio"
    );
    assert_eq!(
        CtxVar::BonusAttackSpeedRatio as u8,
        CtxVar::WLevel as u8 + 1
    );
}

#[allow(clippy::excessive_precision)]
#[test]
fn half_precision_rounds_ties_to_even_and_handles_boundaries() {
    const RATIO: AttackSpeedRatio = AttackSpeedRatio::new(1.5);
    const VALUE: f32 = RATIO.get();
    assert_eq!(VALUE, 1.5);
    for (input, expected) in [
        (0.0, 0.0),
        (-0.0, -0.0),
        (1.0, 1.0),
        (-2.0, -2.0),
        (1.00048828125, 1.0),
        (1.00146484375, 1.001953125),
        (-1.00048828125, -1.0),
        (0.0000000298023223876953125, 0.0),
        (0.000000059604644775390625, 0.000000059604644775390625),
        (0.0000000894069671630859375, 0.00000011920928955078125),
        (0.0000610053539276123046875, 0.00006103515625),
        (65504.0, 65504.0),
        (65520.0, f32::INFINITY),
        (f32::INFINITY, f32::INFINITY),
        (f32::NEG_INFINITY, f32::NEG_INFINITY),
        (f32::MIN_POSITIVE, 0.0),
    ] {
        assert_eq!(
            AttackSpeedRatio::new(input).get().to_bits(),
            expected.to_bits(),
            "{input}"
        );
    }
    assert!(AttackSpeedRatio::new(f32::NAN).get().is_nan());
    assert_eq!(AttackSpeedRatio::default().get(), 0.0);
}

#[test]
fn every_binary16_value_decodes_and_finite_values_round_trip() {
    // Decode fixed-width storage to independently exercise all representations,
    // including subnormals and both signs, rather than just sampled game values.
    let config = bincode::config::standard().with_fixed_int_encoding();
    for bits in 0..=u16::MAX {
        let (ratio, used): (AttackSpeedRatio, _) =
            bincode::decode_from_slice(&bits.to_le_bytes(), config).unwrap();
        assert_eq!(used, 2);
        let exponent = (bits >> 10) & 31;
        let fraction = bits & 1023;
        if exponent == 31 && fraction != 0 {
            assert!(ratio.get().is_nan());
            continue;
        }
        let magnitude = if exponent == 31 {
            f32::INFINITY
        } else if exponent == 0 {
            fraction as f32 * 2.0_f32.powi(-24)
        } else {
            (1.0 + fraction as f32 / 1024.0) * 2.0_f32.powi(exponent as i32 - 15)
        };
        let expected = if bits & 0x8000 != 0 {
            -magnitude
        } else {
            magnitude
        };
        assert_eq!(
            ratio.get().to_bits(),
            expected.to_bits(),
            "half bits {bits:04x}"
        );
        assert_eq!(
            AttackSpeedRatio::new(expected),
            ratio,
            "half bits {bits:04x}"
        );
        let mut encoded = [0; 2];
        assert_eq!(
            bincode::encode_into_slice(ratio, &mut encoded, config).unwrap(),
            2
        );
        assert_eq!(encoded, bits.to_le_bytes());
    }
}

#[test]
fn serde_accepts_a_semantic_ratio_instead_of_binary16_bits() {
    use serde::{
        Deserialize,
        de::value::{Error, F32Deserializer},
    };
    let value = AttackSpeedRatio::deserialize(F32Deserializer::<Error>::new(1.5)).unwrap();
    assert_eq!(value.get(), 1.5);
}
