use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_teemo(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM),
		(0, 2, "E_0_0_2_MAXIMUM", Target::MAXIMUM),
		(1, 0, "E_0_1_0_MONSTER", Target::MINIMUM),
		(1, 1, "E_0_1_1_MONSTER", Target::MINIMUM),
		(1, 2, "E_0_1_2_MONSTER", Target::MINIMUM)
	);
	ability!(
		r,
		(5, 0, "R_0_5_0", Target::MINIMUM),
		(5, 1, "R_0_5_1_MAXIMUM", Target::MAXIMUM)
	);
}