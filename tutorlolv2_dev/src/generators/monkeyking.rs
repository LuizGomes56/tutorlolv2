use super::*;

// #![preserve] "10/12/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_monkeyking(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void));
    ability!(w, (2, 0, Void));
    ability!(e, (0, 0, Void));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (4, 0, _3Max)
    );
}
