use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_fiddlesticks(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MINIMUM", Target::MINIMUM),
		(2, 0, "Q_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(4, 0, "W_0_4_0", Target::MINIMUM),
		(4, 1, "W_0_4_1", Target::MINIMUM),
		(4, 2, "W_0_4_2", Target::MINIMUM),
		(4, 3, "W_0_4_3_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
}