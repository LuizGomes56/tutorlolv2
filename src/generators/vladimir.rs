use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_vladimir(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(2, 0, "Q_0_2_0_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(4, 0, "E_0_4_0_MAX", Target::MAXIMUM),
		(4, 1, "E_0_4_1_MIN", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 1, "R_0_1_1", Target::MINIMUM)
	);
}