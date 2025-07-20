use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_kayn(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MAXIMUM", Target::MAXIMUM),
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM),
		(2, 0, "Q_0_2_0_MONSTER", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MONSTER", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Target::MINIMUM)
	);
}