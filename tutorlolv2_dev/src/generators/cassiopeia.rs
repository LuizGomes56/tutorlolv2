use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_cassiopeia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(w, (1, 0, _1Min, Min), (1, 2, _2Max, Max));
    ability!(e, (1, 0, _1, Min), (1, 3, _2Max, Max));
    ability!(r, (0, 0, _1Min, Min));
}
