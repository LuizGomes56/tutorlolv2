use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_gangplank(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(0, 2, "R_0_0_2_MAX", Target::MAXIMUM),
		(1, 0, "R_0_1_0_MAX", Target::MAXIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM),
		(2, 0, "R_0_2_0_MAX", Target::MAXIMUM),
		(2, 1, "R_0_2_1_MAX", Target::MAXIMUM)
	);
}