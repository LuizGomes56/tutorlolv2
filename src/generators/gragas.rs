use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_gragas(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_MAX", Target::MAXIMUM),
		(1, 1, "Q_0_1_1_MAX", Target::MAXIMUM),
		(1, 3, "Q_0_1_3_MIN", Target::MINIMUM),
		(1, 4, "Q_0_1_4_MIN", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "W_0_1_1_MNSTR", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}