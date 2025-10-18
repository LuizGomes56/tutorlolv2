use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_diana(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (0, 0, _1Min), (0, 2, _2Max));
    ability!(e, (0, 0, _1Min));
    ability!(r, (1, 0, _1), (1, 1, _2Min), (1, 2, _3Max));
}
