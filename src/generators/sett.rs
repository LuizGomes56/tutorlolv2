use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sett(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min),
		(0, 1, "Q_0_0_1_BONUS", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MNSTR", Min),
		(0, 1, "E_0_0_1", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min),
		(1, 1, "R_0_1_1", Min)
	);
}