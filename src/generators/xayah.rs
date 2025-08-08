use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_xayah(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAX", Target::MAXIMUM),
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Target::MINIMUM),
		(2, 1, "E_0_2_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}