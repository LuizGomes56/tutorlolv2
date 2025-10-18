use super::*;

// #![preserve] "15.18.1" | "09/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_kassadin(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(w, (0, 0, Void, Min));
    ability!(e, (0, 0, Void, Min));
    ability!(
        r,
        (0, 0, _1, Min),
        (1, 0, _2, Min),
        (1, 1, _3, Min),
        (1, 2, _1Max, Max)
    );
}
