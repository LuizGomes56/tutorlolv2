use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_maokai(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1, Min),
        (0, 1, _1Min, Min),
        (0, 2, Monster2, Min)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (2, 0, _3Min, Min)
    );
    ability!(r, (1, 0, _1Min, Min));
}
