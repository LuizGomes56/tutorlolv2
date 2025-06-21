use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E_0_0_0", Target::MINIMUM),
        (0, 1, "E_0_0_1", Target::MINIMUM),
        (0, 2, "E_0_0_2_MAXIMUM", Target::MAXIMUM),
        (1, 0, "E_0_1_0_MONSTER", Target::MINIMUM),
        (1, 1, "E_0_1_1_MONSTER", Target::MINIMUM),
        (1, 2, "E_0_1_2_MONSTER", Target::MINIMUM)
    );
    ability!(
        r,
        (3, 0, "R_0_3_0", Target::MINIMUM),
        (3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
