use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_skarner(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 1, _1, Min),
        (0, 2, _2, Min),
        (3, 0, Monster1, Min),
        (0, 0, Monster2, Min),
        (0, 1, _3Min, Min)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (1, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
