use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_twistedfate(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min),
		(2, 0, "W_0_2_0", Min),
		(5, 0, "W_0_5_0", Min)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1_BONUS", Min)
	);
}