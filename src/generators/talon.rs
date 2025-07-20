use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_talon(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 2, "W_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(1, 0, "R_0_1_0_MAXIMUM", Target::MAXIMUM)
	);
}