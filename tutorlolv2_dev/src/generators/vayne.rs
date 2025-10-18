use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_vayne(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1));
    ability!(w, (1, 0, _1), (1, 1, _2));
    ability!(e, (0, 0, _1Min), (1, 0, _2), (1, 1, _3Max));
    ability!(r, (0, 0, _1));
}
