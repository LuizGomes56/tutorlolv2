use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_elise(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_MNSTR", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 0, "Q_1_0_0_MNSTR", Min),
		(0, 1, "Q_1_0_1", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min)
	);
}