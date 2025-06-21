use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MONSTER", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MONSTER", Target::MINIMUM),
		(1, 2, "Q_0_1_2", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0_MINIMUM", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM),
		(2, 0, "E_0_2_0_MONSTER", Target::MINIMUM),
		(2, 1, "E_0_2_1_MONSTER", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "R_0_0_1_MONSTER", Target::MINIMUM),
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}