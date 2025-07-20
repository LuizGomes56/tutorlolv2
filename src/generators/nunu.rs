use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_nunu(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 1, "Q_0_1_1", Target::MINIMUM),
		(2, 2, "Q_0_2_2", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "W_0_2_1_MINIMUM", Target::MINIMUM),
		(4, 0, "W_0_4_0_MAXIMUM", Target::MAXIMUM),
		(4, 1, "W_0_4_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MAXIMUM", Target::MAXIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(3, 0, "E_0_3_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}