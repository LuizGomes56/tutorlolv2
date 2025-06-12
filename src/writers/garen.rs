use super::{
	Ability, CdnChampion, Champion,
	FxHashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(2, 0, "E_0_2_0_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}