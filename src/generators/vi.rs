use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_vi(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_MAX", Target::MAXIMUM),
		(1, 1, "Q_0_1_1_MIN", Target::MINIMUM)
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
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}