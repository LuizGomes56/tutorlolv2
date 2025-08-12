use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ziggs(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min),
		(1, 1, "E_0_1_1_MAX", Max),
		(1, 2, "E_0_1_2", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1", Min)
	);
}