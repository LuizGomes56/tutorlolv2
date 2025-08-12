use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_heimerdinger(data: CdnChampion) -> Champion {
    ability!(
        w,
        (0, 0, "W_0_0_0", Min),
        (1, 0, "W_0_1_0_MAX", Max),
        (1, 1, "W_0_1_1_MAX", Max),
        (1, 2, "W_0_1_2", Min),
        (1, 3, "W_0_1_3", Min),
        (1, 4, "W_0_1_4_MAX", Max),
        (1, 5, "W_0_1_5_MAX", Max),
        (0, 0, "W_1_0_0_MAX", Max),
        (0, 1, "W_1_0_1_MAX", Max),
        (0, 2, "W_1_0_2", Min),
        (0, 3, "W_1_0_3", Min),
        (0, 4, "W_1_0_4", Min),
        (2, 0, "W_1_2_0", Min)
    );
    ability!(
        e,
        (0, 0, "E_0_0_0", Min),
        (2, 0, "E_1_2_0", Min)
    );
}
