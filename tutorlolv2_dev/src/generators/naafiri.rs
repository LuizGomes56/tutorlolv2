use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_naafiri(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max),
        (0, 3, _4Max),
        (1, 1, _5),
        (1, 2, _6Max),
        (1, 3, _7),
        (1, 4, _8Max)
    );
    ability!(
        e,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max)
    );
    ability!(r, (0, 0, _1Min), (0, 1, _2Min));
}
