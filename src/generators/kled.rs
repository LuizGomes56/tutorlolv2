use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kled(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Min),
		(0, 1, "Q_0_0_1", Min),
		(2, 0, "Q_0_2_0", Min),
		(2, 2, "Q_0_2_2_MAX", Max),
		(0, 0, "Q_1_0_0", Min),
		(2, 0, "Q_1_2_0_MAX", Max),
		(2, 1, "Q_1_2_1", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_BONUS", Min),
		(2, 0, "W_0_2_0_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(2, 0, "E_0_2_0_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MAX", Max),
		(1, 1, "R_0_1_1_MIN", Min)
	);
}