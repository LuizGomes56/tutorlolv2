use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zac(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1", Min)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1_MAX", Max),
		(2, 0, "R_0_2_0", Min)
	);
}