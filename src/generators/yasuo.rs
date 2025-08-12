use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yasuo(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(2, 0, "E_0_2_0_BONUS", Min),
		(2, 1, "E_0_2_1_BONUS", Min),
		(2, 2, "E_0_2_2_MAX", Max)
	);
	ability!(
		r,
		(3, 0, "R_0_3_0", Min)
	);
}