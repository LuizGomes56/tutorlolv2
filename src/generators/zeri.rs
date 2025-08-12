use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zeri(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0_BONUS", Min),
		(1, 1, "E_0_1_1", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}