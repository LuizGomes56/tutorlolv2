use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_yunara(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM),
		(0, 3, "Q_0_0_3_BONUS", Target::MINIMUM),
		(0, 4, "Q_0_0_4_MAXIMUM", Target::MAXIMUM),
		(2, 0, "Q_0_2_0_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1", Target::MINIMUM),
		(0, 2, "W_0_0_2", Target::MINIMUM),
		(0, 3, "W_0_0_3_MAXIMUM", Target::MAXIMUM),
		(0, 4, "W_0_0_4_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}