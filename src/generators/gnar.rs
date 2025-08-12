use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_gnar(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (0, 1, "Q_0_0_1", Min),
        (0, 0, "Q_1_0_0", Min)
    );
    ability!(
        w,
        (2, 0, "W_0_2_0_BONUS", Min),
        (0, 0, "W_1_0_0", Min)
    );
    ability!(
        e,
        (4, 0, "E_0_4_0", Min),
        (0, 0, "E_1_0_0", Min)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0_MAX", Max),
        (1, 1, "R_0_1_1", Min)
    );
}
