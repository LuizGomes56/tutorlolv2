use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_smolder(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Max),
		(0, 1, "Q_0_0_1_MAX", Max),
		(0, 2, "Q_0_0_2", Min),
		(3, 0, "Q_0_3_0_MAX", Max),
		(3, 1, "Q_0_3_1_MAX", Max),
		(3, 2, "Q_0_3_2", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1", Min),
		(0, 2, "W_0_0_2_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MAX", Max),
		(0, 1, "E_0_0_1", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAX", Max),
		(0, 1, "R_0_0_1", Min)
	);
}