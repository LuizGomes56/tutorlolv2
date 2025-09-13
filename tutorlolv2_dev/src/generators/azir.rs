use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_azir(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, Void, Min));
    ability!(w, (3, 0, Void, Min));
    ability!(e, (1, 0, Void, Min));
    ability!(r, (1, 0, Void, Min));
}
