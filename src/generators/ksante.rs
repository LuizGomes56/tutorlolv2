use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ksante(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "W_0_1_1_BONUS", Target::MINIMUM),
		(1, 2, "W_0_1_2_MAX", Target::MAXIMUM),
		(4, 0, "W_0_4_0_MNSTR", Target::MINIMUM),
		(4, 1, "W_0_4_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(3, 0, "R_0_3_0", Target::MINIMUM),
		(3, 1, "R_0_3_1_MAX", Target::MAXIMUM)
	);
}