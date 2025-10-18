use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_swain(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1), (0, 1, _2Min), (0, 2, _3Max));
    ability!(w, (0, 0, _1Min), (0, 1, _2Min));
    ability!(e, (1, 0, _1Min));
    ability!(r, (2, 1, _1Min), (0, 0, _2Min));
}
