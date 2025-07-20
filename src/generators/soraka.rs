use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_soraka(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 1, "E_0_1_1_MAXIMUM", Target::MAXIMUM)
	);
}