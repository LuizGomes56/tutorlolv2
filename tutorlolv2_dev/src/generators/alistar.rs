use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_alistar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(w, (0, 0, Void));
    ability!(e, (0, 0, Void), (0, 1, Max));
    ability!(r, (0, 0, Void));
}
