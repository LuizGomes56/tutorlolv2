use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_taliyah(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(0, 2, "Q_0_0_2_MAXIMUM", Target::MAXIMUM),
		(3, 0, "Q_0_3_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(1, 0, "E_0_1_0_MAXIMUM", Target::MAXIMUM),
		(2, 0, "E_0_2_0", Target::MINIMUM)
	);
}