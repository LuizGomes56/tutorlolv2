use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_gangplank(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_BONUS", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1", Min),
		(0, 2, "R_0_0_2_MAX", Max),
		(1, 0, "R_0_1_0_MAX", Max),
		(1, 1, "R_0_1_1", Min),
		(2, 0, "R_0_2_0_MAX", Max),
		(2, 1, "R_0_2_1_MAX", Max)
	);
}