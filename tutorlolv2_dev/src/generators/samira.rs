use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_samira(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (2, 0, _1Min), (2, 1, _2Max));
    ability!(e, (0, 1, _1Min));
    ability!(
        r,
        (1, 0, _1Min),
        (1, 1, _2Min),
        (1, 2, _3Max),
        (1, 3, _4Max)
    );
}
