#![cfg(feature = "bake")]

use databake::{Bake, CrateEnv};
use types::*;

#[test]
fn nested_metadata_bakes_as_a_const_expression_with_qualified_paths() {
    const VALUE: TypeMetadata<AbilityId> = TypeMetadata {
        kind: AbilityId::Q(AbilityName::Min),
        damage_type: DamageType::Physical,
        attributes: Attrs::Undefined,
    };
    let expected = databake::quote!(types::TypeMetadata {
        kind: types::AbilityId::Q(types::AbilityName::Min,),
        damage_type: types::DamageType::Physical,
        attributes: types::Attrs::Undefined,
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
