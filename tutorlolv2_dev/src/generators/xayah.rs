use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_xayah(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (1, 0, _3Min),
        (1, 1, _4Max)
    );
    ability!(e, (2, 0, _1Min), (2, 1, _2Min));
    ability!(r, (0, 0, _1Min));
}
