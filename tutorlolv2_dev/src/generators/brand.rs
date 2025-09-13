use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_brand(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
    ability!(w, (0, 0, _1Max, Max), (1, 0, _2Min, Min));
    ability!(e, (1, 0, _1Min, Min));
    ability!(r, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
}
