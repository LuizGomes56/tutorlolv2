use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_fiora(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min));
    ability!(w, (1, 0, _1Min, Min));
    ability!(e, (2, 0, _1Min, Min));
}
