use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_kaisa(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0_MAXIMUM", Target::MAXIMUM),
		(3, 0, "Q_0_3_0", Target::MINIMUM),
		(3, 1, "Q_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
}