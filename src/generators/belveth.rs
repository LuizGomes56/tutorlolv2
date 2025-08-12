use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_belveth(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (2, 0, "Q_0_2_0", Min),
        (2, 1, "Q_0_2_1_MNSTR", Min),
        (2, 2, "Q_0_2_2_MNSTR", Min)
    );
    ability!(w, (0, 0, "W_0_0_0", Min));
    ability!(
        e,
        (0, 0, "E_0_0_0", Min),
        (3, 0, "E_0_3_0_MNSTR", Min),
        (3, 1, "E_0_3_1_MNSTR", Min),
        (5, 0, "E_0_5_0_MAX", Max),
        (5, 1, "E_0_5_1_MIN", Min)
    );
    ability!(
        r,
        (1, 0, "R_0_1_0", Min),
        (2, 0, "R_0_2_0_BONUS", Min),
        (2, 1, "R_0_2_1_MNSTR", Min)
    );
}
