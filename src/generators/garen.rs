use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_garen(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_BONUS", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(3, 0, "E_0_3_0_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}