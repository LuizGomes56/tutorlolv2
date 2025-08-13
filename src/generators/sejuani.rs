use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sejuani(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Max, Max)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (2, 0, _2Max, Max));
}
