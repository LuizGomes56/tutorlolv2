use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_viktor(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(2, 0, "Q_0_2_0", Min),
		(2, 1, "Q_0_2_1_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0", Min),
		(1, 1, "E_0_1_1_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(5, 0, "R_0_5_0", Min),
		(5, 1, "R_0_5_1_MAX", Max)
	);
}