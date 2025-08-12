use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_leblanc(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(1, 0, "Q_0_1_0_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0", Min),
		(1, 1, "E_0_1_1_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(2, 0, "R_0_2_0", Min),
		(2, 1, "R_0_2_1", Min),
		(2, 2, "R_0_2_2_MAX", Max),
		(3, 0, "R_0_3_0", Min),
		(3, 1, "R_0_3_1", Min),
		(3, 2, "R_0_3_2_MAX", Max)
	);
}