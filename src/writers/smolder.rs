use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Target::MINIMUM),
        (0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM),
        (0, 2, "Q_0_0_2_MAXIMUM", Target::MAXIMUM),
        (3, 0, "Q_0_3_0", Target::MINIMUM),
        (3, 1, "Q_0_3_1_MAXIMUM", Target::MAXIMUM),
        (3, 2, "Q_0_3_2_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        w,
        (0, 0, "W_0_0_0", Target::MINIMUM),
        (0, 1, "W_0_0_1", Target::MINIMUM),
        (0, 2, "W_0_0_2_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        e,
        (0, 0, "E_0_0_0", Target::MINIMUM),
        (0, 1, "E_0_0_1_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        r,
        (0, 1, "R_0_0_1", Target::MINIMUM),
        (0, 2, "R_0_0_2_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
