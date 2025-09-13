use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_qiyana(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (2, 0, _3Min, Min),
        (2, 1, _4Min, Min),
        (4, 0, _5Max, Max),
        (4, 1, _6Max, Max)
    );
    ability!(w, (3, 1, _1, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (1, 0, Monster1, Min), (1, 1, _1Min, Min));
}
