use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_shen(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_MNSTR", Min),
		(2, 0, "Q_0_2_0_BONUS", Min),
		(2, 1, "Q_0_2_1_MAX", Max),
		(3, 0, "Q_0_3_0_BONUS", Min),
		(3, 1, "Q_0_3_1_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
}