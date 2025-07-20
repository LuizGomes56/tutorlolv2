use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_aatrox(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1", Target::MINIMUM),
		(3, 0, "Q_0_3_0", Target::MINIMUM),
		(3, 1, "Q_0_3_1", Target::MINIMUM),
		(4, 0, "Q_0_4_0_MAXIMUM", Target::MAXIMUM),
		(4, 1, "Q_0_4_1_MAXIMUM", Target::MAXIMUM),
		(5, 0, "Q_0_5_0", Target::MINIMUM),
		(5, 1, "Q_0_5_1", Target::MINIMUM),
		(2, 0, "Q_1_2_0", Target::MINIMUM),
		(2, 1, "Q_1_2_1", Target::MINIMUM),
		(3, 0, "Q_1_3_0", Target::MINIMUM),
		(3, 1, "Q_1_3_1", Target::MINIMUM),
		(4, 0, "Q_1_4_0_MAXIMUM", Target::MAXIMUM),
		(4, 1, "Q_1_4_1_MAXIMUM", Target::MAXIMUM),
		(5, 0, "Q_1_5_0", Target::MINIMUM),
		(5, 1, "Q_1_5_1", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1", Target::MINIMUM),
		(1, 0, "W_0_1_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_BONUS", Target::MINIMUM)
	);
}