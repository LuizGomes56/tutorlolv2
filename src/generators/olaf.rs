use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_olaf(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (3, 0, Monster1, Min),
        (3, 1, Monster2, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1, Min));
}
