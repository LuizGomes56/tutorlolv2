use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kaisa(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, _1Min),
        (2, 0, _2Max),
        (3, 0, _3Min),
        (3, 1, _4Max)
    );
    ability!(w, (0, 0, _1Min));
}
