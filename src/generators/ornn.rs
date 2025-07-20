use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ornn(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_MINIMUM", Target::MINIMUM),
		(1, 1, "W_0_1_1_MONSTER", Target::MINIMUM),
		(1, 2, "W_0_1_2_MAXIMUM", Target::MAXIMUM),
		(1, 3, "W_0_1_3_MONSTER", Target::MINIMUM),
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(2, 1, "W_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(3, 0, "R_0_3_0_MAXIMUM", Target::MAXIMUM)
	);
}