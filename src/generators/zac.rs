use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_zac(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1", Target::MINIMUM)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1_MAXIMUM", Target::MAXIMUM),
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}