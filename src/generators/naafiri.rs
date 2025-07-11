use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_naafiri(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MAXIMUM", Target::MAXIMUM),
		(0, 3, "Q_0_0_3_MAXIMUM", Target::MAXIMUM),
		(1, 0, "Q_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "Q_0_1_1_BONUS", Target::MINIMUM),
		(1, 2, "Q_0_1_2_MAXIMUM", Target::MAXIMUM),
		(1, 3, "Q_0_1_3_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM),
		(0, 2, "E_0_0_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1", Target::MINIMUM)
	);
}