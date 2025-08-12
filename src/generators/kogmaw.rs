use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kogmaw(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(0, 1, "W_0_0_1_BONUS", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAX", Max),
		(0, 1, "R_0_0_1_MIN", Min)
	);
}