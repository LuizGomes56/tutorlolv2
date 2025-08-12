use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_renekton(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 5, "Q_0_0_5", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1_MAX", Max),
		(1, 0, "W_0_1_0_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 0, "E_1_0_0_MAX", Max),
		(2, 1, "E_1_2_1_BONUS", Min),
		(2, 2, "E_1_2_2_BONUS", Min),
		(2, 3, "E_1_2_3_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1_MAX", Max)
	);
}