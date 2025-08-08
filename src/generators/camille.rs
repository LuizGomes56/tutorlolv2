use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_camille(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1_BONUS", Target::MINIMUM),
		(3, 0, "Q_0_3_0_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(2, 0, "W_0_2_0_MNSTR", Target::MINIMUM),
		(2, 1, "W_0_2_1_MNSTR", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_1_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0_BONUS", Target::MINIMUM)
	);
}