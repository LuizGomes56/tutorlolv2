use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_karma(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 0, "Q_1_0_0_BONUS", Target::MINIMUM),
		(0, 1, "Q_1_0_1_MAX", Target::MAXIMUM),
		(2, 0, "Q_1_2_0", Target::MINIMUM),
		(2, 1, "Q_1_2_1_BONUS", Target::MINIMUM),
		(2, 2, "Q_1_2_2_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAX", Target::MAXIMUM)
	);
}