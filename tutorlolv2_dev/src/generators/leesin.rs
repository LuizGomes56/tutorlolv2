use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_leesin(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 0, _2Max),
        (0, 1, Minion1)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min), (1, 0, _2Min));
}
