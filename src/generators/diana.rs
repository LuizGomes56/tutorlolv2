use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_diana(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min),
		(0, 2, "W_0_0_2_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_BONUS", Min),
		(1, 1, "R_0_1_1", Min),
		(1, 2, "R_0_1_2_MAX", Max)
	);
}