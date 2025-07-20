use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_gwen(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2", Target::MINIMUM),
		(0, 3, "Q_0_0_3", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MAXIMUM", Target::MAXIMUM),
		(1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM),
		(1, 2, "Q_0_1_2_MINIMUM", Target::MINIMUM),
		(1, 3, "Q_0_1_3_MINIMUM", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(3, 0, "R_0_3_0_MAXIMUM", Target::MAXIMUM),
		(3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM),
		(3, 2, "R_0_3_2_MAXIMUM", Target::MAXIMUM)
	);
}