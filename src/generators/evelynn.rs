use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_evelynn(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "Q_0_1_1_BONUS", Target::MINIMUM),
		(3, 0, "Q_0_3_0", Target::MINIMUM),
		(3, 1, "Q_0_3_1_MAXIMUM", Target::MAXIMUM),
		(3, 2, "Q_0_3_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(4, 0, "W_0_4_0_BONUS", Target::MINIMUM)
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