use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_amumu(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(w, (0, 0, Void, Min));
    ability!(e, (0, 0, Void, Min), (1, 0, _1Min, Min));
    ability!(r, (0, 0, Void, Min));
}
