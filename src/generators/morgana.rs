use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_morgana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "W_0_0_1_MAXIMUM", Target::MAXIMUM),
		(0, 2, "W_0_0_2_MINIMUM", Target::MINIMUM),
		(0, 3, "W_0_0_3_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(0, 2, "R_0_0_2_MAXIMUM", Target::MAXIMUM)
	);
}