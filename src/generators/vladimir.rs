use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_vladimir(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Min),
		(2, 0, "Q_0_2_0_MAX", Max)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min),
		(1, 1, "W_0_1_1_MAX", Max)
	);
	ability!(
		e,
		(4, 0, "E_0_4_0_MAX", Max),
		(4, 1, "E_0_4_1_MIN", Min)
	);
	ability!(
		r,
		(1, 1, "R_0_1_1", Min)
	);
}