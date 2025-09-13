use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kled(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1, Min),
        (0, 1, _1Min, Min),
        (2, 0, _2Min, Min),
        (2, 2, _3Max, Max),
        (0, 0, _4Min, Min),
        (2, 0, _5Max, Max),
        (2, 1, _6Min, Min)
    );
    ability!(w, (0, 0, _1, Min), (2, 0, _2, Min));
    ability!(e, (0, 0, _1Min, Min), (2, 0, _2Max, Max));
    ability!(r, (1, 0, _1Max, Max), (1, 1, Minion1, Min));
}
