use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_twitch(data: CdnChampion) -> Champion {
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MINIMUM", Target::MINIMUM),
		(1, 2, "E_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_BONUS", Target::MINIMUM)
	);
}