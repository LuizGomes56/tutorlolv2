use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_evelynn(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0_BONUS", Target::MINIMUM),
		(2, 1, "Q_0_2_1_BONUS", Target::MINIMUM),
		(5, 0, "Q_0_5_0", Target::MINIMUM),
		(5, 1, "Q_0_5_1_MAXIMUM", Target::MAXIMUM),
		(5, 2, "Q_0_5_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 0, "E_1_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}