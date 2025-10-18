use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_talon(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (0, 1, _2Min));
    ability!(
        w,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 2, _3Max)
    );
    ability!(r, (0, 1, _1Min), (1, 0, _2Max));
}
