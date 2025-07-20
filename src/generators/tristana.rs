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
		(0, 0, "E_0_0_0_MINIMUM", Target::MINIMUM),
		(2, 0, "E_0_2_0", Target::MINIMUM),
		(3, 0, "E_0_3_0_BONUS", Target::MINIMUM),
		(3, 1, "E_0_3_1_BONUS", Target::MINIMUM),
		(3, 2, "E_0_3_2", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Target::MINIMUM)
	);
}