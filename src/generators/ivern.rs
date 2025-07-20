use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ivern(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_BONUS", Target::MINIMUM),
		(3, 0, "W_0_3_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
}