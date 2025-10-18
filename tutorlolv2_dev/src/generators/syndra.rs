use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_syndra(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (2, 0, _1Min), (3, 0, _2), (3, 1, _3Max));
    ability!(e, (0, 0, _1Min));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (0, 2, Minion1)
    );
}
