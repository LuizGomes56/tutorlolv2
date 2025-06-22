use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1", Target::MINIMUM),
		(2, 2, "Q_0_2_2_MAXIMUM", Target::MAXIMUM),
		(2, 3, "Q_0_2_3_MONSTER", Target::MINIMUM),
		(2, 4, "Q_0_2_4_MONSTER", Target::MINIMUM),
		(2, 5, "Q_0_2_5_MONSTER", Target::MINIMUM),
		(2, 6, "Q_0_2_6_MONSTER", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Target::MINIMUM)
	);
}