use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_annie(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(w, (0, 0, Void, Min));
    ability!(e, (1, 0, Void, Min));
    ability!(r, (0, 0, Void, Min));
}
