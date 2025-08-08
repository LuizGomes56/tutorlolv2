use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_nilah(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Target::MAXIMUM),
		(0, 1, "Q_0_0_1_MIN", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1_MAX", Target::MAXIMUM),
		(0, 2, "R_0_0_2", Target::MINIMUM),
		(0, 3, "R_0_0_3_MAX", Target::MAXIMUM)
	);
}