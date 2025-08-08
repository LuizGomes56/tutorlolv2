use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_sett(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "Q_0_0_1_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MNSTR", Target::MINIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM)
	);
}