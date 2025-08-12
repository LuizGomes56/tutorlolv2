use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_illaoi(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
    ability!(w, (3, 0, _1, Min), (3, 1, Minion1, Min));
    ability!(e, (3, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
