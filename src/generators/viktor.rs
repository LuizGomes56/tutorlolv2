use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_viktor(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(5, 0, "R_0_5_0", Target::MINIMUM),
		(5, 1, "R_0_5_1_MAXIMUM", Target::MAXIMUM)
	);
}