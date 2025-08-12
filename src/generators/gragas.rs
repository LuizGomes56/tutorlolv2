use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_gragas(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_MAX", Max),
		(1, 1, "Q_0_1_1_MAX", Max),
		(1, 3, "Q_0_1_3_MIN", Min),
		(1, 4, "Q_0_1_4_MIN", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(1, 0, "W_0_1_0_BONUS", Min),
		(1, 1, "W_0_1_1_MNSTR", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}