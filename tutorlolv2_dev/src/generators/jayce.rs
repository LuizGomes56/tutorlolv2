use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_jayce(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 0, _2Min),
        (1, 0, _3Max)
    );
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (0, 0, _3Min)
    );
    ability!(e, (0, 0, _1Min), (0, 1, Monster1));
}
