use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_briar(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 2, "W_0_2_2", Target::MINIMUM),
		(0, 0, "W_1_0_0_BONUS", Target::MINIMUM),
		(1, 0, "W_1_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0_BONUS", Target::MINIMUM),
		(2, 1, "E_0_2_1_MAXIMUM", Target::MAXIMUM),
		(3, 0, "E_0_3_0_MAXIMUM", Target::MAXIMUM),
		(3, 1, "E_0_3_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Target::MINIMUM)
	);
}