use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_illaoi(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Min)
	);
	ability!(
		w,
		(3, 0, "W_0_3_0_BONUS", Min),
		(3, 1, "W_0_3_1_MIN", Min)
	);
	ability!(
		e,
		(3, 0, "E_0_3_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}