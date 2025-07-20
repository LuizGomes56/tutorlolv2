use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_neeko(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MONSTER", Target::MINIMUM),
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}