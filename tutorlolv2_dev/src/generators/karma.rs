use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_karma(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 0, _2),
        (0, 1, _3Max),
        (2, 0, _4Min),
        (2, 1, _5),
        (2, 2, _6Max)
    );
    ability!(w, (0, 0, _1Min), (1, 1, _2Max));
}
