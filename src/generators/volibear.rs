use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_volibear(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (1, 0, _1Min, Min));
    ability!(r, (4, 0, _1Min, Min));
}
