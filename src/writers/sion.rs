use super::{
	Ability, CdnChampion, Champion,
	FxHashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAXIMUM", Target::MAXIMUM),
		(0, 1, "Q_0_0_1_MINIMUM", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MAXIMUM", Target::MAXIMUM),
		(2, 0, "Q_0_2_0_MONSTER", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MONSTER", Target::MINIMUM),
		(2, 2, "Q_0_2_2_MINIMUM", Target::MINIMUM),
		(2, 3, "Q_0_2_3_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0_MINIMUM", Target::MINIMUM),
		(3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
}