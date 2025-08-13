use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nasus(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (1, 1, _2Min, Min),
        (1, 2, _3Max, Max)
    );
    ability!(r, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
}
