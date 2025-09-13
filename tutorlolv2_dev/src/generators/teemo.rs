use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_teemo(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max),
        (1, 0, Monster1, Min),
        (1, 1, Monster2, Min),
        (1, 2, Monster3, Min)
    );
    ability!(r, (5, 0, _1Min, Min), (5, 1, _2Max, Max));
}
