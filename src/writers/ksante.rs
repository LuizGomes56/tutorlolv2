use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        w,
        (2, 0, "W_0_2_0", Target::MINIMUM),
        (2, 1, "W_0_2_1_MONSTER", Target::MINIMUM),
        (3, 0, "W_0_3_0_BONUS", Target::MINIMUM),
        (3, 1, "W_0_3_1_BONUS", Target::MINIMUM),
        (3, 2, "W_0_3_2_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (1, 0, "R_0_1_0", Target::MINIMUM),
        (1, 1, "R_0_1_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
