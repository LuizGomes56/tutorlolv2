use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_nocturne(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}