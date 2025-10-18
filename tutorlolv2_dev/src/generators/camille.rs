use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_camille(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1), (3, 0, _2Max));
    ability!(
        w,
        (0, 0, _1Min),
        (1, 0, _2),
        (2, 0, Monster1),
        (2, 1, Monster2)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (2, 0, _1));
}
