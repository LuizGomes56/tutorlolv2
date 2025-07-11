use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_akshan(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Target::MINIMUM),
        (1, 0, "Q_0_1_0_MAXIMUM", Target::MAXIMUM),
        (2, 0, "Q_0_2_0", Target::MINIMUM)
    );
    // ability!(
    // 	e,
    // 	(2, 0, "E_0_2_0", Target::MINIMUM)
    // );
    // ability!(
    // 	r,
    // 	(2, 0, "R_0_2_0_MINIMUM", Target::MINIMUM),
    // 	(2, 1, "R_0_2_1_MAXIMUM", Target::MAXIMUM),
    // 	(2, 2, "R_0_2_2_MINIMUM", Target::MINIMUM),
    // 	(2, 3, "R_0_2_3", Target::MINIMUM)
    // );
}
