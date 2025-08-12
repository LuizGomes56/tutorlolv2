use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_poppy(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Min),
		(0, 1, "Q_0_0_1_MNSTR", Min),
		(0, 2, "Q_0_0_2", Min),
		(1, 1, "Q_0_1_1_MNSTR", Min),
		(1, 2, "Q_0_1_2_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 1, "E_0_1_1_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MAX", Max),
		(3, 0, "R_0_3_0", Min)
	);
}