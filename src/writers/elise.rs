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
		(0, 1, "Q_0_0_1_MONSTER", Target::MINIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM),
		(0, 1, "Q_1_0_1_MONSTER", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
}