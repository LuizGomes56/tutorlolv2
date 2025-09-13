use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_leblanc(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (1, 0, _2Max, Max));
    ability!(w, (0, 0, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Max, Max)
    );
    ability!(
        r,
        (1, 0, _1Min, Min),
        (2, 0, _2Min, Min),
        (2, 1, _3Min, Min),
        (2, 2, _4Max, Max),
        (3, 0, _5Min, Min),
        (3, 1, _6Min, Min),
        (3, 2, _7Max, Max)
    );
}
