use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_shyvana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min)
	);
	ability!(
		w,
		(0, 2, "W_0_0_2", Min),
		(2, 0, "W_0_2_0_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}