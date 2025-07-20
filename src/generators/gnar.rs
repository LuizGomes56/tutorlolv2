use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_gnar(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_BONUS", Target::MINIMUM),
		(0, 0, "W_1_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(4, 0, "E_0_4_0", Target::MINIMUM),
		(0, 0, "E_1_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAXIMUM", Target::MAXIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM)
	);
}