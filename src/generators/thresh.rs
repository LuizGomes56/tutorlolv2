use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_thresh(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0_BONUS", Min),
		(1, 1, "E_0_1_1_BONUS", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}