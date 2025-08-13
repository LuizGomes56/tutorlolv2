use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_velkoz(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Max, Max)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (3, 0, _1Min, Min), (3, 1, _2Max, Max));
}
