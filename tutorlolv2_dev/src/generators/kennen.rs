use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kennen(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (0, 0, _1Min), (2, 0, _2));
    ability!(e, (0, 0, _1Min), (0, 1, _2Min));
    ability!(r, (2, 0, _1Max), (3, 0, _2Min));
}
