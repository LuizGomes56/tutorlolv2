use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_heimerdinger(data: CdnChampion) -> Champion {
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0_MAXIMUM", Target::MAXIMUM),
		(1, 1, "W_0_1_1_MAXIMUM", Target::MAXIMUM),
		(1, 2, "W_0_1_2", Target::MINIMUM),
		(1, 3, "W_0_1_3", Target::MINIMUM),
		(1, 4, "W_0_1_4_MAXIMUM", Target::MAXIMUM),
		(1, 5, "W_0_1_5_MAXIMUM", Target::MAXIMUM),
		(0, 0, "W_1_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "W_1_0_1_MAXIMUM", Target::MAXIMUM),
		(0, 2, "W_1_0_2", Target::MINIMUM),
		(0, 3, "W_1_0_3", Target::MINIMUM),
		(0, 4, "W_1_0_4", Target::MINIMUM),
		(2, 0, "W_1_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(2, 0, "E_1_2_0", Target::MINIMUM)
	);
}