use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_anivia(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 2, "Q_0_2_2_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(3, 0, "R_0_3_0", Target::MINIMUM)
	);
}