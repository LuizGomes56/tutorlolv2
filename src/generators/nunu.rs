use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nunu(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 1, "Q_0_1_1", Min),
		(2, 2, "Q_0_2_2", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_MAX", Max),
		(2, 1, "W_0_2_1_MIN", Min),
		(4, 0, "W_0_4_0_MAX", Max),
		(4, 1, "W_0_4_1_MIN", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1_MAX", Max),
		(1, 0, "E_0_1_0", Min),
		(3, 0, "E_0_3_0_MAX", Max)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Min)
	);
}