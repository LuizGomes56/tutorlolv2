use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 0, "Q_1_0_0_BONUS", Target::MINIMUM),
		(0, 1, "Q_1_0_1_MAXIMUM", Target::MAXIMUM),
		(1, 0, "Q_1_1_0", Target::MINIMUM),
		(1, 1, "Q_1_1_1_BONUS", Target::MINIMUM),
		(1, 2, "Q_1_1_2_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0_MAXIMUM", Target::MAXIMUM)
	);
}