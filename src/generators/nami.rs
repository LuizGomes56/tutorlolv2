use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nami(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(1, 1, "W_0_1_1", Min),
		(1, 2, "W_0_1_2_MIN", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Min),
		(0, 2, "E_0_0_2_BONUS", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}