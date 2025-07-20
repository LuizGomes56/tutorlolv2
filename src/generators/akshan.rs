use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_akshan(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(3, 0, "Q_0_3_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(4, 0, "E_0_4_0", Target::MINIMUM)
	);
	ability!(
		r,
		(4, 0, "R_0_4_0", Target::MINIMUM),
		(4, 1, "R_0_4_1_MAXIMUM", Target::MAXIMUM),
		(4, 2, "R_0_4_2_MINIMUM", Target::MINIMUM),
		(4, 3, "R_0_4_3_MINIMUM", Target::MINIMUM)
	);
}