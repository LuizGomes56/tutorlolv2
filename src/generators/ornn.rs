use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ornn(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_MIN", Min),
		(1, 1, "W_0_1_1_MNSTR", Min),
		(1, 2, "W_0_1_2_MAX", Max),
		(1, 3, "W_0_1_3_MNSTR", Min),
		(2, 0, "W_0_2_0", Min),
		(2, 1, "W_0_2_1_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(3, 0, "R_0_3_0_MAX", Max)
	);
}