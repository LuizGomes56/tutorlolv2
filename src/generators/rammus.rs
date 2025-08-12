use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_rammus(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MNSTR", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}