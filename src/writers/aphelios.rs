use super::*;

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
	ability!(
		p,
		(3, 0, "P_0_3_0_BONUS", Target::MINIMUM)
	);
}