use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_pyke(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
}