use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_sylas(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1", Target::MINIMUM),
		(1, 2, "Q_0_1_2_MAXIMUM", Target::MAXIMUM),
		(1, 3, "Q_0_1_3_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_1_0_0", Target::MINIMUM)
	);
}