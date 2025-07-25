use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_volibear(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
	ability!(
		r,
		(4, 0, "R_0_4_0", Target::MINIMUM)
	);
}