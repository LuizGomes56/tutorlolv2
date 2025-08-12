use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sion(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Max),
		(0, 1, "Q_0_0_1_MAX", Max),
		(0, 2, "Q_0_0_2_MIN", Min),
		(3, 0, "Q_0_3_0_MAX", Max),
		(3, 1, "Q_0_3_1_MNSTR", Min),
		(3, 2, "Q_0_3_2_MIN", Min),
		(3, 3, "Q_0_3_3_MNSTR", Min)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0_MAX", Max),
		(2, 1, "R_0_2_1_MIN", Min)
	);
}