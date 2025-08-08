use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_pantheon(data: CdnChampion) -> Champion {
	ability!(
		q,
		(4, 0, "Q_0_4_0", Target::MINIMUM),
		(4, 1, "Q_0_4_1", Target::MINIMUM),
		(4, 2, "Q_0_4_2_MAX", Target::MAXIMUM),
		(4, 3, "Q_0_4_3_MAX", Target::MAXIMUM),
		(5, 0, "Q_0_5_0_MAX", Target::MAXIMUM),
		(5, 1, "Q_0_5_1", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(3, 0, "E_0_3_0", Target::MINIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Target::MINIMUM),
		(3, 1, "R_0_3_1", Target::MINIMUM)
	);
}