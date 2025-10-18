use super::*;

// #![preserve] "15.20.1" | "10/14/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zed(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Max, Max), (0, 1, Min, Min));
    ability!(e, (0, 0, Void, Min));
    ability!(r, (2, 0, Void, Min));
}
