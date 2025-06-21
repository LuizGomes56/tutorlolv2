use super::*;

// #![stable] "06/18/2025" | "25.11"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E", Target::MINIMUM),
        (2, 0, "E_MAX", Target::MAXIMUM)
    );
    ability!(r, (0, 0, "R", Target::MINIMUM));
    merge_ability!("E");
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
