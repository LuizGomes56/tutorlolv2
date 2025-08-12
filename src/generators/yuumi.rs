use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yuumi(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(1, 1, "Q_0_1_1_MAX", Max),
		(2, 0, "Q_0_2_0_BONUS", Min)
	);
	ability!(
		r,
		(4, 0, "R_0_4_0", Min),
		(4, 1, "R_0_4_1", Min),
		(4, 2, "R_0_4_2_MAX", Max)
	);
}