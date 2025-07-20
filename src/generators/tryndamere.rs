use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_tryndamere(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 1, "Q_0_1_1_BONUS", Target::MINIMUM)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
}