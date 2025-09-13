use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_soraka(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (1, 1, _2Max, Max));
}
