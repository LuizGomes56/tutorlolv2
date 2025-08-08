use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_poppy(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MNSTR", Target::MINIMUM),
		(0, 2, "Q_0_0_2", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MNSTR", Target::MINIMUM),
		(1, 2, "Q_0_1_2_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MAX", Target::MAXIMUM),
		(3, 0, "R_0_3_0", Target::MINIMUM)
	);
}