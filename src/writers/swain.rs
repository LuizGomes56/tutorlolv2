use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Target::MINIMUM),
        (0, 1, "Q_0_0_1_BONUS", Target::MINIMUM),
        (0, 2, "Q_0_0_2_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        w,
        (0, 0, "W_0_0_0", Target::MINIMUM),
        (0, 1, "W_0_0_1", Target::MINIMUM)
    );
    ability!(e, (1, 0, "E_0_1_0", Target::MINIMUM));
    ability!(
        r,
        (1, 0, "R_0_1_0", Target::MINIMUM),
        (0, 0, "R_1_0_0", Target::MINIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
