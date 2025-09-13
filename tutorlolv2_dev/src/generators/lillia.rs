use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lillia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(
        w,
        (0, 0, _1Max, Max),
        (0, 1, _2Min, Min),
        (1, 0, _3Max, Max),
        (1, 1, _4Min, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (2, 0, _1Min, Min));
}
