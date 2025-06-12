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
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(1, 0, "R_0_1_0", Target::MINIMUM)
	);
}