use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ornn(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        w,
        (1, 0, "W_0_1_0_MAXIMUM", Target::MAXIMUM),
        (1, 1, "W_0_1_1", Target::MINIMUM),
        (2, 0, "W_0_2_0_MAXIMUM", Target::MAXIMUM),
        (2, 1, "W_0_2_1_MINIMUM", Target::MINIMUM),
        (2, 2, "W_0_2_2_MONSTER", Target::MINIMUM),
        (2, 3, "W_0_2_3_MONSTER", Target::MINIMUM)
    );
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (2, 0, "R_0_2_0_MAXIMUM", Target::MAXIMUM)
    );
}
