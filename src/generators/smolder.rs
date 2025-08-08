use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_smolder(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Target::MAXIMUM),
		(0, 1, "Q_0_0_1_MAX", Target::MAXIMUM),
		(0, 2, "Q_0_0_2", Target::MINIMUM),
		(3, 0, "Q_0_3_0_MAX", Target::MAXIMUM),
		(3, 1, "Q_0_3_1_MAX", Target::MAXIMUM),
		(3, 2, "Q_0_3_2", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1", Target::MINIMUM),
		(0, 2, "W_0_0_2_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MAX", Target::MAXIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAX", Target::MAXIMUM),
		(0, 1, "R_0_0_1", Target::MINIMUM)
	);
}