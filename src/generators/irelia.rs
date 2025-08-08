use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_irelia(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0_MAX", Target::MAXIMUM),
		(3, 1, "W_0_3_1_MIN", Target::MINIMUM)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}