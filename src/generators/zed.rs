use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zed(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Min)
	);
}