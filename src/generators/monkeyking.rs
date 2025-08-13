use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_monkeyking(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1, Min));
    ability!(w, (2, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (4, 0, _3Max, Max)
    );
}
