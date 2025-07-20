use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_galio(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1", Target::MINIMUM),
		(2, 0, "W_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "W_0_2_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}