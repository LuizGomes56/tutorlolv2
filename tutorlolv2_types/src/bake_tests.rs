use crate::{AbilityId, TypeMetadata};
use std::string::ToString;

#[test]
fn nested_metadata_bakes_as_a_const_expression_with_qualified_paths() {
    databake::test_bake!(
        TypeMetadata<AbilityId>,
        const,
        crate::TypeMetadata {
            kind: crate::AbilityId::Q(crate::AbilityName::Min),
            damage_type: crate::DamageType::Physical,
            attributes: crate::Attrs::Undefined,
        },
        tutorlolv2_types,
    );
}

#[test]
fn identifier_slice_bakes_values_without_extra_references() {
    use databake::{Bake, CrateEnv};

    const VALUES: &[crate::CtxVar] = &[crate::CtxVar::AbilityPower, crate::CtxVar::Level];
    let code = VALUES.bake(&CrateEnv::default());
    let expected = databake::quote!(&[
        tutorlolv2_types::CtxVar::AbilityPower,
        tutorlolv2_types::CtxVar::Level
    ]);
    assert_eq!(code.to_string(), expected.to_string());
}
