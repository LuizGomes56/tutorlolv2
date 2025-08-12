use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kalista(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Min),
		(1, 1, "W_0_1_1_MAX", Max)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min),
		(1, 1, "E_0_1_1", Min)
	);
}