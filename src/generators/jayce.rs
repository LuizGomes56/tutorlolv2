use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_jayce(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 0, "Q_1_0_0", Target::MINIMUM),
		(1, 0, "Q_1_1_0_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM),
		(0, 1, "W_0_0_1_MAX", Target::MAXIMUM),
		(0, 0, "W_1_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM),
		(0, 1, "E_0_0_1_MNSTR", Target::MINIMUM)
	);
}