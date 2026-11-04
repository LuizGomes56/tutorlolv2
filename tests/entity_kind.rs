use strum::VariantArray;
use tutorlolv2::{CastId, ChampionId, EntityKind, ItemId, RuneId};

#[test]
fn cast_id_default_kind_classifies_every_variant() {
    fn check<T: CastId + VariantArray>(expected: EntityKind) {
        for _ in T::VARIANTS {
            assert_eq!(T::KIND, expected);
        }
    }
    check::<ChampionId>(EntityKind::Champion);
    check::<ItemId>(EntityKind::Item);
    check::<RuneId>(EntityKind::Rune);
}
