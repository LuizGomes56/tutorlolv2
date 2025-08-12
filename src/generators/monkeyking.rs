use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_monkeyking(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1_BONUS", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1_MAX", Max),
		(4, 0, "R_0_4_0_MAX", Max)
	);
}