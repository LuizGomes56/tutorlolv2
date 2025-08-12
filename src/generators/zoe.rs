use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zoe(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Max),
		(0, 1, "Q_0_0_1_MIN", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min),
		(1, 1, "W_0_1_1_MAX", Max)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min),
		(2, 0, "E_0_2_0_BONUS", Min),
		(2, 1, "E_0_2_1_MAX", Max)
	);
}