use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_yasuo(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(2, 0, "E_0_2_0_BONUS", Target::MINIMUM),
		(2, 1, "E_0_2_1_BONUS", Target::MINIMUM),
		(2, 2, "E_0_2_2_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Target::MINIMUM)
	);
}