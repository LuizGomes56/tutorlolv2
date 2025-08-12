use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_karma(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 0, "Q_1_0_0_BONUS", Min),
		(0, 1, "Q_1_0_1_MAX", Max),
		(2, 0, "Q_1_2_0", Min),
		(2, 1, "Q_1_2_1_BONUS", Min),
		(2, 2, "Q_1_2_2_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(1, 1, "W_0_1_1_MAX", Max)
	);
}