use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nilah(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Max),
		(0, 1, "Q_0_0_1_MIN", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1_MAX", Max),
		(0, 2, "R_0_0_2", Min),
		(0, 3, "R_0_0_3_MAX", Max)
	);
}