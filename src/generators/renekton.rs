use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_renekton(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 5, "Q_0_0_5", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1_MAX", Target::MAXIMUM),
		(1, 0, "W_0_1_0_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 0, "E_1_0_0_MAX", Target::MAXIMUM),
		(2, 1, "E_1_2_1_BONUS", Target::MINIMUM),
		(2, 2, "E_1_2_2_BONUS", Target::MINIMUM),
		(2, 3, "E_1_2_3_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1_MAX", Target::MAXIMUM)
	);
}