use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_leesin(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 0, "Q_1_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "Q_1_0_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}