use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nidalee(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MAX", Max),
		(0, 1, "Q_0_0_1_MIN", Min),
		(0, 0, "Q_1_0_0_MAX", Max),
		(0, 1, "Q_1_0_1_MAX", Max),
		(0, 2, "Q_1_0_2_MAX", Max),
		(0, 3, "Q_1_0_3_MIN", Min),
		(1, 0, "Q_1_1_0_MAX", Max),
		(1, 1, "Q_1_1_1_MIN", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 1, "W_0_0_1_MAX", Max),
		(0, 0, "W_1_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_1_0_0", Min)
	);
}