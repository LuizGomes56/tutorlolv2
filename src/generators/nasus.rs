use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nasus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 1, "E_0_1_1", Min),
		(1, 2, "E_0_1_2_MAX", Max)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1_MAX", Max)
	);
}