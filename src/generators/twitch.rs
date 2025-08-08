use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_twitch(data: CdnChampion) -> Champion {
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(2, 0, "E_0_2_0_MAX", Target::MAXIMUM),
		(2, 1, "E_0_2_1_MIN", Target::MINIMUM),
		(2, 2, "E_0_2_2", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_BONUS", Target::MINIMUM)
	);
}