use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ashe(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void), (0, 2, Max));
    ability!(w, (0, 1, Void));
    ability!(r, (0, 0, Void));
}
