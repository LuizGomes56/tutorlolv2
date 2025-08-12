use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_fizz(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Min));
    ability!(
        w,
        (0, 0, "W_0_0_0_BONUS", Min),
        (1, 0, "W_0_1_0_BONUS", Min),
        (2, 0, "W_0_2_0", Min),
        (2, 1, "W_0_2_1_MAX", Max)
    );
    ability!(
        e,
        (1, 0, "E_0_1_0", Min),
        (0, 0, "E_1_0_0", Min)
    );
    ability!(
        r,
        (1, 0, "R_0_1_0", Min),
        (3, 0, "R_0_3_0", Min),
        (4, 0, "R_0_4_0", Min)
    );
}
