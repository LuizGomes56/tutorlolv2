use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_malphite(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_BONUS", Target::MINIMUM),
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}