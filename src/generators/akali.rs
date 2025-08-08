use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_akali(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(2, 0, "E_0_2_0", Target::MINIMUM),
		(2, 1, "E_0_2_1_MAX", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(2, 0, "R_0_2_0_MAX", Target::MAXIMUM),
		(2, 1, "R_0_2_1_MIN", Target::MINIMUM)
	);
}