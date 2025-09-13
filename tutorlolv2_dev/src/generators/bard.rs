use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_bard(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void, Min));
}
