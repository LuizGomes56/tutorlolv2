use super::*;

// #![preserve] "15.19.1" | "10/04/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zyra(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(e, (0, 0, Void, Min));
    ability!(r, (0, 0, Void, Min));
}
