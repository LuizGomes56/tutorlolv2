use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_lissandra(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (2, 0, _1Min, Min));
}
