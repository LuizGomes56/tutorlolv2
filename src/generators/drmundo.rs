use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_drmundo(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0_MONSTER", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1_MAXIMUM", Target::MAXIMUM),
		(2, 0, "W_0_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "E_0_0_1_BONUS", Target::MINIMUM),
		(3, 0, "E_0_3_0_BONUS", Target::MINIMUM)
	);
}