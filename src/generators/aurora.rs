use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_aurora(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MAX", Target::MAXIMUM),
		(1, 1, "Q_0_1_1_MIN", Target::MINIMUM),
		(1, 2, "Q_0_1_2_MAX", Target::MAXIMUM),
		(1, 3, "Q_0_1_3_MIN", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}