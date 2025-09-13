use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_malzahar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (3, 0, _1Min, Min), (3, 1, _2Min, Min));
    ability!(e, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (1, 0, _3Min, Min),
        (1, 1, _4Max, Max)
    );
}
