use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_nami(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 1, "W_0_1_1", Target::MINIMUM),
		(1, 2, "W_0_1_2_MINIMUM", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Target::MINIMUM),
		(0, 2, "E_0_0_2_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}