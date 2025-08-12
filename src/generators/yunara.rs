use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yunara(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min),
		(0, 1, "Q_0_0_1_MAX", Max),
		(0, 3, "Q_0_0_3_BONUS", Min),
		(0, 4, "Q_0_0_4_MAX", Max),
		(2, 0, "Q_0_2_0_BONUS", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1", Min),
		(0, 2, "W_0_0_2_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min)
	);
}