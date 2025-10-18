use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sylas(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 1, _3Min),
        (1, 2, _4Max),
        (1, 3, _5Max)
    );
    ability!(w, (0, 0, _1Min));
    ability!(e, (0, 0, _1Min));
}
