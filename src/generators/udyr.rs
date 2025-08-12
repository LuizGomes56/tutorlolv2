use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_udyr(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min),
		(0, 1, "Q_0_0_1_MAX", Max),
		(1, 1, "Q_0_1_1_BONUS", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 2, "R_0_1_2_MAX", Max)
	);
}