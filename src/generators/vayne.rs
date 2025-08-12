use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_vayne(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Min),
		(1, 1, "W_0_1_1_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0_BONUS", Min),
		(1, 1, "E_0_1_1_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_BONUS", Min)
	);
}