use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_qiyana(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (2, 0, _3Min),
        (2, 1, _4Min),
        (4, 0, _5Max),
        (4, 1, _6Max)
    );
    ability!(w, (3, 1, _1));
    ability!(e, (0, 0, _1Min));
    ability!(r, (1, 0, Monster1), (1, 1, _1Min));
}
