use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_corki(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Min));
    ability!(
        w,
        (1, 0, "W_0_1_0", Min),
        (1, 1, "W_0_1_1_MAX", Max)
    );
    ability!(
        e,
        (0, 0, "E_0_0_0", Min),
        (0, 2, "E_0_0_2_MAX", Max)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Min),
        (2, 0, "R_0_2_0", Min)
    );
}
