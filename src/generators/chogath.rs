use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_chogath(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Min));
    ability!(w, (0, 0, "W_0_0_0", Min));
    ability!(e, (0, 0, "E_0_0_0", Min), (0, 2, "E_0_0_2_MAX", Max));
    ability!(r, (0, 0, "R_0_0_0", Min), (0, 1, "R_0_0_1", Min));
}
