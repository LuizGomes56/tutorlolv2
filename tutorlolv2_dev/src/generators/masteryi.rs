use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_masteryi(data: CdnChampion) -> Champion {
    ability!(
        q,
        (3, 0, Monster1, Min),
        (3, 1, Monster2, Min),
        (3, 2, _1Max, Max),
        (3, 3, Monster3, Min),
        (3, 4, _2Min, Min),
        (3, 5, _3Min, Min),
        (3, 6, Monster4, Min)
    );
    ability!(w, (2, 0, _1Min, Min), (2, 1, _2Min, Min));
    ability!(e, (0, 0, _1, Min));
}
