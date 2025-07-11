use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_mel(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Target::MINIMUM),
        (0, 1, "Q_0_0_1", Target::MINIMUM),
        (0, 3, "Q_0_0_3_MAXIMUM", Target::MAXIMUM),
        (0, 4, "Q_0_0_4_MAXIMUM", Target::MAXIMUM)
    );
    ability!(w, (1, 0, "W_0_1_0", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E_0_0_0", Target::MINIMUM),
        (1, 0, "E_0_1_0", Target::MINIMUM),
        (1, 1, "E_0_1_1", Target::MINIMUM),
        (2, 0, "E_0_2_0", Target::MINIMUM),
        (2, 1, "E_0_2_1", Target::MINIMUM),
        (2, 2, "E_0_2_2", Target::MINIMUM)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0_MAXIMUM", Target::MAXIMUM),
        (0, 1, "R_0_0_1", Target::MINIMUM),
        // (1, 0, "R_0_1_0", Target::MINIMUM)
    );
}
