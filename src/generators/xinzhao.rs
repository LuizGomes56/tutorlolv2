use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_xinzhao(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min), (0, 1, _2, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
