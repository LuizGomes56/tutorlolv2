use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_skarner(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1_BONUS", Target::MINIMUM),
		(0, 2, "Q_0_0_2_BONUS", Target::MINIMUM),
		(3, 0, "Q_0_3_0_MONSTER", Target::MINIMUM),
		(0, 0, "Q_1_0_0_MONSTER", Target::MINIMUM),
		(0, 1, "Q_1_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}