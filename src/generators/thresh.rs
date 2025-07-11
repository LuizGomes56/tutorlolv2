use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_thresh(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "E_0_0_1_BONUS", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}