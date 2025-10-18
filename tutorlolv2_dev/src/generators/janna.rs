use super::*;

// #![preserve] "15.20.1" | "10/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_janna(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Min),
        (0, 1, Max),
        /*(0, 2, Minion1)*/
    );
    ability!(w, (0, 0, Void));
    ability!(e, (2, 0, Void));
}
