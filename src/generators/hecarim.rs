use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_hecarim(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(3, 0, "E_0_3_0_MAXIMUM", Target::MAXIMUM),
		(3, 1, "E_0_3_1_MINIMUM", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}