use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_nautilus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MAXIMUM", Target::MAXIMUM),
		(0, 2, "E_0_0_2", Target::MINIMUM),
		(1, 0, "E_0_1_0_MONSTER", Target::MINIMUM),
		(1, 1, "E_0_1_1_MONSTER", Target::MINIMUM),
		(1, 2, "E_0_1_2_MONSTER", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(1, 0, "R_0_1_0_MAXIMUM", Target::MAXIMUM)
	);
}