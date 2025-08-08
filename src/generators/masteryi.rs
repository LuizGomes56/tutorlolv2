use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_masteryi(data: CdnChampion) -> Champion {
	ability!(
		q,
		(3, 0, "Q_0_3_0_MNSTR", Target::MINIMUM),
		(3, 1, "Q_0_3_1_MNSTR", Target::MINIMUM),
		(3, 2, "Q_0_3_2_MAX", Target::MAXIMUM),
		(3, 3, "Q_0_3_3_MNSTR", Target::MINIMUM),
		(3, 4, "Q_0_3_4", Target::MINIMUM),
		(3, 5, "Q_0_3_5", Target::MINIMUM),
		(3, 6, "Q_0_3_6_MNSTR", Target::MINIMUM)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Target::MINIMUM),
		(2, 1, "W_0_2_1", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Target::MINIMUM)
	);
}