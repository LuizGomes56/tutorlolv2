use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_teemo(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1", Min),
		(0, 2, "E_0_0_2_MAX", Max),
		(1, 0, "E_0_1_0_MNSTR", Min),
		(1, 1, "E_0_1_1_MNSTR", Min),
		(1, 2, "E_0_1_2_MNSTR", Min)
	);
	ability!(
		r,
		(5, 0, "R_0_5_0", Min),
		(5, 1, "R_0_5_1_MAX", Max)
	);
}