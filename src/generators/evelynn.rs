use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_evelynn(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, "Q_0_1_0", Min),
        (2, 0, "Q_0_2_0_BONUS", Min),
        (2, 1, "Q_0_2_1_BONUS", Min),
        (5, 0, "Q_0_5_0", Min),
        (5, 1, "Q_0_5_1_MAX", Max),
        (5, 2, "Q_0_5_2_MAX", Max)
    );
    ability!(w, (2, 0, "W_0_2_0_BONUS", Min));
    ability!(
        e,
        (0, 0, "E_0_0_0", Min),
        (0, 0, "E_1_0_0", Min)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Min),
        (1, 0, "R_0_1_0", Min)
    );
}
