use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ashe(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 1, "W_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}