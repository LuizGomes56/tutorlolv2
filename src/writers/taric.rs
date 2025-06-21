use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
