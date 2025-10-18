use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_velkoz(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 1, _3Max)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (3, 0, _1Min), (3, 1, _2Max));
}
