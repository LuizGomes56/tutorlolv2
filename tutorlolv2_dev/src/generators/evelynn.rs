use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_evelynn(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, _1Min),
        (2, 0, _2),
        (2, 1, _3),
        (5, 0, _4Min),
        (5, 1, _5Max),
        (5, 2, _6Max)
    );
    ability!(w, (2, 0, _1));
    ability!(e, (0, 0, _1Min), (0, 0, _2Min));
    ability!(r, (0, 0, _1Min), (1, 0, _2Min));
}
