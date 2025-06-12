use super::{
	Ability, CdnChampion, Champion,
	FxHashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM),
		(1, 0, "Q_1_1_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAXIMUM", Target::MAXIMUM),
		(0, 0, "W_1_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MONSTER", Target::MINIMUM)
	);
}