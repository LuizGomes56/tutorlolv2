use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_tristana(data: CdnChampion) -> Champion {
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0_MINIMUM", Target::MINIMUM),
		(2, 0, "E_0_2_0_BONUS", Target::MINIMUM),
		(2, 1, "E_0_2_1_BONUS", Target::MINIMUM),
		(2, 2, "E_0_2_2", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}