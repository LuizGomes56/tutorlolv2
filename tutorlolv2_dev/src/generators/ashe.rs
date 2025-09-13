use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ashe(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void, Min), (0, 2, Max, Max));
    ability!(w, (0, 1, Void, Min));
    ability!(r, (0, 0, Void, Min));
}
