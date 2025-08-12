use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_samira(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Min),
		(2, 1, "W_0_2_1_MAX", Max)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1", Min),
		(1, 2, "R_0_1_2_MAX", Max),
		(1, 3, "R_0_1_3_MAX", Max)
	);
}