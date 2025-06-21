use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
        (0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM),
        (1, 1, "Q_0_1_1_BONUS", Target::MINIMUM)
    );
    ability!(
        r,
        (1, 0, "R_0_1_0", Target::MINIMUM),
        (1, 1, "R_0_1_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test() {}
