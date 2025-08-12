use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_xerath(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(1, 0, "W_0_1_0_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MAX", Max),
		(2, 0, "R_0_2_0", Min),
		(2, 1, "R_0_2_1_MAX", Max)
	);
}