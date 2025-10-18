use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_olaf(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (3, 0, Monster1),
        (3, 1, Monster2)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1));
}
