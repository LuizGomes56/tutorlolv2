use super::*;

// #![preserve] "15.18.1" | "09/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_akali(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(e, (0, 0, Void), (2, 0, _1), (2, 1, _1Max));
    ability!(r, (0, 0, Void), (2, 0, _1Max), (2, 1, _1Min));

    merge_with![(Q::_1, Q::_1Max), (R::_1Min, R::_1Max)]
}
