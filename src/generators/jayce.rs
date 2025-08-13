use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jayce(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 0, _2Min, Min),
        (1, 0, _3Max, Max)
    );
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (0, 0, _3Min, Min)
    );
    ability!(e, (0, 0, _1Min, Min), (0, 1, Monster1, Min));
}
