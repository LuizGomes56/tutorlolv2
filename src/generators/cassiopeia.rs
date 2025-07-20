use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_cassiopeia(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 2, "W_0_1_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_BONUS", Target::MINIMUM),
		(1, 3, "E_0_1_3_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}