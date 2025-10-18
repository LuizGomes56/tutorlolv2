use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_twitch(data: CdnChampion) -> Champion {
    ability!(
        e,
        (1, 0, _1Min),
        (2, 0, _2Max),
        (2, 1, Minion1),
        (2, 2, _3Min)
    );
    ability!(r, (0, 0, _1));
}
