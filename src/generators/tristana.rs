use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_tristana(data: CdnChampion) -> Champion {
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MIN", Min),
		(2, 0, "E_0_2_0", Min),
		(3, 0, "E_0_3_0_BONUS", Min),
		(3, 1, "E_0_3_1_BONUS", Min),
		(3, 2, "E_0_3_2", Min)
	);
	ability!(
		r,
		(0, 1, "R_0_0_1", Min)
	);
}