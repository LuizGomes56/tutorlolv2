use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_swain(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 2, "Q_0_0_2_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
	ability!(
		r,
		(2, 1, "R_0_2_1", Min),
		(0, 0, "R_1_0_0", Min)
	);
}