use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ivern(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (2, 0, _1, Min), (3, 0, _2, Min));
    ability!(e, (1, 0, _1Min, Min));
}
