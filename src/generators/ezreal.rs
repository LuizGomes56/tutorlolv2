use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ezreal(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Min)
	);
	ability!(
		w,
		(1, 0, "W_0_1_0_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min),
		(1, 0, "R_0_1_0", Min)
	);
}