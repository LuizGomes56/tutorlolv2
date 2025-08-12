use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sivir(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(1, 0, "Q_0_1_0_MIN", Min),
		(1, 1, "Q_0_1_1_MAX", Max)
	);
	ability!(
		w,
		(0, 1, "W_0_0_1", Min),
		(0, 2, "W_0_0_2", Min),
		(0, 3, "W_0_0_3", Min),
		(0, 4, "W_0_0_4", Min)
	);
}