use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_talon(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 2, _3Max, Max)
    );
    ability!(r, (0, 1, _1Min, Min), (1, 0, _2Max, Max));
}
