use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_warwick(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min, Min), (0, 2, Monster1, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Max, Max));
}
