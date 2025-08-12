use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_rell(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 0, "W_1_0_0_BONUS", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_BONUS", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1_MAX", Max)
	);
}