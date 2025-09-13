use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_fizz(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (0, 0, _1, Min),
        (1, 0, _2, Min),
        (2, 0, _3Min, Min),
        (2, 1, _4Max, Max)
    );
    ability!(e, (1, 0, _1Min, Min), (0, 0, _2Min, Min));
    ability!(
        r,
        (1, 0, _1Min, Min),
        (3, 0, _2Min, Min),
        (4, 0, _3Min, Min)
    );
}
