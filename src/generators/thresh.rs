use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_thresh(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "E_0_1_1_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}