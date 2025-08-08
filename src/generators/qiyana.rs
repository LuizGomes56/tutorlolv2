use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_qiyana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM),
		(0, 1, "Q_0_0_1", Target::MINIMUM),
		(2, 0, "Q_1_2_0", Target::MINIMUM),
		(2, 1, "Q_1_2_1", Target::MINIMUM),
		(4, 0, "Q_1_4_0_MAX", Target::MAXIMUM),
		(4, 1, "Q_1_4_1_MAX", Target::MAXIMUM)
	);
	ability!(
		w,
		(3, 1, "W_0_3_1_BONUS", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MNSTR", Target::MINIMUM),
		(1, 1, "R_0_1_1", Target::MINIMUM)
	);
}