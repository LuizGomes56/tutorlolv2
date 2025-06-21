use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, "Q_0_1_0", Target::MINIMUM),
        (1, 1, "Q_0_1_1", Target::MINIMUM),
        (1, 2, "Q_0_1_2", Target::MINIMUM),
        (1, 3, "Q_0_1_3", Target::MINIMUM),
        (2, 0, "Q_0_2_0_MINIMUM", Target::MINIMUM),
        (2, 1, "Q_0_2_1_MINIMUM", Target::MINIMUM),
        (2, 2, "Q_0_2_2_MAXIMUM", Target::MAXIMUM),
        (2, 3, "Q_0_2_3_MAXIMUM", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E_0_0_0_BONUS", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1", Target::MINIMUM),
        (2, 0, "R_0_2_0_MAXIMUM", Target::MAXIMUM),
        (2, 1, "R_0_2_1_MAXIMUM", Target::MAXIMUM),
        (2, 2, "R_0_2_2_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test() {}
