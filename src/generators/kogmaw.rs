use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_kogmaw(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 1, "W_0_0_1_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAX", Target::MAXIMUM),
		(0, 1, "R_0_0_1_MIN", Target::MINIMUM)
	);
}