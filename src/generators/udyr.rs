use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_udyr(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM),
		(1, 1, "Q_0_1_1_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 2, "R_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
}