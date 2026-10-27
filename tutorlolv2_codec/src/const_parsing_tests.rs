use crate::{EntityKind, Error, FormulaDb, GeneratorDb, ParseError};

// One champion with a formula encoding the constant 42.
const FORMULA_BYTES: [u8; 40] = {
    let mut bytes = [0; 40];
    bytes[0] = b'F';
    bytes[1] = b'B';
    bytes[2] = b'C';
    bytes[3] = b'1';
    bytes[4] = 2;
    bytes[6] = 24;
    bytes[12] = 1;
    bytes[18] = 1;
    bytes[34] = 2;
    bytes[39] = 42;
    bytes
};
const FORMULA: FormulaDb<'static> = match FormulaDb::new(&FORMULA_BYTES) {
    Ok(db) => db,
    Err(_) => panic!("invalid formula fixture"),
};
// One champion whose generator is present but empty.
const GENERATOR_BYTES: [u8; 43] = {
    let mut bytes = [0; 43];
    bytes[0] = b'G';
    bytes[1] = b'B';
    bytes[2] = b'C';
    bytes[3] = b'1';
    bytes[4] = 2;
    bytes[6] = 32;
    bytes[8] = 1;
    bytes[24] = 1;
    bytes[34] = 1;
    bytes
};
const GENERATOR: GeneratorDb<'static> = match GeneratorDb::new(&GENERATOR_BYTES) {
    Ok(db) => db,
    Err(_) => panic!("invalid generator fixture"),
};

#[test]
fn codec_lookups_accept_the_shared_entity_kind() {
    let kind: tutorlolv2_types::EntityKind = EntityKind::Champion;
    assert!(matches!(GENERATOR.has_generator(kind, 0), Ok(true)));
}

const _: () = {
    assert!(!core::mem::needs_drop::<ParseError>());
    assert!(FormulaDb::new(&[]).is_err());
    assert!(GeneratorDb::new(&[]).is_err());
    assert!(matches!(FORMULA.champion_formula_id(0, 0), Some(0)));
    assert!(matches!(FORMULA.formula_bytes(0), Some([0, 42])));
    assert!(FORMULA.u16_at(usize::MAX).is_err());
    assert!(FORMULA.u32_at(usize::MAX).is_err());
    assert!(matches!(
        GENERATOR.has_generator(EntityKind::Champion, 0),
        Ok(true)
    ));
    assert!(matches!(
        GENERATOR.entity_bytes(EntityKind::Champion, 0),
        Ok(Some([]))
    ));
    assert!(GENERATOR.has_generator(EntityKind::Champion, 1).is_err());

    let mut bytes = FORMULA_BYTES;
    bytes[0] = 0;
    assert!(FormulaDb::new(&bytes).is_err());
    bytes = FORMULA_BYTES;
    bytes[34] = 3; // Formula extends beyond the data section.
    assert!(FormulaDb::new(&bytes).is_err());
    bytes = FORMULA_BYTES;
    bytes[30] = 2;
    bytes[34] = 1; // Descending offsets.
    assert!(FormulaDb::new(&bytes).is_err());

    let mut bytes = GENERATOR_BYTES;
    bytes[5] = 128; // Unknown flags.
    assert!(GeneratorDb::new(&bytes).is_err());
    bytes = GENERATOR_BYTES;
    bytes[34] = 0; // Header claims a present owner, bitmap doesn't.
    assert!(GeneratorDb::new(&bytes).is_err());
    bytes = GENERATOR_BYTES;
    bytes[34] = 3; // Padding bits must be zero.
    assert!(GeneratorDb::new(&bytes).is_err());
    bytes = GENERATOR_BYTES;
    bytes[37] = 1; // Owner offset exceeds the empty stream.
    assert!(GeneratorDb::new(&bytes).is_err());
};

#[test]
fn const_databases_match_runtime() {
    let runtime = FormulaDb::new(&FORMULA_BYTES).unwrap();
    assert_eq!(FORMULA.formula_bytes(0), runtime.formula_bytes(0));
    let runtime = GeneratorDb::new(&GENERATOR_BYTES).unwrap();
    assert_eq!(GENERATOR.champion_count(), runtime.champion_count());
    assert_eq!(
        GENERATOR.entity_bytes(EntityKind::Champion, 0).unwrap(),
        runtime.entity_bytes(EntityKind::Champion, 0).unwrap()
    );
}

#[test]
fn truncated_databases_are_rejected() {
    for len in 0..FORMULA_BYTES.len() {
        assert!(
            FormulaDb::new(&FORMULA_BYTES[..len]).is_err(),
            "formula length {len}"
        );
    }
    for len in 0..GENERATOR_BYTES.len() {
        assert!(
            GeneratorDb::new(&GENERATOR_BYTES[..len]).is_err(),
            "generator length {len}"
        );
    }
}

#[test]
fn parse_error_converts_to_runtime_error() {
    let error: Error = ParseError::Corrupt("test").into();
    assert!(matches!(error, Error::Corrupt("test")));
}
