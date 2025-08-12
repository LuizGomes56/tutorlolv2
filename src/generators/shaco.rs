use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_shaco(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_BONUS", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_MNSTR", Min),
		(1, 1, "W_0_1_1_MAX", Max),
		(1, 2, "W_0_1_2_MNSTR", Min),
		(1, 3, "W_0_1_3", Min),
		(1, 4, "W_0_1_4_MNSTR", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MAX", Max),
		(0, 1, "E_0_0_1", Min)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0_MAX", Max),
		(3, 1, "R_0_3_1", Min),
		(4, 0, "R_0_4_0", Min)
	);
}