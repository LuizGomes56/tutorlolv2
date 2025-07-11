use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_vayne(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "W_0_1_1_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1_BONUS", Target::MINIMUM)
	);
}