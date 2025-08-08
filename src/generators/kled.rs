use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_kled(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 2, "Q_0_2_2_MAX", Target::MAXIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM),
		(2, 0, "Q_1_2_0_MAX", Target::MAXIMUM),
		(2, 1, "Q_1_2_1", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_BONUS", Target::MINIMUM),
		(2, 0, "W_0_2_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(2, 0, "E_0_2_0_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MAX", Target::MAXIMUM),
		(1, 1, "R_0_1_1_MIN", Target::MINIMUM)
	);
}