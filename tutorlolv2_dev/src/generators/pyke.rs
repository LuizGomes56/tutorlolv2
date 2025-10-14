use super::*;

// #![preserve] "10/13/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_pyke(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, Void, Min));
    ability!(e, (1, 0, Void, Min));
}
