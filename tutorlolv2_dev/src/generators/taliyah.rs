use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_taliyah(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max),
        (3, 0, _4Min)
    );
    ability!(
        e,
        (0, 0, _1Min),
        (1, 0, _2Max),
        (2, 0, _3Min)
    );
}
