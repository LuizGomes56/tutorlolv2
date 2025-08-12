use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kennen(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(2, 0, "W_0_2_0_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1", Min)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0_MAX", Max),
		(3, 0, "R_0_3_0", Min)
	);
}