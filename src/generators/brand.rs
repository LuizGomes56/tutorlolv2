use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_brand(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAX", Max),
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1_MAX", Max)
	);
}