use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_milio(data: CdnChampion) -> Champion {
	ability!(
		q,
		(3, 0, "Q_0_3_0", Target::MINIMUM)
	);
}