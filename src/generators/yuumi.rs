use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yuumi(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (1, 1, _2Max, Max), (2, 0, _3, Min));
    ability!(
        r,
        (4, 0, _1Min, Min),
        (4, 1, _2Min, Min),
        (4, 2, _3Max, Max)
    );
}
