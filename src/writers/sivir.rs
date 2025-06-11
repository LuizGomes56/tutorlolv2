use super::{
	Ability, CdnChampion, Champion,
	HashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0_MINIMUM", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 1, "W_0_0_1", Target::MINIMUM),
		(0, 2, "W_0_0_2", Target::MINIMUM),
		(0, 3, "W_0_0_3", Target::MINIMUM),
		(0, 4, "W_0_0_4", Target::MINIMUM)
	);
}