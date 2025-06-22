use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM),
		(1, 2, "R_0_1_2_MAXIMUM", Target::MAXIMUM),
		(2, 0, "R_0_2_0", Target::MINIMUM),
		(3, 0, "R_0_3_0", Target::MINIMUM),
		(3, 1, "R_0_3_1", Target::MINIMUM),
		(3, 2, "R_0_3_2_MAXIMUM", Target::MAXIMUM)
	);
}