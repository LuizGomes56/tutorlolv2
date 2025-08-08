use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_aurelionsol(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MAX", Target::MAXIMUM),
		(5, 0, "Q_0_5_0_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 0, "R_1_0_0", Target::MINIMUM),
		(1, 0, "R_1_1_0", Target::MINIMUM)
	);
}