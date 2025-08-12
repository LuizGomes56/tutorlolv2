use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_skarner(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1_BONUS", Min),
		(0, 2, "Q_0_0_2_BONUS", Min),
		(3, 0, "Q_0_3_0_MNSTR", Min),
		(0, 0, "Q_1_0_0_MNSTR", Min),
		(0, 1, "Q_1_0_1", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}