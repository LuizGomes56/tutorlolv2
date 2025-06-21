use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, "Q_0_1_0_MINIMUM", Target::MINIMUM),
        (1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM),
        (0, 2, "R_0_0_2", Target::MINIMUM),
        (0, 3, "R_0_0_3_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test() {}
