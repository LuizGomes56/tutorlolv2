use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_volibear(data: CdnChampion) -> Champion {
	ability!(
		q,
		(1, 0, "Q_0_1_0_BONUS", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(1, 0, "E_0_1_0", Min)
	);
	ability!(
		r,
		(4, 0, "R_0_4_0", Min)
	);
}