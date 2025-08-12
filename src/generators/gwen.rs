use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_gwen(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 2, "Q_0_0_2", Min),
		(0, 3, "Q_0_0_3", Min),
		(1, 0, "Q_0_1_0_MAX", Max),
		(1, 1, "Q_0_1_1_MAX", Max),
		(1, 2, "Q_0_1_2_MIN", Min),
		(1, 3, "Q_0_1_3_MIN", Min)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1_BONUS", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1", Min),
		(3, 0, "R_0_3_0_MAX", Max),
		(3, 1, "R_0_3_1_MAX", Max),
		(3, 2, "R_0_3_2_MAX", Max)
	);
}