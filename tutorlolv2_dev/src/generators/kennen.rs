use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kennen(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min), (2, 0, _2, Min));
    ability!(e, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(r, (2, 0, _1Max, Max), (3, 0, _2Min, Min));
}
