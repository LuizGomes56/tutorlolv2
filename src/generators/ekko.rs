use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ekko(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Max, Max)
    );
    ability!(e, (0, 0, _1, Min));
    ability!(r, (0, 1, _1Min, Min));
}
