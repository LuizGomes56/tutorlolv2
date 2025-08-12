use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_qiyana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min),
		(2, 0, "Q_1_2_0", Min),
		(2, 1, "Q_1_2_1", Min),
		(4, 0, "Q_1_4_0_MAX", Max),
		(4, 1, "Q_1_4_1_MAX", Max)
	);
	ability!(
		w,
		(3, 1, "W_0_3_1_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MNSTR", Min),
		(1, 1, "R_0_1_1", Min)
	);
}