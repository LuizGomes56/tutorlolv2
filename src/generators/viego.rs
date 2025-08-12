use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_viego(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Min),
		(0, 1, "Q_0_0_1_MAX", Max),
		(0, 2, "Q_0_0_2_MNSTR", Min),
		(0, 3, "Q_0_0_3_MIN", Min),
		(3, 0, "Q_0_3_0_BONUS", Min),
		(3, 1, "Q_0_3_1_BONUS", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}