use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_elise(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MONSTER", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 0, "Q_1_0_0_MONSTER", Target::MINIMUM),
		(0, 1, "Q_1_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
}