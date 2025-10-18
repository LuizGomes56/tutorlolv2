use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_tristana(data: CdnChampion) -> Champion {
    ability!(w, (0, 0, _1Min));
    ability!(
        e,
        (0, 0, Minion1),
        (2, 0, _1Min),
        (3, 0, _2),
        (3, 1, _3),
        (3, 2, _4Min)
    );
    ability!(r, (0, 1, _1Min));
}
