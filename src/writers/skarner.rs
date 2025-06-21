use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
        (0, 1, "Q_0_0_1_BONUS", Target::MINIMUM),
        (1, 0, "Q_0_1_0_MONSTER", Target::MINIMUM),
        (0, 0, "Q_1_0_0", Target::MINIMUM),
        (0, 1, "Q_1_0_1_MONSTER", Target::MINIMUM)
    );
    ability!(w, (0, 0, "W_0_0_0", Target::MINIMUM));
    ability!(e, (2, 0, "E_0_2_0", Target::MINIMUM));
    ability!(r, (0, 0, "R_0_0_0", Target::MINIMUM));
}

#[writer_macros::test]
pub fn test() {}
