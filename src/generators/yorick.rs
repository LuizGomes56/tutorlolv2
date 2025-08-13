use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yorick(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(
        e,
        (0, 0, Monster1, Min),
        (0, 1, _1Min, Min),
        (0, 2, Minion1, Min)
    );
}
