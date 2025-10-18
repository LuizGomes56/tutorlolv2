use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_skarner(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 1, _1),
        (0, 2, _2),
        (3, 0, Monster1),
        (0, 0, Monster2),
        (0, 1, _3Min)
    );
    ability!(w, (0, 0, _1Min));
    ability!(e, (1, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
