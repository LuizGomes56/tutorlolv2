use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_samira(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(2, 1, "W_0_2_1_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM),
		(1, 2, "R_0_1_2_MAX", Target::MAXIMUM),
		(1, 3, "R_0_1_3_MAX", Target::MAXIMUM)
	);
}