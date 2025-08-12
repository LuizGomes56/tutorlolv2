use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_leona(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min)
	);
	ability!(
		w,
		(0, 2, "W_0_0_2", Min),
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}