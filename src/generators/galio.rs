use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_galio(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Min));
    ability!(
        w,
        (0, 0, "W_0_0_0", Min),
        (0, 1, "W_0_0_1", Min),
        (2, 0, "W_0_2_0_MAX", Max),
        (2, 1, "W_0_2_1_MIN", Min)
    );
    ability!(
        e,
        (1, 0, "E_0_1_0", Min),
        (1, 1, "E_0_1_1", Min)
    );
    ability!(r, (1, 0, "R_0_1_0", Min));
}
