use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sett(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min), (0, 1, _2, Min));
    ability!(w, (1, 0, _1Min, Min));
    ability!(e, (0, 0, Monster1, Min), (0, 1, _1Min, Min));
    ability!(r, (1, 0, _1Min, Min), (1, 1, _2Min, Min));
}
