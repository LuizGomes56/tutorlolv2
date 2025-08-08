use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_morgana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAX", Target::MAXIMUM),
		(0, 1, "W_0_0_1_MAX", Target::MAXIMUM),
		(0, 2, "W_0_0_2_MIN", Target::MINIMUM),
		(0, 3, "W_0_0_3_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(0, 2, "R_0_0_2_MAX", Target::MAXIMUM)
	);
}