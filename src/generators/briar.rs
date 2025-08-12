use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_briar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Min));
    ability!(
        w,
        (2, 2, "W_0_2_2", Min),
        (0, 0, "W_1_0_0_BONUS", Min),
        (1, 0, "W_1_1_0_BONUS", Min)
    );
    ability!(
        e,
        (2, 0, "E_0_2_0_BONUS", Min),
        (2, 1, "E_0_2_1_MAX", Max),
        (3, 0, "E_0_3_0_MAX", Max),
        (3, 1, "E_0_3_1_MIN", Min)
    );
    ability!(r, (3, 0, "R_0_3_0", Min));
}
