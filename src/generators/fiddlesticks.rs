use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_fiddlesticks(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Min),
		(0, 2, "Q_0_0_2_MIN", Min),
		(2, 0, "Q_0_2_0_MAX", Max),
		(2, 1, "Q_0_2_1_MAX", Max)
	);
	ability!(
		w,
		(4, 0, "W_0_4_0", Min),
		(4, 1, "W_0_4_1", Min),
		(4, 2, "W_0_4_2", Min),
		(4, 3, "W_0_4_3_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(0, 1, "R_0_0_1_MAX", Max)
	);
}