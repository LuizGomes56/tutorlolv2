use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kalista(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(w, (1, 0, Void), (1, 1, Max));
    ability!(e, (1, 0, Void), (1, 1, _1));
}
