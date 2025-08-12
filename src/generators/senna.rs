use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_senna(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 1, "Q_0_0_1", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		r,
		(0, 0, "R_0_0_0", Min)
	);
}