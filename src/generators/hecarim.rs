use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_hecarim(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0", Min),
        (0, 1, "Q_0_0_1", Min)
    );
    ability!(
        w,
        (0, 0, "W_0_0_0", Min),
        (0, 1, "W_0_0_1_MAX", Max)
    );
    ability!(
        e,
        (3, 0, "E_0_3_0_MAX", Max),
        (3, 1, "E_0_3_1_MIN", Min)
    );
    ability!(r, (0, 0, "R_0_0_0", Min));
}
