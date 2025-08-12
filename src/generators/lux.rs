use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_lux(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		e,
		(2, 0, "E_0_2_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}