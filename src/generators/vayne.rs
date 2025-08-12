use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_vayne(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(w, (1, 0, _1, Min), (1, 1, _2, Min));
    ability!(e, (0, 0, _1Min, Min), (1, 0, _2, Min), (1, 1, _3Max, Max));
    ability!(r, (0, 0, _1, Min));
}
