use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ivern(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_BONUS", Min),
		(3, 0, "W_0_3_0_BONUS", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
}