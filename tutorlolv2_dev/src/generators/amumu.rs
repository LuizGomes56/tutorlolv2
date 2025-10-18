use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_amumu(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(w, (0, 0, Void));
    ability!(e, (0, 0, Void), (1, 0, _1Min));
    ability!(r, (0, 0, Void));
}
