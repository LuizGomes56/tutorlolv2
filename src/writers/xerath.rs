use super::{
	Ability, CdnChampion, Champion,
	FxHashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(1, 0, "W_0_1_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1_MAXIMUM", Target::MAXIMUM),
		(2, 1, "R_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
}