use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_seraphine(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(1, 0, "Q_0_1_0_MAX", Max)
	);
	ability!(
		e,
		(0, 1, "E_0_0_1", Min),
		(0, 2, "E_0_0_2", Min)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Min)
	);
}