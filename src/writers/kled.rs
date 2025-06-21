use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1_MONSTER", Target::MINIMUM),
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM),
		(2, 0, "Q_1_2_0", Target::MINIMUM),
		(2, 1, "Q_1_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "W_0_1_1_MONSTER", Target::MINIMUM),
		(2, 0, "W_0_2_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(2, 0, "E_0_2_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0_MINIMUM", Target::MINIMUM),
		(3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
}