use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_aurora(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1, Min),
        (1, 0, _1Max, Max),
        (1, 1, _2, Min),
        (1, 2, _2Max, Max),
        (1, 3, _3, Min)
    );
    ability!(e, (0, 0, Void, Min));
    ability!(r, (0, 0, Void, Min));
}
