use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_riven(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_1_0_0_MAX", Target::MAXIMUM),
		(0, 1, "R_1_0_1_MIN", Target::MINIMUM)
	);
}