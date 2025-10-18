use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ekko(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 1, _3Max)
    );
    ability!(e, (0, 0, _1));
    ability!(r, (0, 1, _1Min));
}
