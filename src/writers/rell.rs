use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        w,
        (1, 1, "W_0_1_1", Target::MINIMUM),
        (1, 0, "W_1_1_0_BONUS", Target::MINIMUM)
    );
    ability!(e, (1, 0, "E_0_1_0_BONUS", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
