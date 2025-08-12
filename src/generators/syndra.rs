use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_syndra(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Min),
		(3, 0, "W_0_3_0_BONUS", Min),
		(3, 1, "W_0_3_1_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1_MAX", Max),
		(0, 2, "R_0_0_2_MIN", Min)
	);
}