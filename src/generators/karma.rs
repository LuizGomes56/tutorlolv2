use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_karma(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 0, _2, Min),
        (0, 1, _3Max, Max),
        (2, 0, _4Min, Min),
        (2, 1, _5, Min),
        (2, 2, _6Max, Max)
    );
    ability!(w, (0, 0, _1Min, Min), (1, 1, _2Max, Max));
}
