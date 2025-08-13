use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_elise(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1, Min),
        (0, 1, _1Min, Min),
        (0, 0, Monster2, Min),
        (0, 1, _2Min, Min)
    );
    ability!(w, (1, 0, _1Min, Min));
}
