use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_mel(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 3, "Q_0_0_3_MAX", Max),
		(0, 4, "Q_0_0_4_MAX", Max)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0", Min),
		(1, 1, "E_0_1_1", Min),
		(1, 2, "E_0_1_2", Min),
		(2, 0, "E_0_2_0", Min),
		(2, 1, "E_0_2_1", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(2, 0, "R_0_2_0_MAX", Max),
		(2, 1, "R_0_2_1", Min)
	);
}