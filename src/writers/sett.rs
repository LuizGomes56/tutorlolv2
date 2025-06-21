use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
        (0, 1, "Q_0_0_1_BONUS", Target::MINIMUM)
    );
    ability!(w, (2, 0, "W_0_2_0", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E_0_0_0", Target::MINIMUM),
        (0, 1, "E_0_0_1_MONSTER", Target::MINIMUM)
    );
    ability!(
        r,
        (1, 0, "R_0_1_0", Target::MINIMUM),
        (1, 1, "R_0_1_1", Target::MINIMUM)
    );
}

#[writer_macros::test]
pub fn test(data: Option<CdnChampion>) {}
