use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_xinzhao(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1), (0, 1, _2));
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
