use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sejuani(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(1, 0, "W_0_1_0", Min),
		(1, 1, "W_0_1_1_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(2, 0, "R_0_2_0_MAX", Max)
	);
}