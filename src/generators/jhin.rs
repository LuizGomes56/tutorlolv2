use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jhin(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (1, 0, _2, Min), (1, 1, _3Max, Max));
    ability!(w, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(e, (1, 0, _1Min, Min), (1, 1, _2Min, Min));
    ability!(
        r,
        (1, 0, _1Max, Max),
        (1, 1, Minion1, Min),
        (2, 0, _2Max, Max),
        (2, 1, Minion2, Min)
    );
}
