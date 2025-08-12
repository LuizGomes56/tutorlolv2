use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_lillia(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1_MAX", Max)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0_MAX", Max),
		(0, 1, "W_0_0_1", Min),
		(1, 0, "W_0_1_0_MAX", Max),
		(1, 1, "W_0_1_1", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Min)
	);
}