use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_twistedfate(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(3, 0, "W_0_3_0", Target::MINIMUM),
		(4, 0, "W_0_4_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1_BONUS", Target::MINIMUM)
	);
}