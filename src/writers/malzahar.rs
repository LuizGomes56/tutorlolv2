use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(2, 1, "W_0_2_1", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "E_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(2, 0, "R_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "R_0_2_1", Target::MINIMUM)
	);
}