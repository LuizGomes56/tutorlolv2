use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ahri(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1", Target::MINIMUM),
		(1, 2, "W_0_1_2_MAXIMUM", Target::MAXIMUM),
		(3, 0, "W_0_3_0_MAXIMUM", Target::MAXIMUM),
		(3, 1, "W_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}