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
		(1, 0, "E_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "E_0_1_1_BONUS", Target::MINIMUM),
		(1, 2, "E_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}