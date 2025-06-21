use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        w,
        (1, 2, "W_0_1_2", Target::MINIMUM),
        (1, 3, "W_0_1_3_MINIMUM", Target::MINIMUM)
    );
    ability!(
        e,
        (0, 0, "E_0_0_0_BONUS", Target::MINIMUM),
        (0, 1, "E_0_0_1_BONUS", Target::MINIMUM)
    );
    ability!(r, (0, 0, "R_0_0_0", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
