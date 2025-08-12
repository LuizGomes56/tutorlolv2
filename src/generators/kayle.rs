use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kayle(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_BONUS", Min),
		(2, 0, "E_0_2_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0", Min)
	);
}