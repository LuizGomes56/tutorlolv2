use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_shaco(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_MONSTER", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAXIMUM", Target::MAXIMUM),
		(1, 2, "W_0_1_2_MONSTER", Target::MINIMUM),
		(1, 3, "W_0_1_3", Target::MINIMUM),
		(1, 4, "W_0_1_4_MONSTER", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0_MAXIMUM", Target::MAXIMUM),
		(3, 1, "R_0_3_1", Target::MINIMUM),
		(4, 0, "R_0_4_0", Target::MINIMUM)
	);
}