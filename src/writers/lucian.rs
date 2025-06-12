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
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM),
		(0, 1, "R_0_0_1", Target::MINIMUM),
		(0, 2, "R_0_0_2_MAXIMUM", Target::MAXIMUM),
		(0, 3, "R_0_0_3", Target::MINIMUM),
		(0, 4, "R_0_0_4", Target::MINIMUM),
		(0, 5, "R_0_0_5_MAXIMUM", Target::MAXIMUM)
	);
}