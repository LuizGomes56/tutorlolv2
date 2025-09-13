use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_shyvana(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(w, (0, 2, _1Min, Min), (2, 0, _2, Min));
    ability!(e, (0, 0, _1Min, Min), (1, 0, _2Max, Max));
    ability!(r, (0, 0, _1Min, Min));
}
