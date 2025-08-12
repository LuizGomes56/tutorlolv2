use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_singed(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Min),
		(2, 1, "Q_0_2_1_MAX", Max)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
}