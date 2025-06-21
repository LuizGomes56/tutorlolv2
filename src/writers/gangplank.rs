use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(e, (3, 1, "E_0_3_1_BONUS", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1", Target::MINIMUM),
        (0, 2, "R_0_0_2_MAXIMUM", Target::MAXIMUM),
        (2, 0, "R_0_2_0", Target::MINIMUM),
        (2, 1, "R_0_2_1_MAXIMUM", Target::MAXIMUM),
        (3, 0, "R_0_3_0_MAXIMUM", Target::MAXIMUM),
        (3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test() {}
