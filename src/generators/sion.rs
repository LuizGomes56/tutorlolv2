use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_sion(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM),
		(0, 2, "Q_0_0_2_MINIMUM", Target::MINIMUM),
		(3, 0, "Q_0_3_0_MAXIMUM", Target::MAXIMUM),
		(3, 1, "Q_0_3_1_MONSTER", Target::MINIMUM),
		(3, 2, "Q_0_3_2_MINIMUM", Target::MINIMUM),
		(3, 3, "Q_0_3_3_MONSTER", Target::MINIMUM)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "R_0_2_1_MINIMUM", Target::MINIMUM)
	);
}