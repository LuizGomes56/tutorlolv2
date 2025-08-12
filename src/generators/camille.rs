use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_camille(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, "Q_0_0_1_BONUS", Min), (3, 0, "Q_0_3_0_MAX", Max));
    ability!(
        w,
        (0, 0, "W_0_0_0", Min),
        (1, 0, "W_0_1_0_BONUS", Min),
        (2, 0, "W_0_2_0_MNSTR", Min),
        (2, 1, "W_0_2_1_MNSTR", Min)
    );
    ability!(e, (0, 0, "E_1_0_0", Min));
    ability!(r, (2, 0, "R_0_2_0_BONUS", Min));
}
