use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sona(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(1, 0, "Q_0_1_0_BONUS", Min)
	);
	ability!(
		w,
		(2, 0, "W_0_2_0_MIN", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}