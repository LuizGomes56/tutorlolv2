use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_1_0_0", Target::MINIMUM),
        (0, 0, "Q_2_0_0", Target::MINIMUM),
        (1, 0, "Q_2_1_0_MAXIMUM", Target::MAXIMUM),
        (1, 1, "Q_2_1_1_MAXIMUM", Target::MAXIMUM),
        (0, 0, "Q_3_0_0", Target::MINIMUM),
        (1, 0, "Q_3_1_0", Target::MINIMUM),
        (1, 1, "Q_3_1_1_MAXIMUM", Target::MAXIMUM),
        (1, 2, "Q_3_1_2_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        w,
        (0, 0, "W_3_0_0_BONUS", Target::MINIMUM),
        (0, 1, "W_3_0_1_MAXIMUM", Target::MAXIMUM),
        (1, 0, "W_3_1_0_BONUS", Target::MINIMUM)
    );
    ability!(
        e,
        (0, 0, "E_1_0_0", Target::MINIMUM),
        (0, 0, "E_2_0_0", Target::MINIMUM),
        (0, 0, "E_3_0_0", Target::MINIMUM)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM),
        (2, 0, "R_0_2_0", Target::MINIMUM),
        (2, 1, "R_0_2_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
