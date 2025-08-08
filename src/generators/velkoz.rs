use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_velkoz(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Target::MINIMUM),
		(3, 1, "R_0_3_1_MAX", Target::MAXIMUM)
	);
}