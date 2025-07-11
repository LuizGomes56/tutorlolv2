use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_jinx(data: CdnChampion) -> Champion {
    ability!(w, (0, 0, "W_0_0_0", Target::MINIMUM));
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    // ability!(
    // 	r,
    // 	(1, 0, "R_0_1_0_MAXIMUM", Target::MAXIMUM),
    // 	(1, 1, "R_0_1_1_MINIMUM", Target::MINIMUM),
    // 	(2, 0, "R_0_2_0_MAXIMUM", Target::MAXIMUM),
    // 	(2, 1, "R_0_2_1_MINIMUM", Target::MINIMUM)
    // );
}
