use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_leblanc(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (1, 0, _2Max));
    ability!(w, (0, 0, _1Min));
    ability!(
        e,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 1, _3Max)
    );
    ability!(
        r,
        (1, 0, _1Min),
        (2, 0, _2Min),
        (2, 1, _3Min),
        (2, 2, _4Max),
        (3, 0, _5Min),
        (3, 1, _6Min),
        (3, 2, _7Max)
    );
}
