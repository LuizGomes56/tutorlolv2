use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_viego(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAX", Target::MAXIMUM),
		(0, 2, "Q_0_0_2_MNSTR", Target::MINIMUM),
		(0, 3, "Q_0_0_3_MIN", Target::MINIMUM),
		(3, 0, "Q_0_3_0_BONUS", Target::MINIMUM),
		(3, 1, "Q_0_3_1_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}