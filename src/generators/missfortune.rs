use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_missfortune(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(0, 1, "E_0_0_1_MAX", Max)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_MAX", Max)
	);
}