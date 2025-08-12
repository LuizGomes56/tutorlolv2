use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_tahmkench(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Min)
	);
	ability!(
		w,
		(2, 1, "W_0_2_1", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min),
		(1, 1, "E_0_1_1_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_1_0_0", Min)
	);
}