use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kayn(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1_MAX", Max),
		(1, 0, "Q_0_1_0", Min),
		(1, 1, "Q_0_1_1_MAX", Max),
		(2, 0, "Q_0_2_0_MNSTR", Min),
		(2, 1, "Q_0_2_1_MNSTR", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Min)
	);
}