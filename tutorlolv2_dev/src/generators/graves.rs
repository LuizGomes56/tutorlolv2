use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_graves(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Max, Max)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (1, 0, _2Min, Min));
}
