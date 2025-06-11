use super::{
	Ability, CdnChampion, Champion,
	HashMap, Target, extract_ability_damage
};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(2, 0, "Q_0_2_0", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_MINIMUM", Target::MINIMUM),
		(2, 1, "W_0_2_1_MAXIMUM", Target::MAXIMUM),
		(3, 0, "W_0_3_0_MINIMUM", Target::MINIMUM),
		(3, 1, "W_0_3_1_MAXIMUM", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MAXIMUM", Target::MAXIMUM),
		(2, 0, "E_0_2_0_MAXIMUM", Target::MAXIMUM),
		(3, 0, "E_0_3_0", Target::MINIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}