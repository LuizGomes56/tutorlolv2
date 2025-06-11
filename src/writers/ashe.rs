use super::{
	Ability, CdnChampion, Champion,
	HashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM),
		(2, 1, "Q_0_2_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 1, "W_0_0_1", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}