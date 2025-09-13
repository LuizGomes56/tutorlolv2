use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nocturne(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (1, 0, _2, Min));
    ability!(e, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(r, (2, 0, _1Min, Min));
}
