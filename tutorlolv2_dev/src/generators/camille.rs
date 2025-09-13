use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_camille(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1, Min), (3, 0, _2Max, Max));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (1, 0, _2, Min),
        (2, 0, Monster1, Min),
        (2, 1, Monster2, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (2, 0, _1, Min));
}
