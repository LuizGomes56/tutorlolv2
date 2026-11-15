#![cfg(feature = "bake")]

use databake::{Bake, CrateEnv};
use types::*;

#[test]
fn nested_metadata_bakes_as_a_const_expression_with_qualified_paths() {
    const VALUE: TypeMetadata<AbilityId> = TypeMetadata {
        kind: AbilityId::Q(AbilityName::Min),
        damage_type: DamageType::Physical,
        attributes: Attrs::NONE,
    };
    let expected = databake::quote!(types::TypeMetadata {
        kind: types::AbilityId::Q(types::AbilityName::Min,),
        damage_type: types::DamageType::Physical,
        attributes: types::Attrs::from_bits(0u8),
    });
    assert_eq!(
        VALUE.bake(&CrateEnv::default()).to_string(),
        expected.to_string()
    );
}

#[test]
fn identifier_slice_bakes_values_without_extra_references() {
    const VALUES: &[CtxVar] = &[CtxVar::AbilityPower, CtxVar::Level];
    let code = VALUES.bake(&CrateEnv::default());
    let expected = databake::quote!(&[types::CtxVar::AbilityPower, types::CtxVar::Level]);
    assert_eq!(code.to_string(), expected.to_string());
}

#[test]
fn attributes_pack_trigger_area_and_role_in_one_byte() {
    assert_eq!(size_of::<Attrs>(), 1);
    assert_eq!(size_of::<TypeMetadata<AbilityId>>(), 4);

    for trigger in [Trigger::Direct, Trigger::EveryHit, Trigger::Charged] {
        for area in [false, true] {
            for role in [AttackRole::Standalone, AttackRole::IsAttack] {
                let attrs = Attrs::new(trigger, area, role);
                assert_eq!(
                    (attrs.trigger(), attrs.is_area(), attrs.role()),
                    (trigger, area, role)
                );
                assert_eq!(Attrs::from_bits(attrs.bits()), attrs);
            }
        }
    }

    let spellblade = Attrs::from(Trigger::Spellblade);
    assert!(spellblade.onhit_max() && !spellblade.onhit_min());
    assert!(Attrs::from(Trigger::EveryHit).doubled_by_seething_strike());
    assert!(!Attrs::NONE.onhit_max() && !Attrs::NONE.implies_attack());
}
