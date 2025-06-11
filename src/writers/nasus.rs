use super::{
	Ability, CdnChampion, Champion,
	HashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Target::MINIMUM),
		(1, 1, "R_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
}