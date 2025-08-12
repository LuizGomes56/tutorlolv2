use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_drmundo(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (2, 0, "Q_0_2_0_MNSTR", Min),
        (2, 1, "Q_0_2_1_MIN", Min)
    );
    ability!(
        w,
        (0, 0, "W_0_0_0", Min),
        (0, 1, "W_0_0_1_MAX", Max),
        (2, 0, "W_0_2_0", Min)
    );
    ability!(
        e,
        (0, 0, "E_0_0_0_BONUS", Min),
        (0, 1, "E_0_0_1_BONUS", Min),
        (3, 0, "E_0_3_0_BONUS", Min)
    );
}
