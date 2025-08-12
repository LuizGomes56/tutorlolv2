use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_briar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (2, 2, _1Min, Min), (0, 0, _2, Min), (1, 0, _3, Min));
    ability!(
        e,
        (2, 0, _1, Min),
        (2, 1, _2Max, Max),
        (3, 0, _3Max, Max),
        (3, 1, Minion1, Min)
    );
    ability!(r, (3, 0, _1Min, Min));
}
