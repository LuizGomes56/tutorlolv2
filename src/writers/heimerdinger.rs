use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAXIMUM", Target::MAXIMUM),
		(1, 2, "W_0_1_2_MAXIMUM", Target::MAXIMUM),
		(1, 3, "W_0_1_3", Target::MINIMUM),
		(1, 4, "W_0_1_4_MAXIMUM", Target::MAXIMUM),
		(1, 5, "W_0_1_5_MAXIMUM", Target::MAXIMUM),
		(0, 0, "W_1_0_0", Target::MINIMUM),
		(1, 0, "W_1_1_0", Target::MINIMUM),
		(1, 1, "W_1_1_1", Target::MINIMUM),
		(1, 2, "W_1_1_2_MAXIMUM", Target::MAXIMUM),
		(1, 3, "W_1_1_3", Target::MINIMUM),
		(1, 4, "W_1_1_4_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 0, "E_1_0_0", Target::MINIMUM)
	);
}