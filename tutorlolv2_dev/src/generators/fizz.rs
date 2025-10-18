use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_fizz(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (0, 0, _1),
        (1, 0, _2),
        (2, 0, _3Min),
        (2, 1, _4Max)
    );
    ability!(e, (1, 0, _1Min), (0, 0, _2Min));
    ability!(
        r,
        (1, 0, _1Min),
        (3, 0, _2Min),
        (4, 0, _3Min)
    );
}
