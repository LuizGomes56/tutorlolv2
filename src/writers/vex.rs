use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(w, (0, 1, "W_0_0_1", Target::MINIMUM));
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (2, 0, "R_0_2_0", Target::MINIMUM),
        (2, 1, "R_0_2_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
