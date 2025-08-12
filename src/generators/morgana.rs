use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_morgana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAX", Max),
		(0, 1, "W_0_0_1_MAX", Max),
		(0, 2, "W_0_0_2_MIN", Min),
		(0, 3, "W_0_0_3_MAX", Max)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Min),
		(0, 2, "R_0_0_2_MAX", Max)
	);
}