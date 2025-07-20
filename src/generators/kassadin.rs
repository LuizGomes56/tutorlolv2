use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_kassadin(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(1, 0, "R_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "R_0_1_1_BONUS", Target::MINIMUM),
		(1, 2, "R_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
}