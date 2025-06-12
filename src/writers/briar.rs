use super::{
	Ability, CdnChampion, Champion,
	FxHashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 2, "W_0_1_2", Target::MINIMUM),
		(0, 0, "W_1_0_0_BONUS", Target::MINIMUM),
		(1, 0, "W_1_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0_MAXIMUM", Target::MAXIMUM),
		(2, 1, "E_0_2_1_MINIMUM", Target::MINIMUM),
		(3, 0, "E_0_3_0_BONUS", Target::MINIMUM),
		(3, 1, "E_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}