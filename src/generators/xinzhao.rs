use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_xinzhao(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min),
		(0, 1, "Q_0_0_1_BONUS", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1", Min),
		(0, 2, "W_0_0_2_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}