use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_draven(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0_BONUS", Min));
    ability!(e, (0, 0, "E_0_0_0", Min));
    ability!(
        r,
        (0, 0, "R_0_0_0", Min),
        (0, 1, "R_0_0_1_MAX", Max),
        (4, 0, "R_0_4_0_MIN", Min),
        (4, 1, "R_0_4_1_MAX", Max)
    );
}
