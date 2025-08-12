use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_twitch(data: CdnChampion) -> Champion {
	ability!(
		e,
		(1, 0, "E_0_1_0", Min),
		(2, 0, "E_0_2_0_MAX", Max),
		(2, 1, "E_0_2_1_MIN", Min),
		(2, 2, "E_0_2_2", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0_BONUS", Min)
	);
}