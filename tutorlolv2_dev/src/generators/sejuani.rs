use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sejuani(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 1, _3Max)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min), (2, 0, _2Max));
}
