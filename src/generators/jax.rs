use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jax(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q_0_1_0", Min));
    ability!(w, (0, 0, "W_0_0_0_BONUS", Min));
    ability!(
        e,
        (1, 0, "E_0_1_0_MAX", Max),
        (1, 1, "E_0_1_1_MIN", Min)
    );
    ability!(
        r,
        (0, 4, "R_0_0_4", Min),
        (2, 0, "R_0_2_0_BONUS", Min)
    );
}
