use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_rengar(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0_BONUS", Min)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Min)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Min)
	);
}