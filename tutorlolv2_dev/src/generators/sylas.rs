use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sylas(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Min, Min),
        (1, 2, _4Max, Max),
        (1, 3, _5Max, Max)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
}
