use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_cassiopeia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Min), (0, 1, "Q_0_0_1_MAX", Max));
    ability!(w, (1, 0, "W_0_1_0", Min), (1, 2, "W_0_1_2_MAX", Max));
    ability!(e, (1, 0, "E_0_1_0_BONUS", Min), (1, 3, "E_0_1_3_MAX", Max));
    ability!(r, (0, 0, "R_0_0_0", Min));
}
