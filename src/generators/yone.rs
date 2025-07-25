use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_yone(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1", Target::MINIMUM),
		(0, 2, "W_0_0_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(3, 0, "E_0_3_0", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM),
		(1, 2, "R_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
}