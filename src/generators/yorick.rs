use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_yorick(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MONSTER", Target::MINIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM),
		(0, 2, "E_0_0_2_MINIMUM", Target::MINIMUM)
	);
}