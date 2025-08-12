use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_amumu(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, None, Min));
    ability!(w, (0, 0, None, Min));
    ability!(e, (0, 0, None, Min), (1, 0, _1Min, Min));
    ability!(r, (0, 0, None, Min));
}
