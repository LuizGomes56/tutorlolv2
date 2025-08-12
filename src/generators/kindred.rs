use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kindred(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0", Min),
		(3, 1, "W_0_3_1_MNSTR", Min)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Min),
		(2, 1, "E_0_2_1", Min)
	);
}