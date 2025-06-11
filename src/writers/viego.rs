use super::{
	Ability, CdnChampion, Champion,
	HashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Target::MINIMUM),
		(0, 1, "Q_0_0_1_BONUS", Target::MINIMUM),
		(3, 0, "Q_0_3_0_MINIMUM", Target::MINIMUM),
		(3, 1, "Q_0_3_1_MAXIMUM", Target::MAXIMUM),
		(3, 2, "Q_0_3_2_MONSTER", Target::MINIMUM),
		(3, 3, "Q_0_3_3_MONSTER", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}