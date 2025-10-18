use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_thresh(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min));
    ability!(e, (0, 0, _1Min), (1, 0, _2), (1, 1, _3));
    ability!(r, (0, 0, _1Min));
}
