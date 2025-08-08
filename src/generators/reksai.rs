use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_reksai(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_1_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}