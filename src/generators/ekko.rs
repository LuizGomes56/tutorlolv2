use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_ekko(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(1, 0, "Q_0_1_0", Target::MINIMUM),
		(1, 1, "Q_0_1_1_MAX", Target::MAXIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Target::MINIMUM)
	);
}