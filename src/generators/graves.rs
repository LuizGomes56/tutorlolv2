use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_graves(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (1, 0, "Q_0_1_0", Min),
        (1, 1, "Q_0_1_1_MAX", Max)
    );
    ability!(w, (0, 0, "W_0_0_0", Min));
    ability!(
        r,
        (0, 0, "R_0_0_0", Min),
        (1, 0, "R_0_1_0", Min)
    );
}
