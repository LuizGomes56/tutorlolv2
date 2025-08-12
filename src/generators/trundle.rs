use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_trundle(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min),
		(1, 0, "Q_0_1_0", Min),
		(1, 1, "Q_0_1_1_BONUS", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAX", Max),
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1", Min)
	);
}