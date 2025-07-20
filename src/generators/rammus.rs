use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_rammus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MONSTER", Target::MINIMUM)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Target::MINIMUM)
	);
}