use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_samira(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (2, 0, _1Min, Min), (2, 1, _2Max, Max));
    ability!(e, (0, 1, _1Min, Min));
    ability!(
        r,
        (1, 0, _1Min, Min),
        (1, 1, _2Min, Min),
        (1, 2, _3Max, Max),
        (1, 3, _4Max, Max)
    );
}
