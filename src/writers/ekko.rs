use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Target::MINIMUM),
        (2, 0, "Q_0_2_0", Target::MINIMUM),
        (2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E_0_0_0_BONUS", Target::MINIMUM));
    ability!(r, (1, 0, "R_0_1_0", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
