use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_morgana(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        w,
        (1, 0, "W_0_1_0_MINIMUM", Target::MINIMUM),
        (1, 1, "W_0_1_1_MAXIMUM", Target::MAXIMUM),
        (1, 2, "W_0_1_2_MAXIMUM", Target::MAXIMUM),
        (1, 3, "W_0_1_3_MAXIMUM", Target::MAXIMUM)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM)
    );
}
