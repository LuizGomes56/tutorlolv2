use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_yuumi(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAX", Target::MAXIMUM),
		(2, 0, "Q_0_2_0_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(4, 0, "R_0_4_0", Target::MINIMUM),
		(4, 1, "R_0_4_1", Target::MINIMUM),
		(4, 2, "R_0_4_2_MAX", Target::MAXIMUM)
	);
}