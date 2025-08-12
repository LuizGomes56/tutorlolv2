use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_masteryi(data: CdnChampion) -> Champion {
	ability!(
		q,
		(3, 0, "Q_0_3_0_MNSTR", Min),
		(3, 1, "Q_0_3_1_MNSTR", Min),
		(3, 2, "Q_0_3_2_MAX", Max),
		(3, 3, "Q_0_3_3_MNSTR", Min),
		(3, 4, "Q_0_3_4", Min),
		(3, 5, "Q_0_3_5", Min),
		(3, 6, "Q_0_3_6_MNSTR", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0", Min),
		(2, 1, "W_0_2_1", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Min)
	);
}