use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_renata(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Target::MINIMUM)
	);
}