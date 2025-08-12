use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_naafiri(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 2, "Q_0_0_2_MAX", Max),
		(0, 3, "Q_0_0_3_MAX", Max),
		(1, 1, "Q_0_1_1_BONUS", Min),
		(1, 2, "Q_0_1_2_MAX", Max),
		(1, 3, "Q_0_1_3_BONUS", Min),
		(1, 4, "Q_0_1_4_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1", Min),
		(0, 2, "E_0_0_2_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1", Min)
	);
}