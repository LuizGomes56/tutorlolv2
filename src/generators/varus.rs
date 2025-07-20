use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_varus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM),
		(2, 2, "Q_0_2_2_MINIMUM", Target::MINIMUM),
		(2, 3, "Q_0_2_3_MINIMUM", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "W_0_0_1_MINIMUM", Target::MINIMUM),
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "W_0_1_1_BONUS", Target::MINIMUM),
		(1, 2, "W_0_1_2_BONUS", Target::MINIMUM),
		(1, 3, "W_0_1_3_BONUS", Target::MINIMUM),
		(4, 0, "W_0_4_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}