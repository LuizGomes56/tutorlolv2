use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_jax(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_MAXIMUM", Target::MAXIMUM),
		(1, 1, "E_0_1_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 4, "R_0_0_4", Target::MINIMUM),
		(2, 0, "R_0_2_0_BONUS", Target::MINIMUM)
	);
}