use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_belveth(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MNSTR", Target::MINIMUM),
		(2, 2, "Q_0_2_2_MNSTR", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(3, 0, "E_0_3_0_MNSTR", Target::MINIMUM),
		(3, 1, "E_0_3_1_MNSTR", Target::MINIMUM),
		(5, 0, "E_0_5_0_MAX", Target::MAXIMUM),
		(5, 1, "E_0_5_1_MIN", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(2, 0, "R_0_2_0_BONUS", Target::MINIMUM),
		(2, 1, "R_0_2_1_MNSTR", Target::MINIMUM)
	);
}