use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_yorick(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1));
    ability!(
        e,
        (0, 0, Monster1),
        (0, 1, _1Min),
        (0, 2, Minion1)
    );
}
