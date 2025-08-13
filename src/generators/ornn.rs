use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ornn(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (1, 0, Minion1, Min),
        (1, 1, Monster1, Min),
        (1, 2, _1Max, Max),
        (1, 3, Monster2, Min),
        (2, 0, _2Min, Min),
        (2, 1, _3Max, Max)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (3, 0, _2Max, Max));
}
