use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_shyvana(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1));
    ability!(w, (0, 2, _1Min), (2, 0, _2));
    ability!(e, (0, 0, _1Min), (1, 0, _2Max));
    ability!(r, (0, 0, _1Min));
}
