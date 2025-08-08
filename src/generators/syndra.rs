use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_syndra(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(3, 0, "W_0_3_0_BONUS", Target::MINIMUM),
		(3, 1, "W_0_3_1_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1_MAX", Target::MAXIMUM),
		(0, 2, "R_0_0_2_MIN", Target::MINIMUM)
	);
}