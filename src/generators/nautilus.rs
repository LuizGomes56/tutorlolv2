use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nautilus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1_MAX", Max),
		(0, 2, "E_0_0_2", Min),
		(1, 0, "E_0_1_0_MNSTR", Min),
		(1, 1, "E_0_1_1_MNSTR", Min),
		(1, 2, "E_0_1_2_MNSTR", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(1, 0, "R_0_1_0_MAX", Max)
	);
}