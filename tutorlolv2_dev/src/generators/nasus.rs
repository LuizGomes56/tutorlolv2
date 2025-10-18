use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nasus(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1));
    ability!(
        e,
        (0, 0, _1Min),
        (1, 1, _2Min),
        (1, 2, _3Max)
    );
    ability!(r, (1, 0, _1Min), (1, 1, _2Max));
}
