use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_riven(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_1_0_0_MINIMUM", Target::MINIMUM),
		(0, 1, "R_1_0_1_MAXIMUM", Target::MAXIMUM)
	);
}