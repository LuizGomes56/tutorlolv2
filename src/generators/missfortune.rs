use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_missfortune(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
}