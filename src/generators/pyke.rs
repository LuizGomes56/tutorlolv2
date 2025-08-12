use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_pyke(data: CdnChampion) -> Champion {
	ability!(
		q,
		(2, 0, "Q_0_2_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
}