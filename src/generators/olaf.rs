use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_olaf(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(3, 0, "Q_0_3_0_MNSTR", Min),
		(3, 1, "Q_0_3_1_MNSTR", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_BONUS", Min)
	);
}