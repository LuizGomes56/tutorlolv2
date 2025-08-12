use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_talon(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(1, 0, "W_0_1_0", Min),
		(1, 2, "W_0_1_2_MAX", Max)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Min),
		(1, 0, "R_0_1_0_MAX", Max)
	);
}