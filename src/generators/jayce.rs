use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jayce(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (0, 0, "Q_1_0_0", Min),
        (1, 0, "Q_1_1_0_MAX", Max)
    );
    ability!(
        w,
        (0, 0, "W_0_0_0", Min),
        (0, 1, "W_0_0_1_MAX", Max),
        (0, 0, "W_1_0_0", Min)
    );
    ability!(
        e,
        (0, 0, "E_0_0_0", Min),
        (0, 1, "E_0_0_1_MNSTR", Min)
    );
}
