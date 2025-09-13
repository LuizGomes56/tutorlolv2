use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_garen(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (3, 0, _2Max, Max));
    ability!(r, (0, 0, _1Min, Min));
}
