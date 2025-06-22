use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, "Q_0_2_0_BONUS", Target::MINIMUM));
    ability!(
        w,
        (2, 0, "W_0_2_0", Target::MINIMUM),
        (2, 1, "W_0_2_1_MAXIMUM", Target::MAXIMUM),
        (2, 2, "W_0_2_2_MONSTER", Target::MINIMUM),
        (2, 3, "W_0_2_3_MONSTER", Target::MINIMUM),
        (2, 4, "W_0_2_4_MONSTER", Target::MINIMUM)
    );
    ability!(
        e,
        (1, 0, "E_0_1_0", Target::MINIMUM),
        (1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        r,
        (2, 0, "R_0_2_0", Target::MINIMUM),
        (3, 0, "R_0_3_0", Target::MINIMUM),
        (3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM)
    );
}
