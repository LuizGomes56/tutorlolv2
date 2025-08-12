use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sylas(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(1, 0, "Q_0_1_0", Min),
		(1, 1, "Q_0_1_1", Min),
		(1, 2, "Q_0_1_2_MAX", Max),
		(1, 3, "Q_0_1_3_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_1_0_0", Min)
	);
}