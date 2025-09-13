use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_syndra(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (2, 0, _1Min, Min), (3, 0, _2, Min), (3, 1, _3Max, Max));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (0, 2, Minion1, Min)
    );
}
