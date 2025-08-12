use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_taliyah(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min),
		(0, 1, "Q_0_0_1", Min),
		(0, 2, "Q_0_0_2_MAX", Max),
		(3, 0, "Q_0_3_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min),
		(1, 0, "E_0_1_0_MAX", Max),
		(2, 0, "E_0_2_0", Min)
	);
}