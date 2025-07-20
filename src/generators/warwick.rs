use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_warwick(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MONSTER", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAXIMUM", Target::MAXIMUM)
	);
}