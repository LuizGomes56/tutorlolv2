use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_zilean(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM)
	);
}