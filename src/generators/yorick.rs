use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yorick(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0_MNSTR", Min),
		(0, 1, "E_0_0_1", Min),
		(0, 2, "E_0_0_2_MIN", Min)
	);
}