use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ksante(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Min),
		(1, 1, "W_0_1_1_BONUS", Min),
		(1, 2, "W_0_1_2_MAX", Max),
		(4, 0, "W_0_4_0_MNSTR", Min),
		(4, 1, "W_0_4_1", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(3, 0, "R_0_3_0", Min),
		(3, 1, "R_0_3_1_MAX", Max)
	);
}