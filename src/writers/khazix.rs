use super::*;

// #![stable] "06/19/2025" | "25.12"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, "Q", Target::MINIMUM),
        (1, 1, "Q_MAX", Target::MAXIMUM)
    );
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
    merge_ability!("Q");
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
