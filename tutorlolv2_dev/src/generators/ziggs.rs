use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ziggs(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
    ability!(w, (1, 0, _1Min, Min));
    ability!(
        e,
        (1, 0, _1Min, Min),
        (1, 1, _2Max, Max),
        (1, 2, _3Min, Min)
    );
    ability!(r, (1, 0, _1Min, Min), (1, 1, _2Min, Min));
}
