use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kaisa(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Min),
		(2, 0, "Q_0_2_0_MAX", Max),
		(3, 0, "Q_0_3_0", Min),
		(3, 1, "Q_0_3_1_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
}