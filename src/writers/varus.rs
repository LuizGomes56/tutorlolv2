use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (2, 0, "Q_0_2_0_MINIMUM", Target::MINIMUM),
        (2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM),
        (2, 2, "Q_0_2_2_MINIMUM", Target::MINIMUM),
        (2, 3, "Q_0_2_3_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        w,
        (0, 0, "W_0_0_0_BONUS", Target::MINIMUM),
        (1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
        (1, 1, "W_0_1_1_BONUS", Target::MINIMUM),
        (1, 2, "W_0_1_2_BONUS", Target::MINIMUM),
        (1, 3, "W_0_1_3_BONUS", Target::MINIMUM),
        (2, 0, "W_0_2_0_MINIMUM", Target::MINIMUM),
        (2, 1, "W_0_2_1_MAXIMUM", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    ability!(r, (0, 0, "R_0_0_0", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test() {}
