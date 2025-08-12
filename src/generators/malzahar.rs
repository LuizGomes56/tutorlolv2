use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_malzahar(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0", Min),
		(3, 1, "W_0_3_1", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1_MAX", Max),
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1_MAX", Max)
	);
}