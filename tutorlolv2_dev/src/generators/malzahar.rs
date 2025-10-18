use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_malzahar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (3, 0, _1Min), (3, 1, _2Min));
    ability!(e, (0, 0, _1Min), (0, 1, _2Max));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (1, 0, _3Min),
        (1, 1, _4Max)
    );
}
