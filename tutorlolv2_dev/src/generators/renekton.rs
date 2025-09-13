use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_renekton(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 5, _2Min, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (1, 0, _3Max, Max)
    );
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 0, _2Max, Max),
        (2, 1, _3, Min),
        (2, 2, _4, Min),
        (2, 3, _5Max, Max)
    );
    ability!(r, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
}
