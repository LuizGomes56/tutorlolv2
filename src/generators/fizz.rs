use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_fizz(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_BONUS", Target::MINIMUM),
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(2, 1, "W_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(0, 0, "E_1_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(3, 0, "R_0_3_0", Target::MINIMUM),
		(4, 0, "R_0_4_0", Target::MINIMUM)
	);
}