use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_tahmkench(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 1, "W_0_2_1", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_1_0_0", Target::MINIMUM)
	);
}