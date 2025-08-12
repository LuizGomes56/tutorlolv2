use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_pantheon(data: CdnChampion) -> Champion {
	ability!(
		q,
		(4, 0, "Q_0_4_0", Min),
		(4, 1, "Q_0_4_1", Min),
		(4, 2, "Q_0_4_2_MAX", Max),
		(4, 3, "Q_0_4_3_MAX", Max),
		(5, 0, "Q_0_5_0_MAX", Max),
		(5, 1, "Q_0_5_1", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(3, 0, "E_0_3_0", Min)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Min),
		(3, 1, "R_0_3_1", Min)
	);
}