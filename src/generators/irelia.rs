use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_irelia(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, "Q_0_0_1", Min));
    ability!(
        w,
        (3, 0, "W_0_3_0_MAX", Max),
        (3, 1, "W_0_3_1_MIN", Min)
    );
    ability!(e, (2, 0, "E_0_2_0", Min));
    ability!(r, (0, 0, "R_0_0_0", Min));
}
