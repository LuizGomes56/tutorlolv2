use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_leblanc(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(2, 0, "R_0_2_0", Target::MINIMUM),
		(2, 1, "R_0_2_1", Target::MINIMUM),
		(2, 2, "R_0_2_2_MAX", Target::MAXIMUM),
		(3, 0, "R_0_3_0", Target::MINIMUM),
		(3, 1, "R_0_3_1", Target::MINIMUM),
		(3, 2, "R_0_3_2_MAX", Target::MAXIMUM)
	);
}