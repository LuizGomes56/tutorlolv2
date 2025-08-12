use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_riven(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Min),
		(2, 1, "Q_0_2_1_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_1_0_0_MAX", Max),
		(0, 1, "R_1_0_1_MIN", Min)
	);
}