use super::*;

// #![stable] "10/18/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_neeko(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void), (1, 0, Monster), (2, 0, _1), (2, 1, _1Max));
    ability!(w, (1, 0, Void));
    ability!(e, (0, 0, Void));
    ability!(r, (2, 0, Void));
    merge_with![(Q::_1, Q::_1Max)];
}
