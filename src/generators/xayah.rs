use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_xayah(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1_MAX", Max),
		(1, 0, "Q_0_1_0", Min),
		(1, 1, "Q_0_1_1_MAX", Max)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Min),
		(2, 1, "E_0_2_1", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}