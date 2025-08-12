use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_varus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0_MAX", Max),
		(2, 1, "Q_0_2_1_MAX", Max),
		(2, 2, "Q_0_2_2_MIN", Min),
		(2, 3, "Q_0_2_3_MIN", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAX", Max),
		(0, 1, "W_0_0_1_MIN", Min),
		(1, 0, "W_0_1_0_BONUS", Min),
		(1, 1, "W_0_1_1_BONUS", Min),
		(1, 2, "W_0_1_2_BONUS", Min),
		(1, 3, "W_0_1_3_BONUS", Min),
		(4, 0, "W_0_4_0_BONUS", Min)
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