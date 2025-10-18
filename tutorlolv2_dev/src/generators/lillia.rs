use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lillia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (0, 1, _2Max));
    ability!(
        w,
        (0, 0, _1Max),
        (0, 1, _2Min),
        (1, 0, _3Max),
        (1, 1, _4Min)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (2, 0, _1Min));
}
