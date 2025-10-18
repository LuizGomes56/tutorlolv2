use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_pantheon(data: CdnChampion) -> Champion {
    ability!(
        q,
        (4, 0, _1Min),
        (4, 1, _2Min),
        (4, 2, _3Max),
        (4, 3, _4Max),
        (5, 0, _5Max),
        (5, 1, _6Min)
    );
    ability!(w, (0, 0, _1Min));
    ability!(e, (3, 0, _1Min));
    ability!(r, (3, 0, _1Min), (3, 1, _2Min));
}
