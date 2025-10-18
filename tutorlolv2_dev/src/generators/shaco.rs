use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_shaco(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1));
    ability!(
        w,
        (1, 0, Monster1),
        (1, 1, _1Max),
        (1, 2, Monster2),
        (1, 3, _2Min),
        (1, 4, Monster3)
    );
    ability!(e, (0, 0, _1Max), (0, 1, _2Min));
    ability!(
        r,
        (3, 0, _1Max),
        (3, 1, _2Min),
        (4, 0, _3Min)
    );
}
