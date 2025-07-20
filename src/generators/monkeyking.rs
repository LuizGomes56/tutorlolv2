use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_monkeyking(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM),
		(4, 0, "R_0_4_0_MAXIMUM", Target::MAXIMUM)
	);
}