use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_lucian(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1", Min),
		(0, 2, "R_0_0_2", Min),
		(0, 3, "R_0_0_3", Min),
		(0, 4, "R_0_0_4_MAX", Max),
		(0, 5, "R_0_0_5_MAX", Max)
	);
}