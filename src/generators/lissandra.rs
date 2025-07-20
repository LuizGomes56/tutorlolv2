use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_lissandra(data: CdnChampion) -> Champion {
	ability!(
		q,
		(0, 0, "Q_0_0_0", Target::MINIMUM)
	);
	ability!(
		w,
		(0, 0, "W_0_0_0", Target::MINIMUM)
	);
	ability!(
		e,
		(0, 0, "E_0_0_0", Target::MINIMUM)
	);
	ability!(
		r,
		(2, 0, "R_0_2_0", Target::MINIMUM)
	);
}