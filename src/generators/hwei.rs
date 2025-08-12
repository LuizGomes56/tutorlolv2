use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_hwei(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_1_0_0", Min),
        (0, 0, "Q_2_0_0", Min),
        (1, 0, "Q_2_1_0_MAX", Max),
        (1, 1, "Q_2_1_1_MAX", Max),
        (0, 0, "Q_3_0_0", Min),
        (1, 0, "Q_3_1_0", Min),
        (1, 1, "Q_3_1_1_MAX", Max),
        (1, 2, "Q_3_1_2_MAX", Max)
    );
    ability!(
        w,
        (0, 0, "W_3_0_0_BONUS", Min),
        (0, 2, "W_3_0_2_MAX", Max),
        (1, 0, "W_3_1_0_BONUS", Min)
    );
    ability!(
        e,
        (0, 1, "E_1_0_1", Min),
        (0, 0, "E_2_0_0", Min),
        (0, 0, "E_3_0_0", Min)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Min),
        (0, 1, "R_0_0_1_MAX", Max),
        (1, 0, "R_0_1_0", Min),
        (1, 1, "R_0_1_1_MAX", Max)
    );
}
