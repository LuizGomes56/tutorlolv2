use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_bard(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Target::MINIMUM)
	);
}