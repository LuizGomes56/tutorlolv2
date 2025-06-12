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
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM),
		(1, 1, "W_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_BONUS", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}