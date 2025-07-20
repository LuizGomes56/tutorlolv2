use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_kindred(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0", Target::MINIMUM),
		(3, 1, "W_0_3_1_MONSTER", Target::MINIMUM)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Target::MINIMUM),
		(2, 1, "E_0_2_1", Target::MINIMUM)
	);
}