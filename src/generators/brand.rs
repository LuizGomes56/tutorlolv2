use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_brand(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAX", Target::MAXIMUM),
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1_MAX", Target::MAXIMUM)
	);
}