use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_elise(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1),
        (0, 1, _1Min),
        (0, 0, Monster2),
        (0, 1, _2Min)
    );
    ability!(w, (1, 0, _1Min));
}
