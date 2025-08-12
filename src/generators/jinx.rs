use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jinx(data: CdnChampion) -> Champion {
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
	ability!(
		r,
		(1, 0, "R_0_1_0_MAX", Max),
		(1, 1, "R_0_1_1_MIN", Min),
		(2, 0, "R_0_2_0_MAX", Max),
		(2, 1, "R_0_2_1_MIN", Min)
	);
}