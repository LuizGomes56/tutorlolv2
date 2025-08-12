use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_rumble(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 2, "Q_0_0_2_MAX", Max),
		(0, 3, "Q_0_0_3_MAX", Max),
		(1, 0, "Q_0_1_0", Min),
		(1, 1, "Q_0_1_1", Min),
		(1, 2, "Q_0_1_2_MAX", Max),
		(1, 3, "Q_0_1_3_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 4, "E_0_0_4_MAX", Max),
		(1, 0, "E_0_1_0", Min),
		(1, 3, "E_0_1_3_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1_MAX", Max),
		(1, 2, "R_0_1_2_MIN", Min)
	);
}