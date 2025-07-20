use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_shen(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_MONSTER", Target::MINIMUM),
		(2, 0, "Q_0_2_0_BONUS", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM),
		(3, 0, "Q_0_3_0_BONUS", Target::MINIMUM),
		(3, 1, "Q_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
}