use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_darius(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (0, 1, "Q_0_0_1", Min)
    );
    ability!(w, (0, 0, "W_0_0_0_BONUS", Min));
    ability!(
        r,
        (0, 0, "R_0_0_0_BONUS", Min),
        (0, 1, "R_0_0_1_MAX", Max),
        (0, 2, "R_0_0_2", Min)
    );
}
