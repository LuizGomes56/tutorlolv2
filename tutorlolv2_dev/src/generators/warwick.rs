use super::*;

// #![preserve] "15.20.1" | "10/14/2025"

#[tutorlolv2_macros::generator]
pub fn gen_warwick(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void, Min), (0, 2, Monster, Min));
    ability!(e, (0, 0, Void, Min));
    ability!(r, (0, 0, Void, Min));
}
